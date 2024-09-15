use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
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
}
