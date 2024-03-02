use chrono::{DateTime, Local, TimeZone};

/// Returns corresponding local datetime from a given timestamp in seconds.
pub fn to_datetime(timestamp: i64) -> DateTime<Local> {
    Local
        .timestamp_opt(timestamp, 0)
        .single()
        .expect("Failed to read timestamp")
}

