use crate::args::Args;
use crate::logger::init_logger;
use crate::monitor::Monitor;
use clap::Parser;
use log::error;
use std::error::Error;

mod actions;
mod args;
mod logger;
mod monitor;

fn main() {
    init_logger();

    let result = run();

    if let Err(e) = result {
        error!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut monitor = Monitor::new(args)?;
    monitor.start()?;

    Ok(())
}
