pub use chrono::{Datelike, DateTime, Local, NaiveDate, Weekday as wd};
pub use error_types::Utc;

pub fn middle_day(year: usize) -> Option<wd> {
    if year % 2 == 0 { return None; }
    let nd = NaiveDate::from_ymd(year as i32, Local::now().month(), 1);
    return Some(nd.weekday());
}