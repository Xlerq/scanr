use std::{net::IpAddr, time::Duration};

pub struct Config {
    pub ip: IpAddr,
    pub start: u16,
    pub end: u16,
}

pub struct ScanSummary {
    pub open_ports: Vec<u16>,
    pub elapsed: Duration,
}

pub enum ScanEvent {
    PortScanned,
    PortOpen(),
}
