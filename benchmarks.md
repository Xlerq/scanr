# Benchmarks (before/after)

Maszyna: 8 rdzeni, `ulimit -n` = 524288. Build: `--release`. Narzędzie: hyperfine.

## Before — model wątków (`cpu*256` = 2048 wątków)

| scenariusz | komenda | wynik |
|---|---|---|
| silnik (localhost, brak timeoutów, RST natychmiast) | `scanr scan 127.0.0.1 1-65535 --speed fast` | ~0.32 s (zgrubnie, ±0.02) |
| timeout-bound (router, porty filtrowane) | `scanr scan 192.168.0.1 1-65535 --speed fast` | **3.262 s ± 0.004 s** (20 runs; User 0.161 s, Sys 1.427 s) |

Wniosek: silnik jest sub-sekundowy. Czas realnego skanu to niemal w całości **timeouty** (~32 rundy × 100 ms przy 2048 współbieżności). io_uring ma pobić scenariusz timeout-bound przez wyższą współbieżność (mniej rund), nie przez szybszy silnik.
