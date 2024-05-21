use futures::future::FutureExt;
use futures::task::noop_waker;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration; // for .boxed() method

async fn async_give_8() -> u8 {
    thread::sleep(Duration::from_secs(3));
    8
}

fn main() {
    let mut future: Pin<Box<dyn Future<Output = ()> + Send>> = async {
        let res = async_give_8().await;
        println!("the number is {res}");
    }
    .boxed(); // Box the future to make it pin-compatible

    // Create a Waker and Context
    let waker = noop_waker();
    let mut context = Context::from_waker(&waker);

    // Poll the future until it completes
    loop {
        match Pin::new(&mut future).poll(&mut context) {
            Poll::Ready(_) => break,
            Poll::Pending => {
                println!("Future is not ready yet, sleeping...");
                thread::sleep(Duration::from_millis(100)); // Sleep a bit before polling again
            }
        }
    }
}
