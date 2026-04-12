use std::{env, net::TcpStream};


fn check_len(v: &[String]) -> bool
{
    if v.len() > 4 
    {
        println!("Error: Too many arguments");
        false
    }
    else if v.len() < 3
    {
        println!("Error: Too few arguments");
        false
    }
    else {true}
}

fn check_ports(s: &u16, e: &u16) -> bool
{
    if s > e
    {
        println!("Error: Start port > End port");
        false
    }
    else {true}
}

fn scan_port(ip: &str, port: u16) -> bool
{
    TcpStream::connect((ip, port)).is_ok()
}

fn main()
{
    let args: Vec<String> = env::args().collect();

    if !check_len(&args) {return;}

    let ip: &str = &args[1];
    let start: u16 = args[2].parse().unwrap();

    let end: u16 = if args.len() == 4
    {
        args[3].parse().unwrap()
    }
    else {start};

    if !check_ports(&start, &end) {return;}

    println!("Ip: {ip}");
    println!("Start: {start}");
    println!("End: {end}");

    if !scan_port(ip, start)
    {
        println!("Port zamkniety");
    }
    else {println!("Port otwarty")}
}
