```                                                                                                                                          
                        █████   ██████   ██████   ████████   ████████ 
                       ███▒▒   ███▒▒███ ▒▒▒▒▒███ ▒▒███▒▒███ ▒▒███▒▒███
                      ▒▒█████ ▒███ ▒▒▒   ███████  ▒███ ▒███  ▒███ ▒▒▒ 
                       ▒▒▒▒███▒███  ███ ███▒▒███  ▒███ ▒███  ▒███     
                       ██████ ▒▒██████ ▒▒████████ ████ █████ █████    
                      ▒▒▒▒▒▒   ▒▒▒▒▒▒   ▒▒▒▒▒▒▒▒ ▒▒▒▒ ▒▒▒▒▒ ▒▒▒▒▒ 
```
Fast TCP connect scanner for Linux, written in Rust. It scans IPv4 targets without
root privileges and automatically selects `io_uring` or the thread fallback.

## Usage

```bash
scanr scan <IPV4> <PORTS> [OPTIONS]
scanr discover <CIDR> [OPTIONS]
```

`<ports>` accepts a single port, a range, a comma-separated list, or a mix of lists and ranges.

## Examples

```bash
scanr scan 192.168.0.1 1-65535 --speed thorough
scanr scan 192.168.0.1 22,80,443 --format json
scanr discover 192.168.0.1/24
scanr discover 192.168.0.1/24 --speed fast
```

## Port input

- `80`: scan one port
- `100-1000`: scan a port range
- `22,80,443`: scan selected ports
- `22,80,100-120`: scan selected ports and ranges
- `"22, 80, 443"`: spaces are allowed when the whole port expression is quoted
- `"20 - 25"`: spaces are allowed in ranges when the expression is quoted

## Output formats

- `table`: human-readable output with progress bar
- `csv`: comma-separated output for scripts or files
- `json`: JSON output for tools

Use `--format <FORMAT>` to select an output format. The default is `table`.

## Speed modes

- `fast`: 250 ms timeout for LAN scans
- `normal`: 500 ms timeout (default)
- `thorough`: 1000 ms timeout for higher-latency networks or VPNs

Use `--speed <SPEED>` to select a speed preset. The default is `normal`.

## File descriptor limit

Before an `io_uring` scan, `scanr` tries to raise its own soft `RLIMIT_NOFILE` to
the value needed by the scan, up to the existing hard limit. This change affects
only the running `scanr` process: it does not modify the shell, system configuration,
or hard limit, and it does not require root.

## Example result

The run below scanned all 65,535 ports of the author's LAN router with
`UringEngine` in 1.54 seconds:

```bash
cargo run --release -- scan 192.168.0.1 1-65535
```

This screenshot is an example, not a portable benchmark. Scan time depends on the
target, network behavior, kernel, timeout profile, and file descriptor limits.

<p align="center">
  <img src="./assets/Scan_full_range.png" alt="Scan of 65535 ports" width="65%">
</p>

`scanr` can also discover hosts with TCP probes.

<p align="center">
  <img src="./assets/Discover.png" alt="Discover of alive hosts" width="65%">
</p>

## Build

```bash
cargo build --release
```

The binary is created at `target/release/scanr`.

Only scan systems that you own or have explicit permission to test.

## License
MIT
