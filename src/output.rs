use crate::models::{Config, ScanSummary};

pub fn print_summary(summary: &ScanSummary, config: &Config) {
    if summary.open_ports.is_empty() {
        print!("\n\nNo open ports found");
    } else {
        println!("\n\nOpen ports");
        println!("=============");

        for open in summary.open_ports.iter() {
            print!("{open} ");
        }
    }

    println!("\n=============");
    println!("Ip: {}", config.ip);

    if config.start != config.end {
        println!("Scan range: {} - {}", config.start, config.end);
    } else {
        println!("Scan port: {}", config.start);
    }

    println!("Elapsed: {} s", summary.elapsed.as_secs_f32());
}
