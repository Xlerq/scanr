use clap::{Args, Parser, Subcommand, ValueEnum};
use std::{net::IpAddr, time::Duration};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand)]
pub enum CliCommand {
    Scan(ScanArgs),
    Discover(DiscoverArgs),
}

#[derive(Args)]
pub struct ScanArgs {
    /// Target IP address to scan
    pub ip: IpAddr,
    /// Ports to scan, e.g 80, 20-25, 22,80,443
    pub ports: String,
    /// Scan speed preset
    #[arg(long, value_enum, default_value_t = CliSpeed::Normal)]
    pub speed: CliSpeed,
    /// Specify output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
    pub format: OutputFormat,
}

#[derive(Args)]
pub struct DiscoverArgs {
    /// Target IP address to discover
    pub cidr: String,
    /// Discovery speed preset
    #[arg(long, value_enum, default_value_t = CliSpeed::Normal)]
    pub speed: CliSpeed,
    /// Specify output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
    pub format: OutputFormat,
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

pub struct ScanSummary {
    pub open_ports: Vec<u16>,
    pub elapsed: Duration,
}

pub struct DiscoverSummary {
    pub alive_hosts: Vec<IpAddr>,
    pub scanned_hosts: usize,
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
