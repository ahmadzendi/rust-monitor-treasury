use bytes::Bytes;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::broadcast;

use crate::config::*;
use crate::state::AppState;
use std::sync::Arc;

pub struct WsManager {
    tx: broadcast::Sender<Bytes>,
    connection_count: AtomicUsize,
}

impl WsManager {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(256);
        Self {
            tx,
            connection_count: AtomicUsize::new(0),
        }
    }

    pub fn subscribe(&self) -> Option<broadcast::Receiver<Bytes>> {
        let count = self.connection_count.fetch_add(1, Ordering::Relaxed);
        if count >= MAX_CONNECTIONS {
            self.connection_count.fetch_sub(1, Ordering::Relaxed);
            return None;
        }
        Some(self.tx.subscribe())
    }

    pub fn unsubscribe(&self) {
        self.connection_count.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn broadcast(&self, data: Bytes) {
        let _ = self.tx.send(data);
    }

    pub fn count(&self) -> usize {
        self.connection_count.load(Ordering::Relaxed)
    }
}

pub async fn heartbeat_loop(state: Arc<AppState>) {
    let ping_msg = Bytes::from_static(b"{\"ping\":true}");
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(HEARTBEAT_INTERVAL_SECS)).await;
        if state.ws_manager.count() > 0 {
            state.ws_manager.broadcast(ping_msg.clone());
        }
    }
}
