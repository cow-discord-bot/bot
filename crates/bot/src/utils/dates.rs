use chrono::{DateTime, NaiveDate};

pub fn format_timestamp_ddmmyyyy(ts: &str) -> String {
	DateTime::parse_from_rfc3339(ts)
		.map(|dt| dt.naive_utc().date())
		.map(|date: NaiveDate| date.format("%d/%m/%Y").to_string())
		.unwrap_or_else(|_| "Invalid date".into())
}
