use log::{info, warn};
use env_logger;

fn main() {
    env_logger::init();

    info!("This is an info message.");
    warn!("This is a warning message.");
}
