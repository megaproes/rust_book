use flexi_logger::{FileSpec, Logger, WriteMode};
use log::{info, warn};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _logger = Logger::try_with_str("info")?
        .log_to_file(FileSpec::default())
        .write_mode(WriteMode::BufferAndFlush)
        .start()?;

    info!("Connected to port ");
	warn!("asdasssss");
    Ok(())
}
