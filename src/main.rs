pub use chrono::{Datelike, DateTime, Local, NaiveDate, Weekday as wd};
pub use chrono::{TimeZone, Utc};
pub use middle_day::middle_day;

fn main() {
    let date = Utc.ymd(2011, 12, 2).and_hms(21, 12, 09);

    assert_eq!(wd::Tue, middle_day(2019).unwrap());
    assert_eq!(wd::Wed, middle_day(1997).unwrap());
    assert_eq!(wd::Mon, middle_day(1663).unwrap());
    assert_eq!(wd::Wed, middle_day(1873).unwrap());
    assert_eq!(wd::Thu, middle_day(1953).unwrap());
    assert_eq!(wd::Wed, middle_day(1879).unwrap());

    // println!("{:?}", middle_day(1022).unwrap());
}





