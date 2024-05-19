// Naive here means that they don’t have any
// time zone info. The easiest way to create them is with the methods that start with
// from_ and end with _opt.

use chrono::naive::{NaiveDate, NaiveTime};

fn main() {
    println!("{:?}", NaiveDate::from_ymd_opt(2023, 3, 25));
    println!("{:?}", NaiveTime::from_hms_opt(12, 5, 30));
    println!(
        "{:?}",
        NaiveDate::from_ymd_opt(2023, 3, 25)
            .unwrap()
            .and_hms_opt(12, 5, 30)
    );
}
