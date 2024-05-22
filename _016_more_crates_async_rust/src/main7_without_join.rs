use rand::*;
use std::time::Duration;
use tokio::{join, time::sleep};
async fn wait_and_give_u8(num: u8) -> u8 {
    let mut rng = rand::thread_rng();
    let wait_time = rng.gen_range(1..100);

    sleep(Duration::from_millis(wait_time)).await;

    println!("Got a number! {num}");
    num
}
#[tokio::main]
async fn main() {
    // let num1 = wait_and_give_u8(1).await;
    // let num2 = wait_and_give_u8(2).await;
    // let num3 = wait_and_give_u8(3).await;
    // println!("{num1}, {num2}, {num3}");
    // the above code will do all by once, likely we need below:
    let nums = join!(
        wait_and_give_u8(1),
        wait_and_give_u8(2),
        wait_and_give_u8(3)
    );
    println!("{nums:?}"); // GREAT!

    use tokio::join;

    #[tokio::main]
    async fn main() {
        let future1 = async {
            // This asynchronous block will run concurrently.
            println!("Hello from future 1!");
        };

        let future2 = async {
            // This asynchronous block will also run concurrently.
            println!("Hello from future 2!");
        };

        join!(future1, future2);
        // Both futures will complete before reaching this point.
        println!("Hello from the main future!");
    }
    // very similar to scoped threads:
    thread::scope(|s| {
        s.spawn(|| {
            // This thread will run concurrently.
            println!("Hello from a thread!");
        });
        // Additional threads can be spawned here.
    });
    // All threads spawned in the scope are guaranteed to finish before this point.
    println!("Hello from the main thread!");
}
