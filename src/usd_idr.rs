use std::sync::Arc;
use scraper::{Html, Selector};

use crate::config::*;
use crate::state::{AppState, UsdIdrEntry};
use crate::utils;

async fn fetch_usd_idr_price() -> Option<String> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .timeout(std::time::Duration::from_secs(30))
        .connect_timeout(std::time::Duration::from_secs(10))
        .gzip(true)
        .build()
        .ok()?;

    let resp = client
        .get("https://www.google.com/finance/quote/USD-IDR")
        .header("Accept", "text/html,application/xhtml+xml")
        .header("Cookie", "CONSENT=YES+cb.20231208-04-p0.en+FX+410")
        .send()
        .await
        .ok()?;

    if resp.status() != 200 {
        return None;
    }

    let text = resp.text().await.ok()?;
    let document = Html::parse_document(&text);
    let selector = Selector::parse("div.YMlKec.fxKbKc").ok()?;

    document
        .select(&selector)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string())
}

pub async fn usd_idr_loop(state: Arc<AppState>) {
    // Reuse a single client across iterations
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .timeout(std::time::Duration::from_secs(30))
        .connect_timeout(std::time::Duration::from_secs(10))
        .gzip(true)
        .pool_max_idle_per_host(5)
        .build()
        .unwrap_or_else(|_| reqwest::Client::new());

    loop {
        match fetch_usd_idr_price_with_client(&client).await {
            Some(price) => {
                let should_update = {
                    let hist = state.usd_idr_history.read();
                    hist.is_empty() || hist.back().map(|h| &h.price) != Some(&price)
                };

                if should_update {
                    let time_str = utils::current_wib_time();
                    let entry = UsdIdrEntry {
                        price,
                        time: time_str,
                    };

                    {
                        let mut hist = state.usd_idr_history.write();
                        if hist.len() >= MAX_USD_HISTORY {
                            hist.pop_front();
                        }
                        hist.push_back(entry);
                    }

                    state.invalidate_cache();
                    let data = state.get_cached_state();
                    state.ws_manager.broadcast(data);
                }
            }
            None => {}
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(USD_POLL_INTERVAL_MS)).await;
    }
}

async fn fetch_usd_idr_price_with_client(client: &reqwest::Client) -> Option<String> {
    let resp = client
        .get("https://www.google.com/finance/quote/USD-IDR")
        .header("Accept", "text/html,application/xhtml+xml")
        .header("Cookie", "CONSENT=YES+cb.20231208-04-p0.en+FX+410")
        .send()
        .await
        .ok()?;

    if resp.status() != 200 {
        return None;
    }

    let text = resp.text().await.ok()?;
    let document = Html::parse_document(&text);
    let selector = Selector::parse("div.YMlKec.fxKbKc").ok()?;

    document
        .select(&selector)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string())
}
