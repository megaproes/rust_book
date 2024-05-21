use std::time::Duration;
use tokio::{select, time::sleep};
async fn sleep_then_string(sleep_time: u64) -> String {
    sleep(Duration::from_millis(sleep_time)).await;
    format!("Slept for {sleep_time} millis!")
}
async fn sleep_then_num(sleep_time: u64) -> u64 {
    sleep(Duration::from_millis(sleep_time)).await;
    sleep_time
}
#[tokio::main]
async fn main() {
  
        let num = select!(
        //biased; the branches are checked in the order they appear, which might be necessary for certain applications where priority or sequence is crucial.
        first = sleep_then_string(10) => first,
        second = sleep_then_string(11) => second,
        third = sleep_then_num(12) => format!("Slept for {third} millis!"),
        _ = sleep(Duration::from_millis(100)) =>
        format!("Timed out after 100 millis!")
        );
        println!("{num}");
    
}
