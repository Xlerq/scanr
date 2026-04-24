use std::io::{self, Write};
use std::time::{Duration, Instant};

use crate::models::{Config, ScanEvent, ScanSummary};
use crate::scanner::scan_range;

pub fn run_cli_scan(config: &Config) -> ScanSummary {
    let timer: Instant = Instant::now();

    let total_ports: u16 = count_total_ports(config);
    let mut scanned_count: u16 = u16::MIN;
    let mut live_open_count: u16 = u16::MIN;

    let open_ports: Vec<u16> = scan_range(config, |event| match event {
        ScanEvent::PortScanned => {
            scanned_count += 1;

            print!(
                "\r{}",
                render_progress(scanned_count, total_ports, live_open_count)
            );

            io::stdout().flush().unwrap();
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

fn render_progress(scanned: u16, total: u16, open_count: u16) -> String {
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

fn count_total_ports(config: &Config) -> u16 {
    let start: u16 = config.start;
    let end: u16 = config.end;

    end - start + 1
}
