use std::{env,
net::
{
    IpAddr,
    SocketAddr,
    TcpStream
},
time::
{
    Duration,
    Instant
}};



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
    else {true}
}

fn check_and_parse_ip(ip: &str) -> Option<IpAddr>
{
    match ip.parse::<IpAddr>()
    {
        Ok(addr) => Some(addr),
        Err(_) =>
        {
            eprintln!("Error: invalid socket address");
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
    else {true}
}

fn scan_port(ip_port: &SocketAddr) -> bool
{
    
    let timeout: Duration = Duration::from_millis(500);
    TcpStream::connect_timeout(ip_port, timeout).is_ok()
}


fn main()
{
    let args: Vec<String> = env::args().collect();

    if !check_len(&args) {return;}

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
    else {start};

    if !check_ports(&start, &end) {return;}
    
    let mut open_count: u16 = u16::MIN;
    let timer: Instant = Instant::now();

    for i in start..=end
    {
        let ip_port: SocketAddr = SocketAddr::new(ip, i);
        if scan_port(&ip_port)
        {
            println!("{i} is open");
            open_count += 1;
        }
    }

    let elapsed: Duration = timer.elapsed();

    if open_count == 0 {println!("No open ports found")};

    println!("Elapsed: {} s", elapsed.as_secs_f32());
    
}
