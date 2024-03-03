use chrono::{DateTime, Local, TimeZone};

/// Returns corresponding local datetime from a given timestamp in seconds.
pub fn to_datetime(timestamp: i64) -> DateTime<Local> {
    Local
        .timestamp_opt(timestamp, 0)
        .single()
        .expect("Failed to read timestamp")
}

pub fn duration_hours(duration: u32) -> String {
    format!("{0:.2}", duration as f64 / 60.0 / 60.0)
}