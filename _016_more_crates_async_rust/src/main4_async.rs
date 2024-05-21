use std::{thread, time::Duration};
use tokio;

async fn async_give_8() -> u8 {
    thread::sleep(Duration::from_secs(3));
    8
}
#[tokio::main]
async fn main() {
    let some_number = async_give_8();

    println!("hello world - {} ", some_number.await);
}
