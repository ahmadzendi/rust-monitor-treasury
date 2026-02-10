use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn current_wib_time() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // UTC + 7 hours
    let wib_secs = now + 7 * 3600;
    let secs_in_day = wib_secs % 86400;
    let hours = secs_in_day / 3600;
    let minutes = (secs_in_day % 3600) / 60;
    let seconds = secs_in_day % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

pub fn format_rupiah(n: i64) -> String {
    let abs_n = n.unsigned_abs();
    let s = abs_n.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();

    if len <= 3 {
        return if n < 0 {
            format!("-{}", s)
        } else {
            s
        };
    }

    let mut result = String::with_capacity(len + len / 3);
    let first_group = len % 3;

    if first_group > 0 {
        result.push_str(&s[..first_group]);
    }

    for i in (first_group..len).step_by(3) {
        if !result.is_empty() {
            result.push('.');
        }
        result.push_str(&s[i..i + 3]);
    }

    if n < 0 {
        format!("-{}", result)
    } else {
        result
    }
}

pub fn format_diff_display(diff: i64, status: &str) -> String {
    match status {
        "🚀" => format!("🚀+{}", format_rupiah(diff)),
        "🔻" => format!("🔻-{}", format_rupiah(diff.abs())),
        _ => "➖tetap".to_string(),
    }
}

pub fn format_waktu_only(created_at: &str, status: &str) -> String {
    // Parse "YYYY-MM-DD HH:MM:SS" and extract time part
    let time_part = if created_at.len() >= 19 {
        &created_at[11..19]
    } else {
        created_at
    };
    format!("{}{}", time_part, status)
}

pub fn calc_profit(buy_rate: i64, sell_rate: i64, modal: i64, pokok: i64) -> String {
    if buy_rate == 0 {
        return "-".to_string();
    }

    let gram = modal as f64 / buy_rate as f64;
    let val = (gram * sell_rate as f64 - pokok as f64) as i64;
    let gram_str = format!("{:.4}", gram).replace('.', ",");

    // Add thousand separators to gram integer part
    let gram_display = format_gram_display(gram);

    if val > 0 {
        format!("+{}🟢{}gr", format_rupiah(val), gram_display)
    } else if val < 0 {
        format!("-{}🔴{}gr", format_rupiah(val.abs()), gram_display)
    } else {
        format!("{}➖{}gr", format_rupiah(0), gram_display)
    }
}

fn format_gram_display(gram: f64) -> String {
    let formatted = format!("{:.4}", gram);
    // Replace dot with comma for Indonesian format
    formatted.replace('.', ",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_rupiah() {
        assert_eq!(format_rupiah(1000), "1.000");
        assert_eq!(format_rupiah(1000000), "1.000.000");
        assert_eq!(format_rupiah(500), "500");
        assert_eq!(format_rupiah(0), "0");
    }

    #[test]
    fn test_format_diff_display() {
        assert_eq!(format_diff_display(5000, "🚀"), "🚀+5.000");
        assert_eq!(format_diff_display(-3000, "🔻"), "🔻-3.000");
        assert_eq!(format_diff_display(0, "➖"), "➖tetap");
    }
}
