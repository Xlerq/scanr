use std::cmp::min;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::sync::mpsc::{self, Sender};
use std::thread::{self, available_parallelism};
use std::time::Duration;

use crate::models::{Config, ScanEvent};

pub fn scan_ports<F>(config: &Config, mut on_event: F) -> Vec<u16>
where
    F: FnMut(ScanEvent),
{
    let mut handles = Vec::new();
    let mut open_ports: Vec<u16> = Vec::new();

    let ip: IpAddr = config.ip;
    let ports: Vec<u16> = config.ports.to_owned();
    let timeout: Duration = config.speed.timeout();

    let chunks: Vec<Vec<u16>> = create_chunks(ports);
    let (tx, rx) = mpsc::channel();

    for chunk in chunks {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || scan_chunk(ip, chunk, tx_clone, timeout));
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

fn create_chunks(ports: Vec<u16>) -> Vec<Vec<u16>> {
    let total_ports: usize = ports.len();
    let real_thread_count = choose_thread_count(total_ports);
    let chunk_len: usize = total_ports.div_ceil(real_thread_count);

    ports
        .chunks(chunk_len)
        .map(|chunk| chunk.to_vec())
        .collect()
}

fn choose_thread_count(total_ports: usize) -> usize {
    let cpu_count: usize = available_parallelism().map(|n| n.get()).unwrap_or(4);
    min(total_ports, cpu_count * 32)
}

fn scan_port(ip_port: &SocketAddr, timeout: Duration) -> bool {
    TcpStream::connect_timeout(ip_port, timeout).is_ok()
}

fn scan_chunk(ip: IpAddr, chunk: Vec<u16>, tx: Sender<ScanEvent>, timeout: Duration) -> Vec<u16> {
    let mut open_ports: Vec<u16> = Vec::new();

    for port in chunk {
        let ip_port: SocketAddr = SocketAddr::new(ip, port);
        if scan_port(&ip_port, timeout) {
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
        let timeout: Duration = Duration::from_millis(100);
        assert!(scan_port(&ip_port, timeout));
    }

    #[test]
    fn number_of_thread() {
        let total_ports: usize = 3;
        let thread_count: usize = choose_thread_count(total_ports);

        assert_eq!(total_ports, thread_count);
    }

    #[test]
    fn number_of_thread2() {
        let total_ports: usize = 60000;
        let thread_count: usize = choose_thread_count(total_ports);

        assert!(thread_count < total_ports);
    }

    #[test]
    fn splits_port_list_into_valid_chunks() {
        let chunks: Vec<Vec<u16>> = create_chunks(vec![1, 2, 3, 4, 5]);

        assert_eq!(chunks, vec![vec![1], vec![2], vec![3], vec![4], vec![5]]);
    }

    #[test]
    fn creates_chunks_that_cover_all_ports() {
        let ports: Vec<u16> = (100..=10000).collect();
        let chunks: Vec<Vec<u16>> = create_chunks(ports.clone());
        assert!(!chunks.is_empty());

        let flattened_ports: Vec<u16> = chunks.into_iter().flatten().collect();

        assert_eq!(flattened_ports, ports);
    }
}
