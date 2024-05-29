use std::{backtrace::Backtrace, env};
fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    println!("{}", Backtrace::capture());
}
