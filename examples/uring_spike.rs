use io_uring::*;
use socket2::{Domain, SockAddr, Socket, Type};
use std::net::SocketAddr;
use std::os::unix::io::AsRawFd;

fn main() {
    let mut uring = IoUring::new(1).unwrap();
    let socket: Socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();

    let socketaddr: SocketAddr = "192.168.0.1:81".parse().unwrap();
    let sock_addr: SockAddr = SockAddr::from(socketaddr);

    let fd: i32 = socket.as_raw_fd();

    let connect = opcode::Connect::new(
        types::Fd(fd),
        sock_addr.as_ptr() as *const _,
        sock_addr.len(),
    )
    .build()
    .user_data(80);

    unsafe {
        uring.submission().push(&connect).unwrap();
        uring.submit_and_wait(1).unwrap();

        let cqe = uring.completion().next().unwrap();
        println!("port {} -> result {}", cqe.user_data(), cqe.result());
    }
}
