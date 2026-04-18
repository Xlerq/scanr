# scanr

Minimal CLI port scanner written in Rust.

## Usage

```bash
scanr <ip> <start_port> [end_port]
```

## Examples

```bash
scanr 192.168.0.10 100 1000
scanr 127.0.0.1 80
```
## Capabilities

Scanner is able to scan 1k ports below 1s.
![Scan 1k](./assets/1000_ports.png)
10k ports in 8s.
![Scan 10k](./assets/10000_ports.png)


## Build

```bash
cargo build --release
```

## License
MIT
