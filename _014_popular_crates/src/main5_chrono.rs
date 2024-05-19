use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use std::time::SystemTime;
fn main() {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let seconds = now.as_secs();
    println!("Seconds from 1970 to today: {seconds}");
    
    let naive_dt = NaiveDateTime::from_timestamp_opt(seconds as i64, 0).unwrap();
    println!("As NaiveDateTime: {naive_dt}");
    
    let utc_dt = DateTime::<Utc>::from_utc(naive_dt, Utc);
    println!("As DateTime<Utc>: {utc_dt}");
    
    let kyiv_offset = FixedOffset::east_opt(3 * 60 * 60).unwrap();
    let kyiv_dt: DateTime<FixedOffset> = DateTime::from_utc(naive_dt, kyiv_offset);
    println!("In a timezone 3 hours from UTC: {kyiv_dt}");
    
    let kyiv_naive_dt = kyiv_dt.naive_local();
    println!("With timezone information removed: {kyiv_naive_dt}");
}
