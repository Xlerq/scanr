use crate::chunks::create_chunks;
use crate::config::DiscoverConfig;
use crate::engine::TcpResult;
use crate::threadengine::scan_port;

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

pub enum DiscoverEvent {
    HostScanned,
    HostUp,
}

pub fn discover<F>(discover_config: &DiscoverConfig, mut on_event: F) -> Vec<Ipv4Addr>
where
    F: FnMut(DiscoverEvent),
{
    let timeout = discover_config.speed.timeout();

    let mut handles = Vec::new();
    let mut discovered_ips: Vec<Ipv4Addr> = Vec::new();

    let chunks: Vec<Vec<Ipv4Addr>> = create_chunks(&discover_config.ips);
    let (tx, rx) = mpsc::channel();

    for chunk in chunks {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || discover_chunk(chunk, tx_clone, timeout));
        handles.push(handle);
    }

    drop(tx);

    for event in rx {
        on_event(event);
    }

    for handle in handles {
        let mut discovered_chunk = handle.join().unwrap();
        discovered_ips.append(&mut discovered_chunk);
    }

    discovered_ips
}

fn discover_chunk(
    chunk: Vec<Ipv4Addr>,
    tx: Sender<DiscoverEvent>,
    timeout: Duration,
) -> Vec<Ipv4Addr> {
    let mut valid_ips: Vec<Ipv4Addr> = Vec::new();

    for ip in chunk {
        if let Some(ip) = discover_ip(ip, timeout) {
            tx.send(DiscoverEvent::HostUp).unwrap();
            valid_ips.push(ip);
        }
        tx.send(DiscoverEvent::HostScanned).unwrap();
    }
    valid_ips
}

fn discover_ip(ip: Ipv4Addr, timeout: Duration) -> Option<Ipv4Addr> {
    for port in TCP_FALLBACK_PORTS {
        let is_active: bool = tcp_probe(ip, port, timeout);

        if is_active {
            return Some(ip);
        }
    }
    None
}

fn tcp_probe(ip: Ipv4Addr, port: u16, timeout: Duration) -> bool {
    let socket = SocketAddr::from(SocketAddrV4::new(ip, port));
    match scan_port(&socket, timeout) {
        TcpResult::PortOpen | TcpResult::PortClosed => true,
        TcpResult::NoResponse => false,
    }
}

const TCP_FALLBACK_PORTS: [u16; 9] = [80, 443, 22, 445, 53, 3389, 8080, 139, 9100];

#[cfg(test)]
mod tests {
    use super::*;

    fn ip(text: &str) -> Ipv4Addr {
        text.parse().unwrap()
    }

    #[test]
    fn create_chunks_preserves_all_ips() {
        let ips = &[
            ip("192.168.1.1"),
            ip("192.168.1.2"),
            ip("192.168.1.3"),
            ip("192.168.1.4"),
            ip("192.168.1.5"),
        ];

        let chunks = create_chunks(ips);
        let flattened_ips: Vec<Ipv4Addr> = chunks.into_iter().flatten().collect();

        assert_eq!(flattened_ips, ips);
    }

    #[test]
    fn create_chunks_does_not_create_empty_chunks_for_non_empty_input() {
        let ips = vec![ip("192.168.1.1"), ip("192.168.1.2")];

        let chunks = create_chunks(&ips);

        assert!(!chunks.is_empty());
        assert!(chunks.iter().all(|chunk| !chunk.is_empty()));
    }
}
