use std::net::IpAddr;
use std::time::Duration;

pub enum ScanEvent {
    PortScanned,
    PortOpen,
}

pub enum TcpResult {
    PortOpen,
    PortClosed,
    NoResponse,
}

pub trait ScanEngine {
    fn scan(
        &self,
        ip: IpAddr,
        ports: &[u16],
        timeout: Duration,
        on_event: &mut dyn FnMut(ScanEvent),
    ) -> Vec<(u16, TcpResult)>;
}
