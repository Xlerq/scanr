use crate::models::DiscoverConfig;
use crate::scanner::scan_port;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

pub fn discover(discover_config: DiscoverConfig) -> Vec<SocketAddr> {
    let ips: Vec<IpAddr> = discover_config.ips;
    let timeout = discover_config.speed.timeout();

    let mut results: Vec<SocketAddr> = Vec::new();

    for ip in ips {
        let mut ip_results: Vec<SocketAddr> = discover_ip(ip, timeout);
        results.append(&mut ip_results);
    }
    results
}

fn discover_ip(ip: IpAddr, timeout: Duration) -> Vec<SocketAddr> {
    let mut results = Vec::new();

    for port in TCP_FALLBACK_PORTS {
        if let Some(port) = tcp_probe(ip, port, timeout) {
            results.push(SocketAddr::new(ip, port));
        }
    }

    results
}

fn tcp_probe(ip: IpAddr, port: u16, timeout: Duration) -> Option<u16> {
    let socket = SocketAddr::new(ip, port);
    if scan_port(&socket, timeout) {
        Some(port)
    } else {
        None
    }
}

const TCP_FALLBACK_PORTS: [u16; 9] = [22, 53, 80, 135, 139, 443, 3389, 8080, 9100];
