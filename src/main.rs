mod models;
mod output;
mod parser;
mod scanner;

use std::env;

use crate::models::{Config, ScanSummary};
use crate::output::print_summary;
use crate::parser::parse_args;
use crate::scanner::scan_range;

fn main() {
    match run() {
        Ok(()) => (),
        Err(err) => eprintln!("{err}"),
    };
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let config: Config = parse_args(&args)?;
    let summary: ScanSummary = scan_range(&config);

    print_summary(&summary, &config);

    Ok(())
}
