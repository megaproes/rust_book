use std::env;
const DEV_URL: &str = "www.somedevurl.com";
const PROD_URL: &str = "www.someprodurl.com";
fn main() {
    match std::env::var("RUST_LOG") {
        Ok(log) => println!("Logging at {log} level"),
        Err(_) => match std::env::var("LOGGER_URL") {
            Ok(url) if url == DEV_URL => {
                println!("Dev url indicated, defaulting to debug");
                env::set_var("RUST_LOG", "DEBUG");
            }
            Ok(url) if url == PROD_URL => {
                println!("Prod url indicated, defaulting to info");
                env::set_var("RUST_LOG", "INFO");
            }
            _ => {
                println!("No valid url indicated, defaulting to info");
                env::set_var("RUST_LOG", "INFO");
            }
        },
    }
}
