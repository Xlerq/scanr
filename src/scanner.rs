use std::net::{IpAddr, SocketAddr, TcpStream};
use std::thread;
use std::time::{Duration, Instant};

use crate::models::{Config, ScanSummary};

pub fn scan_range(config: &Config) -> ScanSummary {
    let timer: Instant = Instant::now();
    let mut open_ports: Vec<u16> = Vec::new();

    let ip: IpAddr = config.ip;
    let start: u16 = config.start;
    let end: u16 = config.end;

    let mid: u16 = start + (end - start) / 2;

    let handle1 = thread::spawn(move || scan_chunk(ip, start, mid));
    let handle2 = thread::spawn(move || scan_chunk(ip, mid + 1, end));

    let mut result1: Vec<u16> = handle1.join().unwrap();
    let mut result2: Vec<u16> = handle2.join().unwrap();

    open_ports.append(&mut result1);
    open_ports.append(&mut result2);

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

fn scan_chunk(ip: IpAddr, start: u16, end: u16) -> Vec<u16> {
    let mut open_ports: Vec<u16> = Vec::new();

    for port in start..=end {
        let ip_port: SocketAddr = SocketAddr::new(ip, port);
        if scan_port(&ip_port) {
            open_ports.push(port);
        }
    }
    open_ports
}
