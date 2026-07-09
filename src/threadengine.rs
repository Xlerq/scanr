use std::io::ErrorKind;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

use crate::chunks::create_chunks;
use crate::engine::{ScanEngine, ScanEvent, TcpResult};

pub struct ThreadEngine;

impl ScanEngine for ThreadEngine {
    fn scan(
        &self,
        ip: IpAddr,
        ports: &[u16],
        timeout: Duration,
        on_event: &mut dyn FnMut(ScanEvent),
    ) -> Vec<(u16, TcpResult)> {
        let mut handles = Vec::new();
        let mut result: Vec<(u16, TcpResult)> = Vec::with_capacity(ports.len());

        let ports: Vec<u16> = ports.to_owned();

        let chunks: Vec<Vec<u16>> = create_chunks(&ports);
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
            result.append(&mut chunk_ports);
        }
        result
    }
}

pub fn scan_port(ip_port: &SocketAddr, timeout: Duration) -> TcpResult {
    match TcpStream::connect_timeout(ip_port, timeout) {
        Ok(_) => TcpResult::PortOpen,
        Err(e) => {
            if e.kind() == ErrorKind::TimedOut {
                TcpResult::NoResponse
            } else {
                TcpResult::PortClosed
            }
        }
    }
}

fn scan_chunk(
    ip: IpAddr,
    chunk: Vec<u16>,
    tx: Sender<ScanEvent>,
    timeout: Duration,
) -> Vec<(u16, TcpResult)> {
    let mut results: Vec<(u16, TcpResult)> = Vec::with_capacity(chunk.len());

    for port in chunk {
        let ip_port: SocketAddr = SocketAddr::new(ip, port);
        let verdict = scan_port(&ip_port, timeout);
        if matches!(verdict, TcpResult::PortOpen) {
            tx.send(ScanEvent::PortOpen).unwrap();
        }
        tx.send(ScanEvent::PortScanned).unwrap();
        results.push((port, verdict))
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;

    #[test]
    fn returns_port_open_when_port_is_listening() {
        let listener: TcpListener = TcpListener::bind("127.0.0.1:0").unwrap();
        let ip_port: SocketAddr = listener.local_addr().unwrap();
        let timeout: Duration = Duration::from_millis(100);
        assert!(matches!(scan_port(&ip_port, timeout), TcpResult::PortOpen));
    }

    #[test]
    fn returns_port_closed_when_host_refuses() {
        // Bind to grab a free port, then drop the listener so nothing is
        // listening: connecting now yields an immediate RST (ConnectionRefused),
        // which is proof the host is alive. This is the case the old `bool`
        // collapsed into "down".
        let listener: TcpListener = TcpListener::bind("127.0.0.1:0").unwrap();
        let ip_port: SocketAddr = listener.local_addr().unwrap();
        drop(listener);
        let timeout: Duration = Duration::from_millis(100);
        assert!(matches!(
            scan_port(&ip_port, timeout),
            TcpResult::PortClosed
        ));
    }
}
