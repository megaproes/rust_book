// Naive here means that they don’t have any
// time zone info. The easiest way to create them is with the methods that start with
// from_ and end with _opt.

use chrono::naive::{NaiveDate, NaiveTime, Days};

fn main() {
    println!("{:?}", NaiveDate::from_ymd_opt(2023, 3, 25));
    println!("{:?}", NaiveTime::from_hms_opt(12, 5, 30));
    
    
    println!(
        "{:?}",
        NaiveDate::from_ymd_opt(2023, 3, 25)
            .unwrap()
            .and_hms_opt(12, 5, 30)
    );
    let date1 = NaiveDate::from_ymd_opt(2024, 3, 25).unwrap();
    let date2 = date1.checked_add_days(Days::new(10));
    assert_eq!(NaiveDate::from_ymd_opt(2024, 4, 4).unwrap(), date2.unwrap());
    let date3 = date1.and_hms_opt(23, 59, 23);
    println!("\n{date3:?}")
}
