use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    // Create a thread pool with 4 threads
    let pool = ThreadPool::new(4);

    // Create a channel to receive results
    let (sender, receiver) = channel();

    for i in 0..8 {
        let sender = sender.clone();
        pool.execute(move || {
            // Simulate some work
            std::thread::sleep(Duration::from_secs(1));
            sender.send(i).unwrap();
        });
    }

    // Drop the original sender so the receiver can close properly
    drop(sender);

    for result in receiver.iter() {
        println!("Received: {}", result);
    }

    println!("All tasks completed.");
}
