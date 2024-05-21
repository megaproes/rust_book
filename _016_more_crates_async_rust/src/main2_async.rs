use std::future::*;
use tokio;
async fn async_give_8() -> u8 {
    8
}

// above is equalent to:
fn async_give_8_long() -> impl Future<Output = u8> {
    async { 8 }
}

#[tokio::main]
async fn main() {
    let some_number = async_give_8().await;
    println!("Got: {}", some_number);
    
    let some_number2: u8;
    while async_give_8().into_future().into_future() {
	   
    }
}

// 											Difference Between Threads and Async

//     1. Threads: Each thread has its own stack and can perform blocking operations independently. 
// 		Threads are managed by the OS, which handles scheduling and context switching.

//     2. Async: Asynchronous functions run on a single thread, leveraging 
// 		the Future trait and the poll method to manage execution. An async runtime (e.g., tokio) handles scheduling and executing futures.