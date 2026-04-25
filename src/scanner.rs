use std::cmp::min;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::sync::mpsc::{self, Sender};
use std::thread::{self, available_parallelism};
use std::time::Duration;

use crate::models::{Config, ScanEvent};

pub fn scan_range<F>(config: &Config, mut on_event: F) -> Vec<u16>
where
    F: FnMut(ScanEvent),
{
    let mut handles = Vec::new();
    let mut open_ports: Vec<u16> = Vec::new();

    let ip: IpAddr = config.ip;
    let start: u16 = config.start;
    let end: u16 = config.end;

    let chunks: Vec<(u16, u16)> = create_chunks(start, end);
    let (tx, rx) = mpsc::channel();

    for (chunk_start, chunk_end) in chunks {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || scan_chunk(ip, chunk_start, chunk_end, tx_clone));
        handles.push(handle);
    }

    drop(tx);

    for event in rx {
        on_event(event);
    }

    for handle in handles {
        let mut chunk_ports = handle.join().unwrap();
        open_ports.append(&mut chunk_ports);
    }

    open_ports
}

fn create_chunks(start: u16, end: u16) -> Vec<(u16, u16)> {
    let mut chunks: Vec<(u16, u16)> = Vec::new();

    let total_ports = end - start + 1;
    let real_thread_count = choose_thread_count(total_ports);
    let chunk_len: u16 = total_ports.div_ceil(real_thread_count);

    for i in 0..real_thread_count {
        let chunk_start = start + (chunk_len * i);
        let chunk_end = (chunk_start + chunk_len - 1).min(end);

        if chunk_start > end {
            break;
        }

        chunks.push((chunk_start, chunk_end));
    }
    chunks
}

fn choose_thread_count(total_ports: u16) -> u16 {
    let cpu_count: usize = available_parallelism().map(|n| n.get()).unwrap_or(4);
    min(total_ports, cpu_count as u16 * 32)
}

fn scan_port(ip_port: &SocketAddr) -> bool {
    let timeout: Duration = Duration::from_millis(300);
    TcpStream::connect_timeout(ip_port, timeout).is_ok()
}

fn scan_chunk(ip: IpAddr, start: u16, end: u16, tx: Sender<ScanEvent>) -> Vec<u16> {
    let mut open_ports: Vec<u16> = Vec::new();

    for port in start..=end {
        let ip_port: SocketAddr = SocketAddr::new(ip, port);
        if scan_port(&ip_port) {
            open_ports.push(port);
            tx.send(ScanEvent::PortOpen).unwrap();
        }
        tx.send(ScanEvent::PortScanned).unwrap();
    }
    open_ports
}

#[cfg(test)]
mod tests {
    use std::net::TcpListener;

    use super::*;

    #[test]
    fn returns_true_when_port_is_open() {
        let listener: TcpListener = TcpListener::bind("127.0.0.1:0").unwrap();
        let ip_port: SocketAddr = listener.local_addr().unwrap();
        assert!(scan_port(&ip_port));
    }

    #[test]
    fn number_of_thread() {
        let total_ports: u16 = 3;
        let thread_count: u16 = choose_thread_count(total_ports);

        assert_eq!(total_ports, thread_count);
    }

    #[test]
    fn number_of_thread2() {
        let total_ports: u16 = 60000;
        let thread_count: u16 = choose_thread_count(total_ports);

        assert!(thread_count < total_ports);
    }

    #[test]
    fn splits_range_into_valid_chunks() {
        let chunks: Vec<(u16, u16)> = create_chunks(1, 5);

        assert_eq!(chunks, [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)]);
    }

    #[test]
    fn creates_chunks_that_cover_entire_range() {
        let chunks: Vec<(u16, u16)> = create_chunks(100, 10000);
        assert!(!chunks.is_empty());

        let first_chunk: (u16, u16) = chunks[0];
        let last_chunk: (u16, u16) = chunks[chunks.len() - 1];

        let (first_start, _) = first_chunk;
        let (_, last_end) = last_chunk;

        assert_eq!(first_start, 100);
        assert_eq!(last_end, 10000);
    }
}
