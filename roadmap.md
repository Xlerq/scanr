# scanr — roadmap

Linux-only, szybki skaner TCP. Silnik na io-uring, sam się stroi (user nie podaje ms).

Zrobione:
- [x] ScanSpeed enum
- [x] port parser (22,80,443 i 1-1000)
- [x] clap
- [x] output: table/json/csv
- [x] host discovery TCP (RST = host żyje)
- [x] refaktor models.rs → args.rs + config.rs
- [x] benchmark hyperfine (baseline before) — benchmarks.md, sekcja "Before"
- [x] service name mapping (port → usługa) — output.rs COMMON_PORTS, wpięte w table/csv/json

Do zrobienia:
- [ ] zamień cpu*252 na limit z ulimit (RLIMIT_NOFILE - zapas)
- [ ] silnik skanu na io-uring
- [ ] adaptywny timeout z RTT (zamiast stałych presetów)
- [ ] współbieżność AIMD 
- [~] output: open/closed/filtered zamiast open/down
- [ ] wykrywanie kłamiącego middleboxa (probe na martwy adres = kontrola)
- [ ] ARP discovery on-link (autorytatywne, omija middlebox; raw socket/root, fallback TCP)
- [~] ICMP echo off-link, łączone z TCP (opcjonalne)
- [ ] ratatui (opcjonalne, na końcu)

## Silnik za traitem (io_uring Linux + mio cross-platform)

- [ ] `trait ScanEngine { fn scan(ip, ports, timeout) -> Vec<TcpResult> }`; thread-engine zostaje jako fallback
- [ ] `UringEngine` (Linux): Connect SQE + LinkTimeout; CQE 0=open, -ECONNREFUSED=closed, timeout=filtered
- [x] (zmierzone): io_uring@16384@100ms = 0.97 s vs baseline 3.26 s → przechodzi ~3.4×. Silnik neutralny przy równej współbieżności; zysk io_uring = tania współbieżność (wątki padają na cgroup `pids.max`=14206). Priorytet 1 to adaptywny timeout (engine-agnostyczny), potem podbicie wątków, potem UringEngine. Szczegóły: `benchmarks.md` sekcja "After".
- [ ] `MioEngine` (epoll/kqueue/IOCP): non-blocking connect → writable → SO_ERROR
- [ ] dispatch: Linux probuje io_uring (kernel ≥5.5) → Uring, else Mio; mac/Win zawsze Mio
