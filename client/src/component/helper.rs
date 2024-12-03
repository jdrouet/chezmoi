use std::fmt::Write;

use another_html_builder::AttributeValue;

pub const DATETIME_FMT: &str = "%Y/%m/%d %H:%M";

pub fn format_datetime<'a>(
    timestamp: u64,
) -> Option<chrono::format::DelayedFormat<chrono::format::strftime::StrftimeItems<'a>>> {
    chrono::DateTime::from_timestamp(timestamp as i64, 0).map(|ts| ts.format(DATETIME_FMT))
}

pub struct Classnames<A, B> {
    first: A,
    second: Option<B>,
}

impl<A, B> From<(A, Option<B>)> for Classnames<A, B> {
    fn from((first, second): (A, Option<B>)) -> Self {
        Self { first, second }
    }
}

impl<A: AttributeValue, B: AttributeValue> AttributeValue for Classnames<A, B> {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.first.render(f)?;
        if let Some(ref second) = self.second {
            f.write_char(' ')?;
            second.render(f)
        } else {
            Ok(())
        }
    }
}
