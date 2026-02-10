use futures_util::{SinkExt, StreamExt};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::config::*;
use crate::state::{AppState, GoldEntry};

#[derive(serde::Deserialize)]
struct PusherMessage {
    event: Option<String>,
    data: Option<serde_json::Value>,
    #[allow(dead_code)]
    channel: Option<String>,
}

#[derive(serde::Deserialize)]
struct GoldRateData {
    buying_rate: Option<serde_json::Value>,
    selling_rate: Option<serde_json::Value>,
    created_at: Option<String>,
}

fn parse_number(value: &serde_json::Value) -> Option<i64> {
    match value {
        serde_json::Value::Number(n) => n.as_i64().or_else(|| n.as_f64().map(|f| f as i64)),
        serde_json::Value::String(s) => {
            let cleaned = s.replace('.', "").replace(',', "");
            cleaned.parse().ok()
        }
        _ => None,
    }
}

async fn process_treasury_data(state: &Arc<AppState>, data: GoldRateData) {
    let buy = match data.buying_rate.as_ref().and_then(parse_number) {
        Some(v) => v,
        None => return,
    };
    let sell = match data.selling_rate.as_ref().and_then(parse_number) {
        Some(v) => v,
        None => return,
    };
    let created_at = match data.created_at {
        Some(ref s) if !s.is_empty() => s.clone(),
        _ => return,
    };

    {
        let mut shown = state.shown_updates.lock();
        if shown.contains(&created_at) {
            return;
        }
        shown.insert(created_at.clone());
        if shown.len() > 5000 {
            let keep = created_at.clone();
            shown.clear();
            shown.insert(keep);
        }
    }

    let has_last = state.has_last_buy.load(Ordering::Relaxed);
    let last = state.last_buy.load(Ordering::Relaxed);

    let (status, diff) = if !has_last {
        ("➖".to_string(), 0i64)
    } else if buy > last {
        ("🚀".to_string(), buy - last)
    } else if buy < last {
        ("🔻".to_string(), buy - last)
    } else {
        ("➖".to_string(), 0i64)
    };

    let entry = GoldEntry {
        buying_rate: buy,
        selling_rate: sell,
        status,
        diff,
        created_at,
    };

    {
        let mut history = state.history.write();
        if history.len() >= MAX_HISTORY {
            history.pop_front();
        }
        history.push_back(entry);
    }

    state.last_buy.store(buy, Ordering::Relaxed);
    state.has_last_buy.store(true, Ordering::Relaxed);

    state.invalidate_cache();
    let data = state.get_cached_state();
    state.ws_manager.broadcast(data);
}

pub async fn treasury_ws_loop(state: Arc<AppState>) {
    let mut consecutive_errors: u32 = 0;

    loop {
        match connect_async(TREASURY_WS_URL).await {
            Ok((ws_stream, _)) => {
                consecutive_errors = 0;
                tracing::info!("Treasury WebSocket connected");

                let (mut write, mut read) = ws_stream.split();

                let subscribe_msg = serde_json::json!({
                    "event": "pusher:subscribe",
                    "data": {
                        "channel": TREASURY_CHANNEL
                    }
                });

                if let Err(e) = write
                    .send(Message::Text(subscribe_msg.to_string().into()))
                    .await
                {
                    tracing::error!("Failed to subscribe: {}", e);
                    continue;
                }

                tracing::info!("Subscribed to channel: {}", TREASURY_CHANNEL);

                while let Some(msg_result) = read.next().await {
                    match msg_result {
                        Ok(Message::Text(text)) => {
                            let text_str: &str = &text;
                            match serde_json::from_str::<PusherMessage>(text_str) {
                                Ok(pusher_msg) => {
                                    let event = pusher_msg.event.as_deref().unwrap_or("");

                                    match event {
                                        e if e == TREASURY_EVENT => {
                                            if let Some(data_val) = pusher_msg.data {
                                                let gold_data: Option<GoldRateData> =
                                                    match data_val {
                                                        serde_json::Value::String(s) => {
                                                            serde_json::from_str(&s).ok()
                                                        }
                                                        other => {
                                                            serde_json::from_value(other).ok()
                                                        }
                                                    };

                                                if let Some(gd) = gold_data {
                                                    process_treasury_data(&state, gd).await;
                                                }
                                            }
                                        }
                                        "pusher:connection_established" => {
                                            tracing::info!("Pusher connection established");
                                        }
                                        "pusher_internal:subscription_succeeded" => {
                                            tracing::info!("Subscription succeeded");
                                        }
                                        "pusher:error" => {
                                            tracing::warn!(
                                                "Pusher error: {:?}",
                                                pusher_msg.data
                                            );
                                        }
                                        _ => {}
                                    }
                                }
                                Err(e) => {
                                    tracing::debug!("Parse error: {}", e);
                                }
                            }
                        }
                        Ok(Message::Ping(data)) => {
                            let _ = write.send(Message::Pong(data)).await;
                        }
                        Ok(Message::Close(_)) => {
                            tracing::info!("Treasury WebSocket closed by server");
                            break;
                        }
                        Err(e) => {
                            tracing::error!("Treasury WebSocket error: {}", e);
                            break;
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                consecutive_errors += 1;
                tracing::error!(
                    "Treasury WS connect error (attempt {}): {}",
                    consecutive_errors,
                    e
                );
            }
        }

        let wait = std::cmp::min(consecutive_errors as u64, 15);
        tracing::info!("Reconnecting Treasury WS in {}s...", wait);
        tokio::time::sleep(tokio::time::Duration::from_secs(wait)).await;
    }
}
