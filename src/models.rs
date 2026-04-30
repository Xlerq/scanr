use clap::{Parser, ValueEnum};
use std::{net::IpAddr, time::Duration};

#[derive(Parser)]
#[command(name = "scanr", about = "Minimal CLI port scanner")]
pub struct Cli {
    pub ip: IpAddr,
    pub ports: String,
    #[arg(long, value_enum, default_value_t = CliSpeed::Normal)]
    pub speed: CliSpeed,
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

#[derive(Clone, ValueEnum)]
pub enum CliSpeed {
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
