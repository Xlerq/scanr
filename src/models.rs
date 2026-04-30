use clap::{Parser, ValueEnum};
use std::{net::IpAddr, time::Duration};

#[derive(Parser)]
pub struct Cli {
    /// Target IP address to scan
    pub ip: IpAddr,
    /// Ports to scan, e.g 80, 20-25, 22,80,443
    pub ports: String,
    /// Scan speed preset
    #[arg(long, value_enum, default_value_t = CliSpeed::Normal)]
    pub speed: CliSpeed,
}

#[derive(Clone, ValueEnum)]
pub enum CliSpeed {
    /// Short timeout for quicker scans, LAN
    Fast,
    /// Default timeout, generally good
    Normal,
    /// Longer timeout for high lateny large networks, VPN
    Thorough,
}

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

impl From<CliSpeed> for ScanSpeed {
    fn from(speed: CliSpeed) -> Self {
        match speed {
            CliSpeed::Fast => ScanSpeed::Fast,
            CliSpeed::Normal => ScanSpeed::Normal,
            CliSpeed::Thorough => ScanSpeed::Thorough,
        }
    }
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
