use std::{
    env,
    net::{IpAddr, SocketAddr, TcpStream},
    time::{Duration, Instant},
};

fn check_len(v: &[String]) -> bool
{
    if v.len() > 4
    {
        println!("Error: too many arguments");
        println!("Usage: scanr <ip> <start_port> [end_port]");
        false
    }
    else if v.len() < 3
    {
        println!("Error: too few arguments");
        println!("Usage: scanr <ip> <start_port> [end_port]");
        false
    }
    else
    {
        true
    }
}

fn check_and_parse_ip(ip: &str) -> Option<IpAddr>
{
    match ip.parse::<IpAddr>()
    {
        Ok(addr) => Some(addr),
        Err(_) =>
        {
            eprintln!("Error: invalid IP address");
            None
        }
    }
}

fn parse_port(text: &str, field: &str) -> Option<u16>
{
    match text.parse()
    {
        Ok(port) => Some(port),
        Err(_) =>
        {
            eprintln!("Error {field} port must be a number");
            None
        }
    }
}

fn check_ports(s: &u16, e: &u16) -> bool
{
    if s > e
    {
        println!("Error: start_port cannot be greater than end_port");
        false
    }
    else
    {
        true
    }
}

fn scan_port(ip_port: &SocketAddr) -> bool
{
    let timeout: Duration = Duration::from_millis(500);
    TcpStream::connect_timeout(ip_port, timeout).is_ok()
}

fn scan_range(ip: IpAddr, start: u16, end: u16) -> Vec<u16>
{
    let mut open_ports: Vec<u16> = Vec::new();
    for i in start..=end
    {
        let ip_port: SocketAddr = SocketAddr::new(ip, i);
        if scan_port(&ip_port)
        {
            open_ports.push(i);
        }
    }
    open_ports
}

fn print_summary(
    open_ports: &Vec<u16>,
    elapsed_time: &Duration,
    ip: &IpAddr,
    start: &u16,
    end: &u16,
)
{
    if open_ports.is_empty()
    {
        print!("No open ports found");
    }
    else
    {
        println!("Open ports");
        println!("=============");

        for open in open_ports
        {
            print!("{open} ");
        }
    }

    println!("\n=============");
    println!("Ip: {ip}");

    if start != end
    {
        println!("Scan range: {start} - {end}");
    }
    else
    {
        println!("Scan port: {start}");
    }

    println!("Elapsed: {} s", elapsed_time.as_secs_f32());
}

fn main()
{
    let args: Vec<String> = env::args().collect();

    if !check_len(&args)
    {
        return;
    }

    let ip: IpAddr = match check_and_parse_ip(&args[1])
    {
        Some(ip) => ip,
        None => return,
    };

    let start: u16 = match parse_port(&args[2], "start")
    {
        Some(port) => port,
        None => return,
    };

    let end: u16 = if args.len() == 4
    {
        match parse_port(&args[3], "end")
        {
            Some(port) => port,
            None => return,
        }
    }
    else
    {
        start
    };

    if !check_ports(&start, &end)
    {
        return;
    }

    let timer: Instant = Instant::now();

    let open_ports: Vec<u16> = scan_range(ip, start, end);

    let elapsed: Duration = timer.elapsed();
    print_summary(&open_ports, &elapsed, &ip, &start, &end);
}
