# scanr — roadmap

Linux-only, szybki skaner TCP. Silnik na io-uring, sam się stroi (user nie podaje ms).

Zrobione:
- [x] ScanSpeed enum
- [x] port parser (22,80,443 i 1-1000)
- [x] clap
- [x] output: table/json/csv
- [x] host discovery TCP (RST = host żyje)
- [x] refaktor models.rs → args.rs + config.rs

Do zrobienia:
- [ ] zamień cpu*252 na limit z ulimit (RLIMIT_NOFILE - zapas)
- [ ] benchmark hyperfine (baseline before)
- [ ] silnik skanu na io-uring
- [ ] adaptywny timeout z RTT (zamiast stałych presetów)
- [ ] współbieżność AIMD (rozkręcaj, cofaj przy stratach)
- [ ] output: open/closed/filtered zamiast open/down
- [ ] wykrywanie kłamiącego middleboxa (probe na martwy adres = kontrola)
- [ ] ARP discovery on-link (autorytatywne, omija middlebox; raw socket/root, fallback TCP)
- [ ] ICMP echo off-link, łączone z TCP (opcjonalne)
- [ ] service name mapping (port → usługa)
- [ ] ratatui (opcjonalne, na końcu)
