use crate::args::Args;
use crate::logger::init_logger;
use crate::monitor::Monitor;
use clap::Parser;
use log::{error, LevelFilter};
use std::error::Error;

mod actions;
mod args;
mod logger;
mod monitor;

fn main() {
    let args = Args::parse();

    let level_filter = LevelFilter::from(args.log_level.clone());

    init_logger(level_filter);

    let result = run(args);

    if let Err(e) = result {
        error!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let mut monitor = Monitor::new(args)?;
    monitor.start()?;

    Ok(())
}
