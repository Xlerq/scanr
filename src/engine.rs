use std::net::Ipv4Addr;
use std::time::Duration;

use io_uring::IoUring;

use crate::threadengine::ThreadEngine;
use crate::uringengine::UringEngine;

pub enum ScanEvent {
    PortScanned,
    PortOpen,
}

pub enum TcpResult {
    PortOpen,
    PortClosed,
    NoResponse,
}

pub enum Engine {
    Thread(ThreadEngine),
    Uring(UringEngine),
}

pub trait ScanEngine {
    fn scan(
        &self,
        ip: Ipv4Addr,
        ports: &[u16],
        timeout: Duration,
        on_event: &mut dyn FnMut(ScanEvent),
    ) -> Vec<(u16, TcpResult)>;
}

impl ScanEngine for Engine {
    fn scan(
        &self,
        ip: Ipv4Addr,
        ports: &[u16],
        timeout: Duration,
        on_event: &mut dyn FnMut(ScanEvent),
    ) -> Vec<(u16, TcpResult)> {
        match self {
            Engine::Thread(e) => e.scan(ip, ports, timeout, on_event),
            Engine::Uring(e) => e.scan(ip, ports, timeout, on_event),
        }
    }
}

pub fn select() -> Engine {
    let os_test: bool = IoUring::new(1).is_ok();
    if os_test {
        Engine::Uring(UringEngine)
    } else {
        Engine::Thread(ThreadEngine)
    }
}
