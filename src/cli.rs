use std::io::{self, Write};
use std::time::{Duration, Instant};

use crate::models::{Config, OutputFormat, ScanEvent, ScanSummary};
use crate::scanner::scan_ports;

pub fn run_cli_scan(config: &Config) -> ScanSummary {
    let timer: Instant = Instant::now();

    let total_ports: usize = config.ports.len();
    let mut scanned_count: usize = usize::MIN;
    let mut live_open_count: usize = usize::MIN;

    let show_progress: bool = matches!(config.format, OutputFormat::Table);

    let open_ports: Vec<u16> = scan_ports(config, |event| match event {
        ScanEvent::PortScanned => {
            scanned_count += 1;

            if show_progress {
                print!(
                    "\r{}",
                    render_progress(scanned_count, total_ports, live_open_count)
                );

                io::stdout().flush().unwrap();
            }
        }

        ScanEvent::PortOpen => {
            live_open_count += 1;
        }
    });

    let elapsed: Duration = timer.elapsed();

    ScanSummary {
        open_ports,
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
