use chrono::{DateTime, FixedOffset, Utc};
use std::str::FromStr;
const SECONDS_IN_HOUR: i32 = 3600;
const UTC_TO_KST_HOURS: i32 = 9;
const UTC_TO_KST_SECONDS: i32 = UTC_TO_KST_HOURS * SECONDS_IN_HOUR;
#[derive(Debug)]
struct UtcUserEvent {
    timestamp: &'static str,
    data: String,
}
#[derive(Debug)]
struct KoreaJapanUserEvent {
    timestamp: DateTime<FixedOffset>,
    data: String,
}
impl From<UtcUserEvent> for KoreaJapanUserEvent {
    fn from(event: UtcUserEvent) -> Self {
        let utc_datetime: DateTime<Utc> = DateTime::from_str(event.timestamp).unwrap();
        let offset = FixedOffset::east_opt(UTC_TO_KST_SECONDS).unwrap();
        let timestamp: DateTime<FixedOffset> = DateTime::from_utc(utc_datetime.naive_utc(), offset);
        Self {
            timestamp,
            data: event.data,
        }
    }
}
fn main() {
    let incoming_event = UtcUserEvent {
        timestamp: "2023-03-27 23:48:50 UTC",
        data: "Something happened in UTC time".to_string(),
    };
    println!("Event as Utc:\n{incoming_event:?}");
    let korea_japan_event = KoreaJapanUserEvent::from(incoming_event);
    println!("Event in Korea/Japan time:\n{korea_japan_event:?}");
}
#[test]
fn utc_to_korea_output_same_evening() {
    let morning_event = UtcUserEvent {
        timestamp: "2023-03-27 09:48:50 UTC",
        data: String::new(),
    };
    let to_korea_japan = KoreaJapanUserEvent::from(morning_event);
    assert_eq!(
        &to_korea_japan.timestamp.to_string(),
        "2023-03-27 18:48:50 +09:00"
    );
}
#[test]
fn utc_to_korea_output_next_morning() {
    let evening_event = UtcUserEvent {
        timestamp: "2023-03-27 23:59:59 UTC",
        data: String::new(),
    };
    let korea_japan_next_morning = KoreaJapanUserEvent::from(evening_event);
    assert_eq!(
        &korea_japan_next_morning.timestamp.to_string(),
        "2023-03-28 08:59:59 +09:00"
    );
}
