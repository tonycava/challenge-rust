pub use error_types::Utc;
pub use middle_day::*;
pub use chrono::{NaiveDate, Weekday as wd};
pub use chrono::TimeZone;


fn main() {
    assert_eq!(wd::Tue, middle_day(2019).unwrap());
    assert_eq!(wd::Wed, middle_day(1997).unwrap());
    assert_eq!(wd::Mon, middle_day(1663).unwrap());
    assert_eq!(wd::Wed, middle_day(1873).unwrap());
    assert_eq!(wd::Thu, middle_day(1953).unwrap());
    assert_eq!(wd::Wed, middle_day(1879).unwrap());

    // println!("{:?}", middle_day(1022).unwrap());
}