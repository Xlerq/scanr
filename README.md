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

## Build

```bash
cargo build --release
```

## License
MIT
