use axum::{
    body::Body,
    extract::{Request, State},
    http::{Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use std::sync::Arc;

use crate::config::*;
use crate::rate_limiter::RateLimitStatus;
use crate::state::AppState;

const HTML_RATE_LIMITED: &str = r#"<!DOCTYPE html>
<html><head><title>429 Too Many Requests</title></head>
<body><h1>Too Many Requests</h1><p>Silakan coba lagi nanti.</p></body></html>"#;

pub fn get_client_ip(req: &Request) -> String {
    if let Some(forwarded) = req.headers().get("x-forwarded-for") {
        if let Ok(val) = forwarded.to_str() {
            if let Some(first) = val.split(',').next() {
                return first.trim().to_string();
            }
        }
    }

    if let Some(real_ip) = req.headers().get("x-real-ip") {
        if let Ok(val) = real_ip.to_str() {
            return val.trim().to_string();
        }
    }

    "unknown".to_string()
}

fn is_suspicious_path(path: &str) -> bool {
    let path_lower = path.to_lowercase();

    // PENTING: Skip pengecekan untuk path /aturTS yang legitimate
    if path_lower.starts_with("/aturt") {
        return false;
    }

    SUSPICIOUS_PATHS
        .iter()
        .any(|&sus| path_lower.contains(sus))
}

fn rate_limited_response() -> Response<Body> {
    Response::builder()
        .status(StatusCode::TOO_MANY_REQUESTS)
        .header("Content-Type", "text/html; charset=utf-8")
        .header("Retry-After", "60")
        .body(Body::from(HTML_RATE_LIMITED))
        .unwrap()
}

fn forbidden_response() -> Response<Body> {
    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"error":"forbidden"}"#))
        .unwrap()
}

pub async fn security_middleware(
    State(state): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> impl IntoResponse {
    let client_ip = get_client_ip(&req);
    let path = req.uri().path().to_string();
    let path_lower = path.to_lowercase();

    // Check blocked IP
    if state.is_ip_blocked(&client_ip) {
        return rate_limited_response();
    }

    // Whitelist legitimate paths - skip rate limit untuk ini
    let is_whitelisted = RATE_LIMIT_WHITELIST.iter().any(|&w| path == w)
        || path_lower.starts_with("/aturt"); // /aturTS dan variannya

    // Rate limit (skip whitelist)
    if !is_whitelisted {
        let (allowed, _count, status) = state.rate_limiter.check(&client_ip);

        match status {
            RateLimitStatus::Blocked => {
                state.block_ip(&client_ip, 600);
                return rate_limited_response();
            }
            RateLimitStatus::Limited => {
                return rate_limited_response();
            }
            RateLimitStatus::Ok => {}
        }
    }

    // Suspicious path check (aturTS sudah di-exclude di is_suspicious_path)
    if is_suspicious_path(&path) {
        state.record_failed_attempt(&client_ip, 3);
        return forbidden_response();
    }

    next.run(req).await.into_response()
}
