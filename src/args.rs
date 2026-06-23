use clap::{Args, Parser, Subcommand, ValueEnum};
use std::net::IpAddr;

use crate::config::OutputFormat;

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
