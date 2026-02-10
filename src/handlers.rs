use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, Query, State, WebSocketUpgrade,
    },
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{any, get},
    Router,
};
use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use subtle::ConstantTimeEq;

use crate::config::*;
use crate::state::AppState;
use crate::template::HTML_TEMPLATE;
use crate::utils;

#[derive(serde::Deserialize)]
pub struct LimitQuery {
    key: Option<String>,
}

/// Extract client IP from headers (sama logic dengan security.rs)
fn ip_from_headers(headers: &HeaderMap) -> String {
    if let Some(forwarded) = headers.get("x-forwarded-for") {
        if let Ok(val) = forwarded.to_str() {
            if let Some(first) = val.split(',').next() {
                return first.trim().to_string();
            }
        }
    }
    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(val) = real_ip.to_str() {
            return val.trim().to_string();
        }
    }
    "unknown".to_string()
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .route("/api/state", get(get_state))
        .route("/ws", get(ws_handler))
        .route("/aturTS", get(atur_ts_no_value))
        .route("/aturTS/", get(atur_ts_no_value))
        .route("/aturTS/{value}", get(set_limit))
        .fallback(any(catch_all))
}

async fn index() -> Html<&'static str> {
    Html(HTML_TEMPLATE)
}

async fn health() -> &'static str {
    "ok"
}

async fn get_state(State(state): State<Arc<AppState>>) -> Response {
    let data = state.get_cached_state();
    (
        StatusCode::OK,
        [("content-type", "application/json")],
        data,
    )
        .into_response()
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_ws(socket, state))
}

async fn handle_ws(socket: WebSocket, state: Arc<AppState>) {
    let mut rx = match state.ws_manager.subscribe() {
        Some(rx) => rx,
        None => return,
    };

    let (mut sender, mut receiver) = socket.split();

    let initial = state.get_cached_state();
    if sender
        .send(Message::Binary(initial.to_vec().into()))
        .await
        .is_err()
    {
        state.ws_manager.unsubscribe();
        return;
    }

    let send_task = tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(data) => {
                    if sender
                        .send(Message::Binary(data.to_vec().into()))
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                    tracing::debug!("WebSocket lagged by {} messages", n);
                    continue;
                }
                Err(_) => break,
            }
        }
    });

    let recv_task = tokio::spawn(async move {
        loop {
            match tokio::time::timeout(
                tokio::time::Duration::from_secs(WS_TIMEOUT_SECS),
                receiver.next(),
            )
            .await
            {
                Ok(Some(Ok(Message::Text(_)))) => {}
                Ok(Some(Ok(Message::Binary(_)))) => {}
                Ok(Some(Ok(Message::Close(_)))) => break,
                Ok(Some(Err(_))) => break,
                Ok(None) => break,
                Err(_) => break,
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    state.ws_manager.unsubscribe();
}

async fn atur_ts_no_value(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Response {
    let client_ip = ip_from_headers(&headers);
    if state.is_ip_blocked(&client_ip) {
        return (StatusCode::TOO_MANY_REQUESTS, "IP diblokir sementara").into_response();
    }
    state.record_failed_attempt(&client_ip, 1);
    (StatusCode::BAD_REQUEST, "Parameter tidak lengkap").into_response()
}

async fn set_limit(
    State(state): State<Arc<AppState>>,
    Path(value): Path<String>,
    Query(query): Query<LimitQuery>,
    headers: HeaderMap,
) -> Response {
    let client_ip = ip_from_headers(&headers);

    if state.is_ip_blocked(&client_ip) {
        return (StatusCode::TOO_MANY_REQUESTS, "IP diblokir sementara").into_response();
    }

    // Validasi key
    let key = match query.key {
        Some(k) if !k.is_empty() => k,
        _ => {
            state.record_failed_attempt(&client_ip, 2);
            return (StatusCode::BAD_REQUEST, "Parameter key diperlukan").into_response();
        }
    };

    // Constant-time comparison
    let key_bytes = key.as_bytes();
    let secret_bytes = SECRET_KEY.as_bytes();

    let is_valid = if key_bytes.len() == secret_bytes.len() {
        key_bytes.ct_eq(secret_bytes).unwrap_u8() == 1
    } else {
        false
    };

    if !is_valid {
        state.record_failed_attempt(&client_ip, 1);
        return (StatusCode::FORBIDDEN, "Akses ditolak - key salah").into_response();
    }

    // Parse value
    let int_value: i64 = match value.parse() {
        Ok(v) => v,
        Err(_) => {
            state.record_failed_attempt(&client_ip, 1);
            return (StatusCode::BAD_REQUEST, "Nilai harus berupa angka").into_response();
        }
    };

    // Rate limit
    let now = utils::current_timestamp();
    let last = state.last_successful_call.load(Ordering::Relaxed);
    if now - last < RATE_LIMIT_SECONDS {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            "Terlalu cepat, tunggu beberapa detik",
        )
            .into_response();
    }

    // Range check
    if int_value < MIN_LIMIT || int_value > MAX_LIMIT {
        return (
            StatusCode::BAD_REQUEST,
            format!("Nilai harus {}-{}", MIN_LIMIT, MAX_LIMIT),
        )
            .into_response();
    }

    // Apply changes
    state.limit_bulan.store(int_value, Ordering::Relaxed);
    state.last_successful_call.store(now, Ordering::Relaxed);

    state.invalidate_cache();
    let cached = state.get_cached_state();
    state.ws_manager.broadcast(cached);

    tracing::info!("Limit bulan updated to {} by {}", int_value, client_ip);

    let resp = serde_json::json!({
        "status": "ok",
        "limit_bulan": int_value
    });

    (StatusCode::OK, axum::Json(resp)).into_response()
}

async fn catch_all(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    req: axum::extract::Request,
) -> Response {
    let client_ip = ip_from_headers(&headers);
    let path = req.uri().path().to_lowercase();

    if state.is_ip_blocked(&client_ip) {
        return (StatusCode::TOO_MANY_REQUESTS, "IP diblokir sementara").into_response();
    }

    // Skip aturTS variants - sudah di-handle route dedicated
    if !path.starts_with("/aturt") {
        if path.contains("admin") || path.contains("config") {
            state.record_failed_attempt(&client_ip, 2);
            return (StatusCode::FORBIDDEN, "Akses ditolak").into_response();
        }
    }

    state.record_failed_attempt(&client_ip, 1);
    (StatusCode::NOT_FOUND, "Halaman tidak ditemukan").into_response()
}
