use reqwest::Client;

// Both functions return a u8 but in different ways. The fn function returns one right
// away, but the async fn returns something that will be a u8 when it’s done.
// Maybe it’ll be done right away, or maybe it won’t.
// And because it’s async, if it’s not done yet, your code can do other work as it waits.
fn give_8() -> u8 {
    8
}
async fn async_give_8() -> u8 {
    8
}

async fn main() {
    // let client = reqwest::blocking::Client::default();
    // let response = client.get("https://www.rust-lang.org").send().unwrap();
    // println!("{}", response.text().unwrap());
    
    let y = async_give_8();
    // println!("{}",  y.await.poll());
}
