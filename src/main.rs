mod cli;
mod discover;
mod models;
mod output;
mod parser;
mod scanner;

use crate::cli::{run_cli_discovery, run_cli_scan};
use crate::models::{Cli, ParsedCommand, ScanSummary};
use crate::output::{print_discovery_summary, print_scan_summary};
use crate::parser::parse_cli;
use clap::Parser;

fn main() {
    match run() {
        Ok(()) => (),
        Err(err) => eprintln!("{err}"),
    };
}

fn run() -> Result<(), String> {
    let cli: Cli = Cli::parse();
    let command = parse_cli(cli)?;

    match command {
        ParsedCommand::Scan(scan_config) => {
            let summary: ScanSummary = run_cli_scan(&scan_config);
            print_scan_summary(&summary, &scan_config);
            Ok(())
        }
        ParsedCommand::Discover(discover_config) => {
            let summary = run_cli_discovery(&discover_config);
            print_discovery_summary(&summary, &discover_config);
            Ok(())
        }
    }
}
