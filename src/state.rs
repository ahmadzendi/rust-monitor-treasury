use arc_swap::ArcSwap;
use bytes::Bytes;
use dashmap::DashMap;
use parking_lot::{Mutex, RwLock};
use std::collections::{HashSet, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

use crate::config::*;
use crate::rate_limiter::RateLimiter;
use crate::utils;
use crate::ws_manager::WsManager;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct GoldEntry {
    pub buying_rate: i64,
    pub selling_rate: i64,
    pub status: String,
    pub diff: i64,
    pub created_at: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct UsdIdrEntry {
    pub price: String,
    pub time: String,
}

#[derive(serde::Serialize)]
pub struct HistoryItem {
    pub buying_rate: String,
    pub selling_rate: String,
    pub buying_rate_raw: i64,
    pub selling_rate_raw: i64,
    pub waktu_display: String,
    pub diff_display: String,
    pub transaction_display: String,
    pub created_at: String,
    pub jt10: String,
    pub jt20: String,
    pub jt30: String,
    pub jt40: String,
    pub jt50: String,
}

#[derive(serde::Serialize)]
pub struct UsdIdrItem {
    pub price: String,
    pub time: String,
}

#[derive(serde::Serialize)]
pub struct FullState {
    pub history: Vec<HistoryItem>,
    pub usd_idr_history: Vec<UsdIdrItem>,
    pub limit_bulan: i64,
}

pub struct AppState {
    pub history: RwLock<VecDeque<GoldEntry>>,
    pub usd_idr_history: RwLock<VecDeque<UsdIdrEntry>>,
    pub last_buy: AtomicI64,
    pub has_last_buy: AtomicBool,
    pub shown_updates: Mutex<HashSet<String>>,
    pub limit_bulan: AtomicI64,
    pub ws_manager: WsManager,
    pub rate_limiter: RateLimiter,
    pub blocked_ips: DashMap<String, u64>,
    pub failed_attempts: DashMap<String, Vec<u64>>,
    pub last_successful_call: AtomicU64,
    pub state_cache: ArcSwap<CachedState>,
    pub cache_version: AtomicU64,
}

pub struct CachedState {
    pub data: Bytes,
    pub version: u64,
    pub created_at: Instant,
}

impl AppState {
    pub fn new() -> Self {
        let empty_state = FullState {
            history: vec![],
            usd_idr_history: vec![],
            limit_bulan: 8,
        };
        let data = serde_json::to_vec(&empty_state).unwrap();

        Self {
            history: RwLock::new(VecDeque::with_capacity(MAX_HISTORY)),
            usd_idr_history: RwLock::new(VecDeque::with_capacity(MAX_USD_HISTORY)),
            last_buy: AtomicI64::new(0),
            has_last_buy: AtomicBool::new(false),
            shown_updates: Mutex::new(HashSet::new()),
            limit_bulan: AtomicI64::new(8),
            ws_manager: WsManager::new(),
            rate_limiter: RateLimiter::new(),
            blocked_ips: DashMap::new(),
            failed_attempts: DashMap::new(),
            last_successful_call: AtomicU64::new(0),
            state_cache: ArcSwap::new(Arc::new(CachedState {
                data: Bytes::from(data),
                version: 0,
                created_at: Instant::now(),
            })),
            cache_version: AtomicU64::new(0),
        }
    }

    pub fn invalidate_cache(&self) {
        self.cache_version.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_cached_state(&self) -> Bytes {
        let current = self.state_cache.load();
        let current_version = self.cache_version.load(Ordering::Relaxed);

        if current.version == current_version
            && current.created_at.elapsed().as_millis() < STATE_CACHE_TTL_MS as u128
        {
            return current.data.clone();
        }

        let data = self.build_full_state_bytes();
        let new_cache = Arc::new(CachedState {
            data: data.clone(),
            version: current_version,
            created_at: Instant::now(),
        });
        self.state_cache.store(new_cache);
        data
    }

    fn build_full_state_bytes(&self) -> Bytes {
        let history = self.history.read();
        let usd_history = self.usd_idr_history.read();
        let limit = self.limit_bulan.load(Ordering::Relaxed);

        let hist_items: Vec<HistoryItem> = history.iter().map(|h| self.build_history_item(h)).collect();
        let usd_items: Vec<UsdIdrItem> = usd_history
            .iter()
            .map(|u| UsdIdrItem {
                price: u.price.clone(),
                time: u.time.clone(),
            })
            .collect();

        let state = FullState {
            history: hist_items,
            usd_idr_history: usd_items,
            limit_bulan: limit,
        };

        Bytes::from(serde_json::to_vec(&state).unwrap_or_default())
    }

    fn build_history_item(&self, h: &GoldEntry) -> HistoryItem {
        let buy_fmt = utils::format_rupiah(h.buying_rate);
        let sell_fmt = utils::format_rupiah(h.selling_rate);
        let diff_display = utils::format_diff_display(h.diff, &h.status);
        let waktu_display = utils::format_waktu_only(&h.created_at, &h.status);
        let transaction_display = format!(
            "Beli: {}<br>Jual: {}<br>{}",
            buy_fmt, sell_fmt, diff_display
        );

        HistoryItem {
            buying_rate: buy_fmt,
            selling_rate: sell_fmt,
            buying_rate_raw: h.buying_rate,
            selling_rate_raw: h.selling_rate,
            waktu_display,
            diff_display,
            transaction_display,
            created_at: h.created_at.clone(),
            jt10: utils::calc_profit(h.buying_rate, h.selling_rate, 10_000_000, 9_669_000),
            jt20: utils::calc_profit(h.buying_rate, h.selling_rate, 20_000_000, 19_330_000),
            jt30: utils::calc_profit(h.buying_rate, h.selling_rate, 30_000_000, 28_995_000),
            jt40: utils::calc_profit(h.buying_rate, h.selling_rate, 40_000_000, 38_660_000),
            jt50: utils::calc_profit(h.buying_rate, h.selling_rate, 50_000_000, 48_325_000),
        }
    }

    pub fn is_ip_blocked(&self, ip: &str) -> bool {
        if let Some(entry) = self.blocked_ips.get(ip) {
            let now = utils::current_timestamp();
            if now < *entry {
                return true;
            }
            drop(entry);
            self.blocked_ips.remove(ip);
            self.failed_attempts.remove(ip);
        }
        false
    }

    pub fn block_ip(&self, ip: &str, duration_secs: u64) {
        let until = utils::current_timestamp() + duration_secs;
        self.blocked_ips.insert(ip.to_string(), until);
    }

    pub fn record_failed_attempt(&self, ip: &str, weight: usize) {
        let now = utils::current_timestamp();
        let mut entry = self
            .failed_attempts
            .entry(ip.to_string())
            .or_insert_with(Vec::new);

        for _ in 0..weight {
            entry.push(now);
        }

        entry.retain(|&t| now - t < 60);

        if entry.len() >= MAX_FAILED_ATTEMPTS {
            drop(entry);
            self.block_ip(ip, BLOCK_DURATION_SECS);
        }
    }
}
