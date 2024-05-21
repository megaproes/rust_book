// Add dependencies in Cargo.toml
// [dependencies]
// tokio = { version = "1", features = ["full"] }
// reqwest = { version = "0.11", features = ["json"] }

use tokio;
use reqwest;
use std::error::Error;

// Example 1: Basic Async Function
async fn async_give_8() -> u8 {
    8
}

// Example 2: Making an HTTP Request
async fn fetch_url(url: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

// Example 3: Running Multiple Async Tasks Concurrently
async fn async_task_1() {
    println!("Task 1 started");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    println!("Task 1 completed");
}

async fn async_task_2() {
    println!("Task 2 started");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("Task 2 completed");
}

// Example 4: Spawning Tasks
async fn async_task() {
    println!("Task started");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    println!("Task completed");
}

// Main function where we use all the async examples
#[tokio::main]
async fn main() {
    // Example 1: Basic Async Function
    // Call async_give_8 and await its result
    let result = async_give_8().await;
    println!("Result from async_give_8: {}", result);

    // Example 2: Making an HTTP Request
    // Fetch URL and handle potential errors
    match fetch_url("https://www.rust-lang.org").await {
        Ok(body) => println!("Fetched body: {}", body),
        Err(e) => eprintln!("Error fetching URL: {}", e),
    }

    // Example 3: Running Multiple Async Tasks Concurrently
    // Run async_task_1 and async_task_2 concurrently
    tokio::join!(async_task_1(), async_task_2());

    // Example 4: Spawning Tasks
    // Spawn a task to run in the background
    let handle = tokio::spawn(async_task());
    // Await the spawned task's completion
    handle.await.expect("Task panicked");

    // You can see that all tasks are run asynchronously and independently,
    // demonstrating the power and flexibility of async/await in Rust.
}
