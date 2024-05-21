// 1. You need to be inside an async fn or an async block to use the .await keyword.

// 2. Type .await to turn output into a concrete type again. (You don’t need to manually use the poll method.)

// 3. You need a run time to manage the polling, which usually means adding #[tokio::main].

// 4. Regular functions can’t await async functions, so if you have a regular function
// that needs to call an async function, it will become async, too. So once you start
// to use async you’ll see a lot of your other functions becoming async, too.

// 5. async functions can call regular functions. This is usually no problem, but
// remember that regular functions will block the thread until they are done.

use reqwest;
use tokio;

#[tokio::main]
async fn main() {
    // Create an HTTP client
    let client = reqwest::Client::default();

    // Send a GET request to the specified URL
    // The 'await' here waits for the request to be sent and the response headers to be received
    let response = client
        .get("https://www.rust-lang.org")
        .send()
        .await
        .unwrap();

    // Extract the response body as a string
    // The 'await' here waits for the response body to be fully read
    let body = response.text().await.unwrap();

    // Print the response body
    println!("{}", body);
}
