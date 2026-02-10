#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod config;
mod handlers;
mod rate_limiter;
mod security;
mod state;
mod template;
mod treasury;
mod usd_idr;
mod utils;
mod ws_manager;

use axum::{middleware as axum_middleware, Router};
use std::sync::Arc;
use tokio::signal;
use tower_http::compression::CompressionLayer;
use tracing_subscriber::EnvFilter;

use crate::state::AppState;

#[tokio::main]
async fn main() {
    // Set default ke INFO supaya bisa lihat log routing
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .compact()
        .init();

    let state = Arc::new(AppState::new());

    let treasury_state = state.clone();
    tokio::spawn(async move {
        treasury::treasury_ws_loop(treasury_state).await;
    });

    let usd_state = state.clone();
    tokio::spawn(async move {
        usd_idr::usd_idr_loop(usd_state).await;
    });

    let heartbeat_state = state.clone();
    tokio::spawn(async move {
        ws_manager::heartbeat_loop(heartbeat_state).await;
    });

    let app = Router::new()
        .merge(handlers::routes())
        .layer(CompressionLayer::new().gzip(true).br(true).deflate(true))
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            security::security_middleware,
        ))
        .with_state(state.clone());

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .unwrap_or(8000);

    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Server starting on {}", addr);

    // Log semua routes yang terdaftar
    tracing::info!("Routes registered:");
    tracing::info!("  GET /");
    tracing::info!("  GET /health");
    tracing::info!("  GET /api/state");
    tracing::info!("  GET /ws");
    tracing::info!("  GET /aturTS/:value?key=xxx");
    tracing::info!("  * fallback -> catch_all");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutdown signal received");
}
