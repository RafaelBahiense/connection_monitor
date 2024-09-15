use clap::{Parser, ValueEnum};
use log::LevelFilter;

#[derive(Parser, Debug)]
#[command(version, about = "Monitor TCP connections of a process")]
pub struct Args {
    #[arg(long)]
    pub pid: i32,
    #[arg(long)]
    pub port: u16,
    #[arg(long, default_value_t = 60)]
    pub timeout: u64,
    #[arg(long, default_value_t = 5)]
    pub interval: u64,
    #[arg(long)]
    pub script: String,
    #[arg(long, default_value = "info", help = "Set the log level")]
    pub log_level: LogLevel,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for LevelFilter {
    fn from(log_level: LogLevel) -> Self {
        match log_level {
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
        }
    }
}

