use crate::actions::perform_action;
use crate::args::Args;
use log::{debug, error, info};
use procfs::process::Process;
use std::error::Error;
use std::time::{Duration, Instant};

pub struct Monitor {
    pid: i32,
    port: u16,
    timeout: Duration,
    interval: Duration,
    initial_starttime: u64,
    script: String,
}

impl Monitor {
    pub fn new(args: Args) -> Result<Self, Box<dyn Error>> {
        let process =
            Process::new(args.pid).map_err(|e| format!("Failed to get process info: {}", e))?;
        let initial_stat = process
            .stat()
            .map_err(|e| format!("Failed to get process stat: {}", e))?;
        let initial_starttime = initial_stat.starttime;
        let process_name = process.status().expect("Failed to get process status").name;

        info!("Monitoring process '{}' (PID {})", process_name, args.pid);

        Ok(Self {
            pid: args.pid,
            port: args.port,
            timeout: Duration::from_secs(args.timeout),
            interval: Duration::from_secs(args.interval),
            initial_starttime,
            script: args.script,
        })
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let mut timer_start: Option<Instant> = None;

        loop {
            debug!("Starting loop iteration.");

            match self.check_established_connections() {
                Ok(connections) => {
                    debug!("Connections found: {}", !connections.is_empty());
                    match (connections.is_empty(), &mut timer_start) {
                        (false, timer) => {
                            if timer.take().is_some() {
                                info!("Established connection found, resetting timer.");
                            } else {
                                debug!("Established connection found, timer was not running.");
                            }
                        }
                        (true, None) => {
                            info!("No established connections found, starting timer.");
                            timer_start = Some(Instant::now());
                        }
                        (true, Some(start_time)) => {
                            let elapsed = start_time.elapsed();
                            if elapsed >= self.timeout {
                                info!(
                                    "Timer reached timeout of {} seconds.",
                                    self.timeout.as_secs()
                                );
                                perform_action(&self.script);
                                *start_time = Instant::now();
                            } else {
                                debug!(
                                    "No established connections. Timer running for {} seconds.",
                                    elapsed.as_secs()
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Error checking connections: {}", e);
                    if !self.is_process_running()? {
                        error!("Process PID {} no longer exists.", self.pid);
                        break;
                    }
                }
            }

            debug!("Sleeping for {:?} seconds.", self.interval);
            std::thread::sleep(self.interval);
        }

        Ok(())
    }

    fn check_established_connections(&self) -> Result<Vec<procfs::net::TcpNetEntry>, String> {
        debug!("Checking established connections for PID {}.", self.pid);

        let process = match Process::new(self.pid) {
            Ok(proc) => proc,
            Err(e) => {
                let msg = format!("Failed to get process info: {}", e);
                error!("{}", msg);
                return Err(msg);
            }
        };

        let tcp_entries = process
            .tcp()
            .map_err(|e| format!("Failed to get TCP connections: {}", e))?;

        let tcp6_entries = process
            .tcp6()
            .map_err(|e| format!("Failed to get TCP6 connections: {}", e))?;

        let all_tcp_entries = tcp_entries.iter().chain(tcp6_entries.iter());

        let established_connections = all_tcp_entries
            .filter(|entry| entry.state == procfs::net::TcpState::Established)
            .filter(|entry| entry.local_address.port() == self.port)
            .cloned()
            .collect::<Vec<_>>();

        debug!(
            "Found {} established connections on port {}.",
            established_connections.len(),
            self.port
        );

        Ok(established_connections)
    }

    fn is_process_running(&self) -> Result<bool, Box<dyn Error>> {
        match Process::new(self.pid) {
            Ok(new_process) => {
                let new_stat = new_process.stat()?;
                Ok(new_stat.starttime == self.initial_starttime)
            }
            Err(_) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args::Args;
    use std::time::Duration;

    #[test]
    // Well... I'm really bad at writing tests
    fn test_monitor_initialization() {
        let args = Args {
            pid: std::process::id() as i32,
            port: 8080,
            timeout: 60,
            interval: 5,
            script: "./test.sh".to_string(),
            log_level: crate::args::LogLevel::Error,
        };
        let monitor = Monitor::new(args).unwrap();
        assert_eq!(monitor.pid, std::process::id() as i32);
        assert_eq!(monitor.port, 8080);
        assert_eq!(monitor.timeout, Duration::from_secs(60));
        assert_eq!(monitor.interval, Duration::from_secs(5));
    }

    #[test]
    fn test_is_process_running() {
        let args = Args {
            pid: std::process::id() as i32,
            port: 8080,
            timeout: 60,
            interval: 5,
            script: "./test.sh".to_string(),
            log_level: crate::args::LogLevel::Error,
        };
        let monitor = Monitor::new(args).unwrap();
        assert!(monitor.is_process_running().unwrap());
    }

    #[test]
    fn test_check_established_connections() {
        let args = Args {
            pid: std::process::id() as i32,
            port: 8080,
            timeout: 60,
            interval: 5,
            script: "./test.sh".to_string(),
            log_level: crate::args::LogLevel::Error,
        };
        let monitor = Monitor::new(args).unwrap();
        let connections = monitor.check_established_connections();
        assert!(connections.unwrap().is_empty());
    }
}
