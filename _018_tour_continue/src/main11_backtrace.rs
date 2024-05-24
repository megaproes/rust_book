use std::{
    backtrace::{Backtrace, BacktraceStatus::*},
    panic,
};
fn main() {
    panic::set_hook(Box::new(|_| {
        println!("Panicked! Trying to get a backtrace...");
        let backtrace = Backtrace::capture();
        match backtrace.status() {
            Disabled => println!("Backtrace isn't enabled, sorry"),
            Captured => println!("Here's the backtrace!!\n{backtrace}"),
            Unsupported => println!("No backtrace possible, sorry"),
            _ => todo!(),
		  // Do some database shutting down stuff
        }
    }));
    std::env::set_var("RUST_BACKTRACE", "0");
    panic!();
}
