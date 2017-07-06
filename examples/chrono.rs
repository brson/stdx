extern crate chrono;
use chrono::*;

fn main() {
    let local: DateTime<Local> = Local::now();
    let utc: DateTime<Utc> = Utc::now();

    let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);

    assert_eq!((dt.year(), dt.month(), dt.day()), (2014, 11, 28));
    assert_eq!((dt.hour(), dt.minute(), dt.second()), (12, 0, 9));

    assert_eq!(dt.format("%Y-%m-%d %H:%M:%S").to_string(), "2014-11-28 12:00:09");
    assert_eq!(dt.format("%a %b %e %T %Y").to_string(), "Fri Nov 28 12:00:09 2014");

    assert_eq!(format!("{}", dt), "2014-11-28 12:00:09 UTC");
}

