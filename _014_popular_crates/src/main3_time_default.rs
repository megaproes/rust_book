use std::thread::sleep;
use std::time::{Duration, UNIX_EPOCH};
use std::time::{Instant, SystemTime};
fn main() {
    let instant = Instant::now();
    let system_time = SystemTime::now();
    println!("{instant:?}");
    println!("{system_time:?}");

    println!(
        "{:?}",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
    );

    let three_seconds = Duration::from_secs(3);
    println!("I must sleep now.");
    sleep(three_seconds);
    println!("Did I miss anything?");
    
}
