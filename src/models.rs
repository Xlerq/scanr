use std::{net::IpAddr, time::Duration};

pub struct Config {
    pub ip: IpAddr,
    pub ports: Vec<u16>,
    pub speed: ScanSpeed,
}

pub struct ScanSummary {
    pub open_ports: Vec<u16>,
    pub elapsed: Duration,
}

pub enum ScanEvent {
    PortScanned,
    PortOpen,
}

pub enum ScanSpeed {
    Fast,
    Normal,
    Thorough,
}

impl ScanSpeed {
    pub fn timeout(&self) -> Duration {
        match self {
            ScanSpeed::Fast => Duration::from_millis(100),
            ScanSpeed::Normal => Duration::from_millis(300),
            ScanSpeed::Thorough => Duration::from_millis(1000),
        }
    }
}
