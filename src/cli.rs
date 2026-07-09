use std::io::{self, IsTerminal, Write};
use std::net::IpAddr;
use std::time::{Duration, Instant};

use crate::config::{DiscoverConfig, OutputFormat, ScanConfig};
use crate::discover::{DiscoverEvent, discover};
use crate::engine::{ScanEngine, ScanEvent, TcpResult, select};
use crate::output::{DiscoverSummary, ScanSummary};

const FRAME_BUDGET: Duration = Duration::from_millis(16);

pub fn run_cli_scan(config: &ScanConfig) -> ScanSummary {
    let engine = select();
    let timer: Instant = Instant::now();
    let mut last_draw: Instant = Instant::now();

    let total_ports: usize = config.ports.len();
    let mut scanned_count: usize = usize::MIN;
    let mut live_open_count: usize = usize::MIN;

    let show_progress: bool = matches!(config.format, OutputFormat::Table);
    let is_terminal: bool = io::stdout().is_terminal();

    let verdicts: Vec<(u16, TcpResult)> = engine.scan(
        config.ip,
        &config.ports,
        config.speed.timeout(),
        &mut |event| match event {
            ScanEvent::PortScanned => {
                scanned_count += 1;

                if show_progress && is_terminal && last_draw.elapsed() >= FRAME_BUDGET {
                    print!(
                        "\r{}",
                        render_progress(scanned_count, total_ports, live_open_count)
                    );

                    io::stdout().flush().unwrap();
                    last_draw = Instant::now();
                }
            }

            ScanEvent::PortOpen => {
                live_open_count += 1;
            }
        },
    );

    if show_progress && is_terminal {
        print!(
            "\r{}",
            render_progress(scanned_count, total_ports, live_open_count)
        );
        io::stdout().flush().unwrap();
    }
    let elapsed: Duration = timer.elapsed();

    let open_ports: Vec<u16> = verdicts
        .iter()
        .filter(|(_, r)| matches!(r, TcpResult::PortOpen))
        .map(|(p, _)| *p)
        .collect();

    ScanSummary {
        open_ports,
        elapsed,
    }
}

pub fn run_cli_discovery(config: &DiscoverConfig) -> DiscoverSummary {
    let timer: Instant = Instant::now();
    let mut last_draw: Instant = Instant::now();

    let total_ips_number: usize = config.ips.len();
    let mut discover_count: usize = usize::MIN;
    let mut live_hosts_up: usize = usize::MIN;

    let show_progress: bool = matches!(config.format, OutputFormat::Table);
    let is_terminal: bool = io::stdout().is_terminal();

    let alive_hosts: Vec<IpAddr> = discover(config, |event| match event {
        DiscoverEvent::HostScanned => {
            discover_count += 1;
            if show_progress && is_terminal && last_draw.elapsed() >= FRAME_BUDGET {
                print!(
                    "\r{}",
                    render_progress(discover_count, total_ips_number, live_hosts_up)
                );

                io::stdout().flush().unwrap();
                last_draw = Instant::now();
            }
        }
        DiscoverEvent::HostUp => {
            live_hosts_up += 1;
        }
    });

    if show_progress && is_terminal {
        print!(
            "\r{}",
            render_progress(discover_count, total_ips_number, live_hosts_up)
        );
        io::stdout().flush().unwrap();
    }

    let elapsed: Duration = timer.elapsed();

    DiscoverSummary {
        alive_hosts,
        scanned_hosts: total_ips_number,
        elapsed,
    }
}

fn render_progress(scanned: usize, total: usize, open_count: usize) -> String {
    let bar_width: usize = 26;

    let ratio: f32 = if total == 0 {
        0.0
    } else {
        scanned as f32 / total as f32
    };

    let full_blocks: usize = (ratio * bar_width as f32).round() as usize;

    let mut bar = String::new();

    for _ in 0..full_blocks {
        bar.push('█');
    }

    while bar.chars().count() < bar_width {
        bar.push('·');
    }

    format!("⟦{}⟧ {}/{}  open: {}", bar, scanned, total, open_count)
}
