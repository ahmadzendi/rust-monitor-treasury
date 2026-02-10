use once_cell::sync::Lazy;

pub const MAX_HISTORY: usize = 1441;
pub const MAX_USD_HISTORY: usize = 11;
pub const USD_POLL_INTERVAL_MS: u64 = 300;
pub const BROADCAST_DEBOUNCE_MS: u64 = 1;
pub const MAX_CONNECTIONS: usize = 500;
pub const STATE_CACHE_TTL_MS: u64 = 20;

pub const MIN_LIMIT: i64 = 0;
pub const MAX_LIMIT: i64 = 88888;
pub const RATE_LIMIT_SECONDS: u64 = 5;
pub const MAX_FAILED_ATTEMPTS: usize = 5;
pub const BLOCK_DURATION_SECS: u64 = 300;

pub const RATE_LIMIT_WINDOW: u64 = 60;
pub const RATE_LIMIT_MAX_REQUESTS: usize = 60;
pub const RATE_LIMIT_STRICT_MAX: usize = 120;

pub const HEARTBEAT_INTERVAL_SECS: u64 = 15;
pub const WS_TIMEOUT_SECS: u64 = 45;

pub static SECRET_KEY: Lazy<String> = Lazy::new(|| {
    std::env::var("ADMIN_SECRET").unwrap_or_else(|_| "indonesia".to_string())
});

pub const TREASURY_WS_URL: &str =
    "wss://ws-ap1.pusher.com/app/52e99bd2c3c42e577e13?protocol=7&client=js&version=7.0.3&flash=false";
pub const TREASURY_CHANNEL: &str = "gold-rate";
pub const TREASURY_EVENT: &str = "gold-rate-event";

pub static SUSPICIOUS_PATHS: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "/admin", "/login", "/wp-admin", "/phpmyadmin", "/.env", "/config",
        "/api/admin", "/administrator", "/wp-login", "/backup", "/.git",
        "/shell", "/cmd", "/exec", "/eval", "/system", "/passwd", "/etc",
    ]
});

pub static RATE_LIMIT_WHITELIST: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec!["/ws", "/api/state"]
});
