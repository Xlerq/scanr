use crate::models::DiscoverConfig;
use crate::scanner::scan_port;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

pub fn discover(discover_config: &DiscoverConfig) -> Vec<IpAddr> {
    let timeout = discover_config.speed.timeout();

    let mut results: Vec<IpAddr> = Vec::new();

    for ip in discover_config.ips.iter().copied() {
        if let Some(ip_result) = discover_ip(ip, timeout) {
            results.push(ip_result);
        }
    }
    results
}

fn discover_ip(ip: IpAddr, timeout: Duration) -> Option<IpAddr> {
    for port in TCP_FALLBACK_PORTS {
        let is_active: bool = tcp_probe(ip, port, timeout);

        if is_active {
            return Some(ip);
        }
    }
    None
}

fn tcp_probe(ip: IpAddr, port: u16, timeout: Duration) -> bool {
    let socket = SocketAddr::new(ip, port);
    scan_port(&socket, timeout)
}

const TCP_FALLBACK_PORTS: [u16; 9] = [80, 443, 22, 445, 53, 3389, 8080, 139, 9100];
