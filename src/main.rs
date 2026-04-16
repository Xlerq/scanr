use std::{
    env,
    net::{IpAddr, SocketAddr, TcpStream},
    time::{Duration, Instant},
};

struct Config {
    ip: IpAddr,
    start: u16,
    end: u16,
}

struct ScanSummary {
    open_ports: Vec<u16>,
    elapsed: Duration,
}

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

fn parse_args(args: &[String]) -> Result<Config, String> {
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
    if v.len() > 4 {
        Err("Error: too many arguments\nUsage: scanr <ip> <start_port> [end_port]".to_string())
    } else if v.len() < 3 {
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

fn scan_port(ip_port: &SocketAddr) -> bool {
    let timeout: Duration = Duration::from_millis(200);
    TcpStream::connect_timeout(ip_port, timeout).is_ok()
}

fn scan_range(config: &Config) -> ScanSummary {
    let timer: Instant = Instant::now();
    let mut open_ports: Vec<u16> = Vec::new();

    for i in config.start..=config.end {
        let ip_port: SocketAddr = SocketAddr::new(config.ip, i);
        if scan_port(&ip_port) {
            open_ports.push(i);
        }
    }

    let elapsed: Duration = timer.elapsed();

    let summary: ScanSummary = ScanSummary {
        open_ports,
        elapsed,
    };
    summary
}

fn print_summary(summary: &ScanSummary, config: &Config) {
    if summary.open_ports.is_empty() {
        print!("No open ports found");
    } else {
        println!("Open ports");
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
