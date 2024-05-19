use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
use std::time::SystemTime;
fn main() {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let seconds = now.as_secs();
    println!("{now:?}\tSeconds from 1970 to today: {seconds}");

    let naive_dt: NaiveDateTime = NaiveDateTime::from_timestamp_opt(seconds as i64, 0).unwrap();
    println!("\nAs NaiveDateTime: {naive_dt}");

    let utc_dt: DateTime<Utc> = DateTime::<Utc>::from_utc(naive_dt, Utc);
    println!("\nAs DateTime<Utc>: {utc_dt}");

    let kyiv_offset: FixedOffset = FixedOffset::east_opt(3 * 60 * 60).unwrap();
    let kyiv_dt: DateTime<FixedOffset> = DateTime::from_utc(naive_dt, kyiv_offset);
    println!("\nIn a timezone 3 hours from UTC: {kyiv_dt}");

    let kyiv_naive_dt: NaiveDateTime = kyiv_dt.naive_local();
    println!("\nWith timezone information removed: {kyiv_naive_dt}");

    //
    // without deprecated:
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let seconds = now.as_secs();
    println!("\n\n{now:?}\tSeconds from 1970 to today: {seconds}");
    
    let date_time: DateTime<Utc> = DateTime::from_timestamp(seconds as i64, 0).unwrap();
    println!("\nAs DateTime<Utc>: {naive_dt}");
    
    let utc_dt = DateTime::naive_utc(&date_time);
    println!("\nAs DateTime<Utc>: {utc_dt:?}");
    
    let kyiv_offset: FixedOffset = FixedOffset::east_opt(3 * 60 * 60).unwrap();
    let kyiv_dt: DateTime<FixedOffset> = DateTime::from_naive_utc_and_offset(naive_dt, kyiv_offset);
    println!("\nIn a timezone 3 hours from UTC: {kyiv_dt}");
    
    let kyiv_naive_dt: NaiveDateTime = kyiv_dt.naive_local();
    println!("\nWith timezone information removed: {kyiv_naive_dt}");
}
