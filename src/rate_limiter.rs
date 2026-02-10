use dashmap::DashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::config::*;
use crate::utils;

pub struct RateLimiter {
    requests: DashMap<String, Vec<u64>>,
    last_cleanup: AtomicU64,
}

#[derive(Debug)]
pub enum RateLimitStatus {
    Ok,
    Limited,
    Blocked,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            requests: DashMap::new(),
            last_cleanup: AtomicU64::new(0),
        }
    }

    fn cleanup(&self, now: u64) {
        let last = self.last_cleanup.load(Ordering::Relaxed);
        if now - last < 30 {
            return;
        }

        if self
            .last_cleanup
            .compare_exchange(last, now, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
        {
            return;
        }

        let cutoff = now.saturating_sub(RATE_LIMIT_WINDOW);
        let mut to_remove = Vec::new();

        for mut entry in self.requests.iter_mut() {
            entry.value_mut().retain(|&t| t > cutoff);
            if entry.value().is_empty() {
                to_remove.push(entry.key().clone());
            }
        }

        for key in to_remove {
            self.requests.remove(&key);
        }
    }

    pub fn check(&self, ip: &str) -> (bool, usize, RateLimitStatus) {
        let now = utils::current_timestamp();
        self.cleanup(now);

        let cutoff = now.saturating_sub(RATE_LIMIT_WINDOW);

        let mut entry = self.requests.entry(ip.to_string()).or_insert_with(Vec::new);
        entry.retain(|&t| t > cutoff);

        let count = entry.len();

        if count >= RATE_LIMIT_STRICT_MAX {
            return (false, count, RateLimitStatus::Blocked);
        }

        if count >= RATE_LIMIT_MAX_REQUESTS {
            return (false, count, RateLimitStatus::Limited);
        }

        entry.push(now);
        (true, count + 1, RateLimitStatus::Ok)
    }
}
