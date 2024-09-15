use log::LevelFilter;
use simple_logger::SimpleLogger;

pub fn init_logger(level: LevelFilter) {
    SimpleLogger::new()
        .with_level(level)
        .init()
        .unwrap();
}
