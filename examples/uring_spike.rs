use io_uring::squeue::Flags;
use io_uring::types::Fd;
use io_uring::*;
use socket2::{Domain, SockAddr, Socket, Type};
use std::net::SocketAddr;
use std::os::unix::io::AsRawFd;

fn main() {
    let ports: Vec<u16> = (1..=15000).collect();
    let n = ports.len();

    let mut uring = IoUring::new(2 * n as u32).unwrap();
    let timeout = types::Timespec::new().sec(1);

    let mut sockets = Vec::with_capacity(n);
    let mut addrs = Vec::with_capacity(n);

    for &port in &ports {
        let socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
        let addr = SockAddr::from(format!("192.168.0.1:{port}").parse::<SocketAddr>().unwrap());
        sockets.push(socket);
        addrs.push(addr);
    }

    for i in 0..n {
        let fd = sockets[i].as_raw_fd();
        let connect = opcode::Connect::new(Fd(fd), addrs[i].as_ptr() as *const _, addrs[i].len())
            .build()
            .flags(Flags::IO_LINK)
            .user_data(ports[i] as u64);
        let link_timeout = opcode::LinkTimeout::new(&timeout).build().user_data(0);

        unsafe {
            uring.submission().push(&connect).unwrap();
            uring.submission().push(&link_timeout).unwrap();
        }
    }

    uring.submit_and_wait(2 * n).unwrap();

    for cqe in uring.completion() {
        let port = cqe.user_data();
        let res = cqe.result();

        if port == 0 {
            continue;
        }

        match res {
            0 => println!("Port {port} open"),
            //-111 => println!("Port {port} closed"),
            //-125 => println!("Port {port} filtered"),
            _ => continue,
        }
    }
}
