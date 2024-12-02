pub const DATETIME_FMT: &str = "%Y/%m/%d %H:%M";

pub fn format_datetime<'a>(
    timestamp: u64,
) -> Option<chrono::format::DelayedFormat<chrono::format::strftime::StrftimeItems<'a>>> {
    chrono::DateTime::from_timestamp(timestamp as i64, 0).map(|ts| ts.format(DATETIME_FMT))
}
