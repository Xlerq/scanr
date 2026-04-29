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

    let scanned_ports: usize = config.ports.len();

    if scanned_ports > 10 {
        write!(writer, "Scanned ports: ")?;
        for i in 0..10 {
            write!(writer, "{} ", config.ports[i])?;
        }
        write!(
            writer,
            "... {} ({} total)",
            config.ports.last().unwrap(),
            config.ports.len()
        )?;
    } else if scanned_ports == 1 {
        write!(writer, "Scanned port: {}", config.ports[0])?;
    } else {
        write!(writer, "Scanned ports: ")?;
        for i in 0..scanned_ports {
            write!(writer, "{} ", config.ports[i])?;
        }
    }

    writeln!(writer, "\nElapsed: {} s", summary.elapsed.as_secs_f32())?;

    Ok(())
}
