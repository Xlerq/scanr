use std::net::{SocketAddr, TcpStream};
use std::time::{Duration, Instant};

use crate::models::{Config, ScanSummary};

pub fn scan_range(config: &Config) -> ScanSummary {
    let timer: Instant = Instant::now();
    let mut open_ports: Vec<u16> = Vec::new();

    for i in config.start..=config.end {
        let ip_port: SocketAddr = SocketAddr::new(config.ip, i);
        if scan_port(&ip_port) {
            open_ports.push(i);
        }
    }

    let elapsed: Duration = timer.elapsed();

    ScanSummary {
        open_ports,
        elapsed,
    }
}

fn scan_port(ip_port: &SocketAddr) -> bool {
    let timeout: Duration = Duration::from_millis(200);
    TcpStream::connect_timeout(ip_port, timeout).is_ok()
}
