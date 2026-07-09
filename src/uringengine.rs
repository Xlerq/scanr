use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::os::unix::io::AsRawFd;
use std::time::Duration;

use io_uring::squeue::Flags;
use io_uring::types::Fd;
use io_uring::{IoUring, opcode, types};
use socket2::{Domain, SockAddr, Socket, Type};

use crate::engine::{ScanEngine, ScanEvent, TcpResult};

pub struct UringEngine;

impl ScanEngine for UringEngine {
    fn scan(
        &self,
        ip: IpAddr,
        ports: &[u16],
        timeout: Duration,
        on_event: &mut dyn FnMut(ScanEvent),
    ) -> Vec<(u16, TcpResult)> {
        if ports.is_empty() {
            return Vec::new();
        }

        // TODO: wyprowadzić z RLIMIT_NOFILE
        const CONCURRENCY: usize = 16384;
        let window = ports.len().min(CONCURRENCY);
        let n = ports.len();

        let mut uring = IoUring::new((2 * window) as u32).unwrap();
        let timeout_spec = types::Timespec::from(timeout);

        let addrs: Vec<SockAddr> = ports
            .iter()
            .map(|&p| SockAddr::from(SocketAddr::new(ip, p)))
            .collect();

        let mut sockets: HashMap<u16, Socket> = HashMap::new();
        let mut next: usize = 0;
        let mut remaining: usize = n;
        let mut results: Vec<(u16, TcpResult)> = Vec::with_capacity(n);

        while remaining > 0 {
            while sockets.len() < window && next < n {
                let port = ports[next];
                let socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
                let fd = socket.as_raw_fd();

                let connect = opcode::Connect::new(
                    Fd(fd),
                    addrs[next].as_ptr() as *const _,
                    addrs[next].len(),
                )
                .build()
                .flags(Flags::IO_LINK)
                .user_data(port as u64);

                let link_timeout = opcode::LinkTimeout::new(&timeout_spec).build().user_data(0);

                unsafe {
                    uring.submission().push(&connect).unwrap();
                    uring.submission().push(&link_timeout).unwrap();
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

                let verdict = match res {
                    0 => TcpResult::PortOpen,
                    -111 => TcpResult::PortClosed, // ECONNREFUSED
                    _ => TcpResult::NoResponse,    // -125 (timeout) i inne
                };

                if res == 0 {
                    on_event(ScanEvent::PortOpen);
                }
                on_event(ScanEvent::PortScanned);

                results.push((port, verdict));
                sockets.remove(&port);
                remaining -= 1;
            }
        }

        results
    }
}
