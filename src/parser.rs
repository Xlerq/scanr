use std::net::IpAddr;

use crate::models::Config;

pub fn parse_args(args: &[String]) -> Result<Config, String> {
    check_len(args)?;
    let ip: IpAddr = parse_ip(&args[1])?;
    let start: u16 = parse_port(&args[2], "start")?;
    let end: u16 = if args.len() == 4 {
        parse_port(&args[3], "end")?
    } else {
        start
    };

    check_ports(&start, &end)?;

    let config: Config = Config { ip, start, end };
    Ok(config)
}

fn check_len(v: &[String]) -> Result<(), String> {
    let length = v.len();
    if length > 4 {
        Err("Error: too many arguments\nUsage: scanr <ip> <start_port> [end_port]".to_string())
    } else if length < 3 {
        Err("Error: too few arguments\nUsage: scanr <ip> <start_port> [end_port]".to_string())
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

fn parse_port(text: &str, field: &str) -> Result<u16, String> {
    match text.parse::<u16>() {
        Ok(port) => Ok(port),
        Err(_) => Err(format!("Error: {field} port is not valid")),
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
}
