use std::io::{self, Write};

use crate::models::{Config, ScanSummary};

pub fn print_summary(summary: &ScanSummary, config: &Config) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    write_summary(&mut handle, summary, config).expect("failed to write scan summary");
}

fn write_summary<W: Write>(
    writer: &mut W,
    summary: &ScanSummary,
    config: &Config,
) -> io::Result<()> {
    if summary.open_ports.is_empty() {
        writeln!(writer, "\n\nNo open ports found")?;
    } else {
        writeln!(writer, "\n\nOpen ports")?;
        writeln!(writer, "=============")?;

        for open in summary.open_ports.iter() {
            write!(writer, "{open} ")?;
        }

        writeln!(writer)?;
    }

    writeln!(writer, "=============")?;
    writeln!(writer, "Ip: {}", config.ip)?;

    if config.start != config.end {
        writeln!(writer, "Scan range: {} - {}", config.start, config.end)?;
    } else {
        writeln!(writer, "Scan port: {}", config.start)?;
    }

    writeln!(writer, "Elapsed: {} s", summary.elapsed.as_secs_f32())?;

    Ok(())
}
