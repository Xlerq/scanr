use std::net::IpAddr;

use crate::models::Config;

pub fn parse_args(args: &[String]) -> Result<Config, String> {
    check_len(args)?;
    let ip: IpAddr = check_and_parse_ip(&args[1])?;
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

fn check_and_parse_ip(ip: &str) -> Result<IpAddr, String> {
    match ip.parse::<IpAddr>() {
        Ok(addr) => Ok(addr),
        Err(_) => Err("Error: invalid IP address".to_string()),
    }
}

fn parse_port(text: &str, field: &str) -> Result<u16, String> {
    match text.parse::<u16>() {
        Ok(port) => Ok(port),
        Err(_) => Err(format!("Error: {field} port must be a number")),
    }
}

fn check_ports(s: &u16, e: &u16) -> Result<(), String> {
    if s > e {
        Err("Error: start_port cannot be greater than end_port".to_string())
    } else {
        Ok(())
    }
}
