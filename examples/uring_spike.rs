use io_uring::squeue::Flags;
use io_uring::types::Fd;
use io_uring::*;
use socket2::{Domain, SockAddr, Socket, Type};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::os::unix::io::AsRawFd;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    const CONCURRENCY: usize = 16384;
    const TIMEOUT: u32 = 10000000;
    let ports: Vec<u16> = (1..=65535).collect();
    let n: u16 = ports.len() as u16;

    let mut uring = IoUring::new(2 * CONCURRENCY as u32).unwrap();
    let timeout = types::Timespec::new().nsec(TIMEOUT);

    let addrs: Vec<SockAddr> = ports
        .iter()
        .map(|&p| SockAddr::from(format!("192.168.0.1:{p}").parse::<SocketAddr>().unwrap()))
        .collect();

    let mut sockets: HashMap<u16, Socket> = HashMap::new();
    let mut next: u16 = u16::MIN;
    let mut remaining: u16 = n;
    let mut open: Vec<u16> = Vec::new();

    while remaining > 0 {
        while sockets.len() < CONCURRENCY && next < n {
            let port = ports[next as usize];
            let socket: Socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
            let fd: i32 = socket.as_raw_fd();

            let connect = opcode::Connect::new(
                Fd(fd),
                addrs[next as usize].as_ptr() as *const _,
                addrs[next as usize].len(),
            )
            .build()
            .flags(Flags::IO_LINK)
            .user_data(port as u64);

            let timeout = opcode::LinkTimeout::new(&timeout).build().user_data(0);

            unsafe {
                uring.submission().push(&connect).unwrap();
                uring.submission().push(&timeout).unwrap();
            }

            sockets.insert(port, socket);
            next += 1;
        }

        uring.submit_and_wait(1).unwrap();

        let batch: Vec<(u16, i32)> = uring
            .completion()
            .map(|c| (c.user_data() as u16, c.result()))
            .collect();

        for (port, res) in batch {
            if port == 0 {
                continue;
            }
            if res == 0 {
                open.push(port);
            }
            sockets.remove(&port);
            remaining -= 1;
        }
    }

    for p in &open {
        println!("Port {p} is open");
    }
    let elapsed: f32 = now.elapsed().as_secs_f32();

    println!("{elapsed}");
}
