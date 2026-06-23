use clap::ValueEnum;
use std::net::IpAddr;
use std::time::Duration;

use crate::args::CliSpeed;

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Table,
    Csv,
    Json,
}

pub struct ScanConfig {
    pub ip: IpAddr,
    pub ports: Vec<u16>,
    pub speed: ScanSpeed,
    pub format: OutputFormat,
}

pub struct DiscoverConfig {
    pub ips: Vec<IpAddr>,
    pub speed: ScanSpeed,
    pub format: OutputFormat,
}

pub enum ParsedCommand {
    Scan(ScanConfig),
    Discover(DiscoverConfig),
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
