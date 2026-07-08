# Benchmarks (before/after)

Maszyna: 8 rdzeni, `ulimit -n` = 524288. Build: `--release`. Narzędzie: hyperfine.

## Before — model wątków (`cpu*256` = 2048 wątków)

| scenariusz | komenda | wynik |
|---|---|---|
| silnik (localhost, brak timeoutów, RST natychmiast) | `scanr scan 127.0.0.1 1-65535 --speed fast` | ~0.32 s (zgrubnie, ±0.02) |
| timeout-bound (router, porty filtrowane) | `scanr scan 192.168.0.1 1-65535 --speed fast` | **3.262 s ± 0.004 s** (20 runs; User 0.161 s, Sys 1.427 s) |

Wniosek: silnik jest sub-sekundowy. Czas realnego skanu to niemal w całości **timeouty** (~32 rundy × 100 ms przy 2048 współbieżności). io_uring ma pobić scenariusz timeout-bound przez wyższą współbieżność (mniej rund), nie przez szybszy silnik.

## After — io_uring vs model wątków (sweep współbieżności)

Ta sama maszyna (8 rdzeni, `ulimit -n` = 524288, `ulimit -u` = 47355), build `--release`.
Cel: `192.168.0.1 1-65535` (scenariusz timeout-bound, router dropuje zamknięte).
**Stały timeout 500 ms** (`--speed normal`), self-timer binarek, pojedyncze przebiegi (wariancja baseline ±0.004 s przy 20 przebiegach — pojedyncze wystarczają).

io_uring: `examples/uring_spike.rs`, stała `CONCURRENCY`. Wątki: `chunks.rs`, `cpu_count * N`.

| współbieżność `W` | model wątków | io_uring | generacje `ceil(65535/W)` |
|---|---|---|---|
| 2048 | 16.08 s | 16.11 s | 32 |
| 8192 | 4.28 s | 4.26 s | 8 |
| 16384 | **crash — `EAGAIN`** | 2.47 s | 4 |

### Ustalenia

1. **Mechanizm silnika jest neutralny.** Przy równej współbieżności wątki i io_uring dają identyczny czas (2048: 16.08 vs 16.11; 8192: 4.28 vs 4.26). Skan timeout-bound nie obciąża CPU (baseline: user 0.16 s / sys 1.4 s na 16 s zegara) — oba silniki tylko czekają na timery. Czas ≈ `ceil(porty/W) × timeout`, niezależnie od silnika.

2. **Model wątków uderza w twardy sufit ~14206 zadań.** Przy `W=16384` wątki panikują: `EAGAIN: failed to spawn thread`. Przyczyna to cgroup sesji systemd `pids.max = 14206` (`DefaultTasksMax`) — **nie** `ulimit -n` (524288) ani `ulimit -u` (47355). Każde połączenie potrzebuje żywego wątku OS; sesja ogranicza zadania do ~14k. io_uring utrzymuje 16384 (i więcej) połączeń na **jednym** wątku, ~0 zadań na połączenie.

3. **Wartość io_uring = tania współbieżność, i nic poza tym.** To nie „szybszy silnik" — to zasięg współbieżności, którego model wątków nie przeżywa. Bezpiecznie skonfigurowane wątki kończą się ~8192 (4.3 s); io_uring robi 16384 (2.5 s) z zapasem do sufitu portów efemerycznych (~28k lokalnych portów).

4. **Największa dźwignia to timeout — i jest engine-agnostyczna.** Na tym LAN-ie RTT ≈ 0.3 ms, a timeout = 500 ms (~1600× RTT). io_uring@16384 przy 100 ms = **0.97 s** (vs 2.47 s przy 500 ms). Adaptywny timeout z RTT przyspieszyłby **oba** silniki jednakowo i przenośnie — prawdopodobnie większy zysk niż zmiana silnika.

### Bramka KILL (roadmap: „UringEngine musi pobić baseline 3.26 s")

Baseline 3.26 s zmierzono przy 100 ms / 2048 (stary preset `fast`). **io_uring@16384@100ms = 0.97 s → przechodzi ~3.4×.** UringEngine zalicza bramkę.
