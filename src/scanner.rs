use std::net::{IpAddr, SocketAddr, TcpStream};
use std::thread;
use std::time::{Duration, Instant};

use crate::models::{Config, ScanSummary};

pub fn scan_range(config: &Config) -> ScanSummary {
    let timer: Instant = Instant::now();
    const THREAD_COUNT: u16 = 256;

    let mut handles = Vec::new();
    let mut open_ports: Vec<u16> = Vec::new();

    let ip: IpAddr = config.ip;
    let start: u16 = config.start;
    let end: u16 = config.end;

    let total_ports = end - start + 1;
    let real_thread_count = total_ports.min(THREAD_COUNT);
    let chunk_len: u16 = total_ports.div_ceil(real_thread_count);

    for i in 0..real_thread_count {
        let chunk_start = start + (chunk_len * i);
        let chunk_end = (chunk_start + chunk_len - 1).min(end);

        if chunk_start > end {
            break;
        }

        let handle = thread::spawn(move || scan_chunk(ip, chunk_start, chunk_end));
        handles.push(handle);
    }

    for handle in handles {
        let mut chunk_ports = handle.join().unwrap();
        open_ports.append(&mut chunk_ports);
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
