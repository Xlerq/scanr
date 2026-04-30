mod cli;
mod models;
mod output;
mod parser;
mod scanner;

use crate::cli::run_cli_scan;
use crate::models::{Cli, Config, ScanSummary};
use crate::output::print_summary;
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
    let config: Config = parse_cli(cli)?;
    let summary: ScanSummary = run_cli_scan(&config);

    print_summary(&summary, &config);

    Ok(())
}
