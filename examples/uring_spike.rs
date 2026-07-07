use io_uring::*;
use socket2::{Domain, SockAddr, Socket, Type};
use std::net::SocketAddr;
use std::os::unix::io::AsRawFd;

fn main() {
    let mut uring = IoUring::new(4).unwrap();
    let timeout = types::Timespec::new().sec(1);

    let socket1 = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
    let addr1 = SockAddr::from("192.168.0.1:80".parse::<SocketAddr>().unwrap());
    let fd1 = socket1.as_raw_fd();
    let connect1 = opcode::Connect::new(types::Fd(fd1), addr1.as_ptr() as *const _, addr1.len())
        .build()
        .flags(squeue::Flags::IO_LINK)
        .user_data(80);
    let timeout1 = opcode::LinkTimeout::new(&timeout).build().user_data(0);

    let socket2 = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
    let addr2 = SockAddr::from("8.8.8.8:81".parse::<SocketAddr>().unwrap());
    let fd2 = socket2.as_raw_fd();
    let connect2 = opcode::Connect::new(types::Fd(fd2), addr2.as_ptr() as *const _, addr2.len())
        .build()
        .flags(squeue::Flags::IO_LINK)
        .user_data(53);
    let timeout2 = opcode::LinkTimeout::new(&timeout).build().user_data(0);

    unsafe {
        uring.submission().push(&connect1).unwrap();
        uring.submission().push(&timeout1).unwrap();
        uring.submission().push(&connect2).unwrap();
        uring.submission().push(&timeout2).unwrap();
    }

    uring.submit_and_wait(4).unwrap();

    for cqe in uring.completion() {
        let port = cqe.user_data();
        let res = cqe.result();

        if port == 0 {
            continue;
        }

        match res {
            0 => println!("Port {port} open"),
            -111 => println!("Port {port} closed"),
            -125 => println!("Port {port} filtered"),
            _ => println!("Unknown"),
        }
    }
}
