use std::net::IpAddr;

use crate::models::{Config, ScanSpeed};

pub fn parse_args(args: &[String]) -> Result<Config, String> {
    check_len(args)?;
    let ip: IpAddr = parse_ip(&args[1])?;
    let ports: Vec<u16> = parse_ports(&args[2])?;
    let mut speed: ScanSpeed = ScanSpeed::Normal;

    let mut i: usize = 3;

    while i < args.len() {
        let arg: &str = args[i].as_str();

        match arg {
            "--speed" => {
                let speed_arg =
                    match args.get(i + 1) {
                        Some(speed) => speed,
                        None => return Err(
                            "Error: missing speed argument\nUsage --speed [fast|normal|thorough]"
                                .to_string(),
                        ),
                    };
                speed = parse_speed(speed_arg)?;
                i += 2;
            }
            _ if arg.starts_with("--") => {
                return Err("Error: invalid flag".to_string());
            }
            _ => {
                return Err("Error too many arguments".to_string());
            }
        }
    }

    let config: Config = Config { ip, ports, speed };
    Ok(config)
}

fn check_len(v: &[String]) -> Result<(), String> {
    if v.len() < 3 {
        Err("Error: too few arguments\nUsage: scanr <ip> <ports>".to_string())
    } else {
        Ok(())
    }
}

fn parse_ip(ip: &str) -> Result<IpAddr, String> {
    match ip.parse::<IpAddr>() {
        Ok(addr) => Ok(addr),
        Err(_) => Err("Error: invalid IP address".to_string()),
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
    Ok(ports)
}

fn parse_range(text: &str) -> Result<Vec<u16>, String> {
    let split: Vec<&str> = text.split('-').collect();
    if split.len() >= 2 {
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
    match text.parse::<u16>() {
        Ok(port) => Ok(port),
        Err(_) => Err("Error: port is not valid".to_string()),
    }
}

fn parse_speed(text: &str) -> Result<ScanSpeed, String> {
    match text {
        "fast" => Ok(ScanSpeed::Fast),
        "normal" => Ok(ScanSpeed::Normal),
        "thorough" => Ok(ScanSpeed::Thorough),
        _ => Err("Error: invalid speed argument\nUsage --speed [fast|normal|thorough]".to_string()),
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
    use super::*;

    fn make_args(args: &[&str]) -> Vec<String> {
        args.iter().map(|arg| arg.to_string()).collect()
    }

    #[test]
    fn parses_valid_ip_and_range() {
        let args = make_args(&["scanr", "127.0.0.1", "20", "25"]);
        let config = parse_args(&args).expect("parser should accept valid arguments");

        assert_eq!(config.ip.to_string(), "127.0.0.1");
        assert_eq!(config.start, 20);
        assert_eq!(config.end, 25);
    }

    #[test]
    fn rejects_when_start_is_greater_than_end() {
        let args = make_args(&["scanr", "127.0.0.1", "100", "20"]);
        let result = parse_args(&args);

        match result {
            Ok(_) => panic!("parser should reject reversed port range"),
            Err(err) => assert_eq!(err, "Error: start_port cannot be greater than end_port"),
        }
    }

    #[test]
    fn parses_valid_single_port() {
        let args = make_args(&["scanr", "127.0.0.1", "67"]);
        let config = parse_args(&args).expect("parser should output valid single port");

        assert_eq!(config.start, 67);
        assert_eq!(config.end, 67);
    }

    #[test]
    fn rejects_invalid_ip() {
        match parse_ip("19c.168.0.10.") {
            Ok(_) => panic!("parser should reject invalid IP address"),
            Err(err) => assert_eq!(err, "Error: invalid IP address"),
        };
    }

    #[test]
    fn rejects_when_max_port_reached() {
        let args = make_args(&["scanr", "127.0.0.1", "64999", "65536"]);
        let result = parse_args(&args);

        match result {
            Ok(_) => panic!("parser should reject port above 65000"),
            Err(err) => assert_eq!(err, "Error: end port is not valid"),
        }
    }

    #[test]
    fn returns_valid_scan_speed() {
        let text: &str = "fast";
        let result: Result<ScanSpeed, String> = parse_speed(text);

        match result {
            Ok(ScanSpeed::Fast) => {}
            Ok(_) => panic!("parser returned wrong scan speed"),
            Err(err) => panic!("parser should return valid scan_speed\ngot error: {err}"),
        }
    }

    #[test]
    fn parses_fast_speed_flag() {
        let args = make_args(&["scanr", "127.0.0.1", "80", "--speed", "fast"]);
        let config = parse_args(&args).expect("parser should accept fast scan speed");

        match config.speed {
            ScanSpeed::Fast => {}
            _ => panic!("parser returned wrong scan speed"),
        }
    }

    #[test]
    fn parses_thorough_speed_flag_with_range() {
        let args = make_args(&["scanr", "127.0.0.1", "20", "25", "--speed", "thorough"]);
        let config = parse_args(&args).expect("parser should accept thorough scan speed");

        assert_eq!(config.start, 20);
        assert_eq!(config.end, 25);

        match config.speed {
            ScanSpeed::Thorough => {}
            _ => panic!("parser returned wrong scan speed"),
        }
    }

    #[test]
    fn rejects_invalid_speed_flag_value() {
        let args = make_args(&["scanr", "127.0.0.1", "80", "--speed", "slow"]);
        let result = parse_args(&args);

        match result {
            Ok(_) => panic!("parser should reject invalid scan speed"),
            Err(err) => assert_eq!(
                err,
                "Error: invalid speed argument\nUsage --speed [fast|normal|thorough]"
            ),
        }
    }

    #[test]
    fn rejects_missing_speed_flag_value() {
        let args = make_args(&["scanr", "127.0.0.1", "80", "--speed"]);
        let result = parse_args(&args);

        match result {
            Ok(_) => panic!("parser should reject missing scan speed"),
            Err(err) => assert_eq!(
                err,
                "Error: missing speed argument\nUsage --speed [fast|normal|thorough]"
            ),
        }
    }
}
