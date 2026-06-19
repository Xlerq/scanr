use ipnet::IpNet;
use std::net::IpAddr;

use crate::models::{Cli, CliCommand, DiscoverConfig, ParsedCommand, ScanConfig};

const MIN_DISCOVERY_PREFIX_LEN: u8 = 16;

pub fn parse_cli(cli: Cli) -> Result<ParsedCommand, String> {
    let command = cli.command;

    match command {
        CliCommand::Scan(scan_args) => {
            let parsed_ports: Vec<u16> = parse_ports(&scan_args.ports)?;
            let scan_config: ScanConfig = ScanConfig {
                ip: scan_args.ip,
                ports: parsed_ports,
                speed: scan_args.speed.into(),
                format: scan_args.format,
            };
            Ok(ParsedCommand::Scan(scan_config))
        }
        CliCommand::Discover(discover_args) => {
            let ips = parse_cidr(&discover_args.cidr)?;
            let discover_config: DiscoverConfig = DiscoverConfig {
                ips,
                speed: discover_args.speed.into(),
                format: discover_args.format,
            };
            Ok(ParsedCommand::Discover(discover_config))
        }
    }
}

fn parse_cidr(text: &str) -> Result<Vec<IpAddr>, String> {
    let network: IpNet = text
        .trim()
        .parse()
        .map_err(|_| "Error: invalid CIDR".to_string())?;

    match network {
        IpNet::V4(ipv4_network) => {
            if ipv4_network.prefix_len() < MIN_DISCOVERY_PREFIX_LEN {
                return Err("Error: CIDR range is to large, use smaller for now".to_string());
            }

            let ips: Vec<IpAddr> = ipv4_network.hosts().map(IpAddr::V4).collect();
            Ok(ips)
        }

        IpNet::V6(_) => Err("Error: IPv6 discovery is not supported yet".to_string()),
    }
}

fn parse_ports(arg: &str) -> Result<Vec<u16>, String> {
    let mut ports: Vec<u16> = Vec::new();
    let parts: Vec<&str> = arg.split(',').collect();

    let mut i: usize = usize::MIN;
    while i < parts.len() {
        let is_range: bool = parts[i].contains('-');

        if is_range {
            let range: Vec<u16> = parse_range(parts[i])?;
            ports.extend_from_slice(&range);
        } else {
            let port: u16 = parse_port(parts[i])?;
            ports.push(port);
        }
        i += 1;
    }
    ports.sort();
    ports.dedup();
    Ok(ports)
}

fn parse_range(text: &str) -> Result<Vec<u16>, String> {
    let split: Vec<&str> = text.split('-').collect();
    if split.len() != 2 {
        return Err("Error: invalid range".to_string());
    }
    let start: u16 = parse_port(split[0])?;
    let end: u16 = parse_port(split[1])?;

    check_ports(&start, &end)?;

    let mut ports: Vec<u16> = Vec::with_capacity((end - start + 1) as usize);

    for i in start..=end {
        ports.push(i);
    }

    Ok(ports)
}

fn parse_port(text: &str) -> Result<u16, String> {
    match text.trim().parse::<u16>() {
        Ok(port) => Ok(port),
        Err(_) => Err("Error: port is not valid".to_string()),
    }
}

fn check_ports(s: &u16, e: &u16) -> Result<(), String> {
    if s > e {
        Err("Error: start_port cannot be greater than end_port".to_string())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use crate::models::{Cli, ParsedCommand, ScanConfig, ScanSpeed};

    use super::*;

    fn parse_test_cli(args: &[&str]) -> Cli {
        Cli::try_parse_from(args).expect("clap should accept valid arguments")
    }

    fn parse_test_scan_config(args: &[&str]) -> ScanConfig {
        let cli = parse_test_cli(args);

        match parse_cli(cli).expect("parser should accept valid scan command") {
            ParsedCommand::Scan(config) => config,
            ParsedCommand::Discover(_) => panic!("parser should return scan config"),
        }
    }

    #[test]
    fn parses_cli_into_config_with_single_port() {
        let config = parse_test_scan_config(&["scanr", "scan", "127.0.0.1", "67"]);

        assert_eq!(config.ip.to_string(), "127.0.0.1");
        assert_eq!(config.ports, vec![67]);
    }

    #[test]
    fn parses_port_range() {
        let ports = parse_ports("20-25").expect("parser should accept valid port range");

        assert_eq!(ports, vec![20, 21, 22, 23, 24, 25]);
    }

    #[test]
    fn parses_port_list() {
        let ports = parse_ports("22,80,443").expect("parser should accept port list");

        assert_eq!(ports, vec![22, 80, 443]);
    }

    #[test]
    fn parses_port_list_with_spaces() {
        let ports = parse_ports("22, 80, 443").expect("parser should accept spaces in port list");

        assert_eq!(ports, vec![22, 80, 443]);
    }

    #[test]
    fn parses_port_range_with_spaces() {
        let ports = parse_ports("20 - 25").expect("parser should accept spaces in port range");

        assert_eq!(ports, vec![20, 21, 22, 23, 24, 25]);
    }

    #[test]
    fn parses_mixed_port_expression() {
        let ports =
            parse_ports("22,80,100-102").expect("parser should accept mixed port expression");

        assert_eq!(ports, vec![22, 80, 100, 101, 102]);
    }

    #[test]
    fn rejects_when_range_start_is_greater_than_end() {
        let result = parse_ports("100-20");

        match result {
            Ok(_) => panic!("parser should reject reversed port range"),
            Err(err) => assert_eq!(err, "Error: start_port cannot be greater than end_port"),
        }
    }

    #[test]
    fn clap_rejects_invalid_ip() {
        let result = Cli::try_parse_from(["scanr", "scan", "19c.168.0.10.", "80"]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_when_max_port_reached() {
        let result = parse_ports("65536");

        match result {
            Ok(_) => panic!("parser should reject port above 65000"),
            Err(err) => assert_eq!(err, "Error: port is not valid"),
        }
    }

    #[test]
    fn rejects_invalid_range_format() {
        let result = parse_ports("1-2-3");

        match result {
            Ok(_) => panic!("parser should reject invalid range format"),
            Err(err) => assert_eq!(err, "Error: invalid range"),
        }
    }

    #[test]
    fn clap_rejects_extra_positional_argument() {
        let result = Cli::try_parse_from(["scanr", "scan", "127.0.0.1", "80", "443"]);

        assert!(result.is_err());
    }

    #[test]
    fn parses_fast_speed_flag() {
        let config =
            parse_test_scan_config(&["scanr", "scan", "127.0.0.1", "80", "--speed", "fast"]);

        match config.speed {
            ScanSpeed::Fast => {}
            _ => panic!("parser returned wrong scan speed"),
        }
    }

    #[test]
    fn parses_thorough_speed_flag_with_range() {
        let config =
            parse_test_scan_config(&["scanr", "scan", "127.0.0.1", "20-25", "--speed", "thorough"]);

        assert_eq!(config.ports, vec![20, 21, 22, 23, 24, 25]);

        match config.speed {
            ScanSpeed::Thorough => {}
            _ => panic!("parser returned wrong scan speed"),
        }
    }

    #[test]
    fn clap_rejects_invalid_speed_flag_value() {
        let result = Cli::try_parse_from(["scanr", "scan", "127.0.0.1", "80", "--speed", "slow"]);

        assert!(result.is_err());
    }

    #[test]
    fn clap_rejects_missing_speed_flag_value() {
        let result = Cli::try_parse_from(["scanr", "scan", "127.0.0.1", "80", "--speed"]);

        assert!(result.is_err());
    }
}
