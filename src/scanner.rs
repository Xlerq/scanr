use std::net::{IpAddr, SocketAddr, TcpStream};
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

use crate::chunks::create_chunks;
use crate::models::{ScanConfig, ScanEvent};

pub fn scan_ports<F>(config: &ScanConfig, mut on_event: F) -> Vec<u16>
where
    F: FnMut(ScanEvent),
{
    let mut handles = Vec::new();
    let mut open_ports: Vec<u16> = Vec::new();

    let ip: IpAddr = config.ip;
    let ports: Vec<u16> = config.ports.to_owned();
    let timeout: Duration = config.speed.timeout();

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
        open_ports.append(&mut chunk_ports);
    }

    open_ports
}

pub fn scan_port(ip_port: &SocketAddr, timeout: Duration) -> bool {
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
    use super::*;
    use std::net::TcpListener;

    #[test]
    fn returns_true_when_port_is_open() {
        let listener: TcpListener = TcpListener::bind("127.0.0.1:0").unwrap();
        let ip_port: SocketAddr = listener.local_addr().unwrap();
        let timeout: Duration = Duration::from_millis(100);
        assert!(scan_port(&ip_port, timeout));
    }
}
