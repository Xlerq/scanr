# scanr — roadmap

## Zakres produktu

- Linux-only, IPv4-only, TCP connect scanner bez uprawnień root.
- `UringEngine` jest silnikiem głównym; ograniczony `ThreadEngine` jest fallbackiem.
- Port i liczba skanowanych portów pozostają `u16`.
- Dozwolone porty: `1..=u16::MAX`. Port `0` jest odrzucany.
- `--speed` wybiera profil; użytkownik nie musi podawać timeoutu ani współbieżności.
- Priorytety: poprawność → brak paniców → UX → powtarzalne benchmarki → tuning.
- „Szybciej” oznacza krótszy czas przy identycznym wyniku, nie więcej false negatives.

## Stan obecny

- [x] CLI z subkomendami `scan` i `discover`.
- [x] Parser pojedynczych portów, list i zakresów.
- [x] Profile `fast`, `normal`, `thorough`.
- [x] Output `table`, `json`, `csv`.
- [x] Nazwy popularnych usług.
- [x] TCP host discovery; prawdziwe `ECONNREFUSED` oznacza żywy host.
- [x] Trait `ScanEngine` oraz `ThreadEngine`.
- [~] `UringEngine`: działa, ale nie ma obsługi błędów ani testów integracyjnych.
- [~] Automatyczny wybór silnika: sprawdza utworzenie ringa, ale nie obsługę `Connect` i `LinkTimeout`.
- [~] Współbieżność z `RLIMIT_NOFILE`: zmiana hard → soft nie jest jeszcze widoczna w tej gałęzi; nadal brakuje zapasu FD.
- [~] Benchmarki: istnieją wyniki eksperymentalne, ale odnoszą się do starego timeoutu i usuniętego `uring_spike`.

## P0 — poprawność i brak paniców

### IPv4-only

- [x] Zmień typ celu z `IpAddr` na `Ipv4Addr` w CLI, konfiguracji i silnikach.
- [ ] Odrzucaj IPv6 na granicy CLI, zanim powstanie `ScanConfig`.
- [ ] Dodaj test: `scan ::1 80` kończy się błędem użycia i kodem wyjścia `2`.
- [x] Usuń z roadmapy i kodu wszystkie plany obsługi IPv6.

### Invariant portów i `u16`

- [ ] `parse_port` odrzuca `0`; akceptuje `1` i `65535`.
- [ ] Zachowaj `u16` dla portów, `total_ports`, `remaining` i limitu współbieżności.
- [ ] Każdą konwersję długości do `u16` wykonuj przez `u16::try_from`, nigdy przez `as`.
- [ ] Zarezerwuj `user_data == 0` wyłącznie dla timeoutu.
- [ ] Dodaj testy: port `0`, pełny zakres `1-65535`, duplikaty i odwrócony zakres.

### Typowane błędy

- [ ] Zmień `ScanEngine::scan` na `Result<Vec<(u16, TcpResult)>, ScanError>`.
- [ ] Dodaj `ParseError` dla złego portu, zakresu i CIDR.
- [ ] Dodaj `ScanError` dla: ring init, unsupported opcode, socket, submit, resource limit, worker spawn/join i target unreachable.
- [ ] Dodaj `AppError`, który opakowuje `ParseError`, `ScanError` i błąd outputu.
- [ ] Usuń `unwrap`/`expect` z błędów zależnych od inputu lub OS; dopuszczaj `expect` tylko po sprawdzeniu invariantu i z konkretnym komunikatem.
- [ ] Usuń błędy jako `String` z prefiksem `"Error:"`; formatowanie błędu wykonuj raz w `main`.
- [ ] Użyj `std::io::Error::from_raw_os_error(-res).kind()` zamiast magicznego `-111`.
- [ ] Mapowanie wyników:
  - `0` → `Open`,
  - `ConnectionRefused` → `Closed`,
  - connect anulowany przez `LinkTimeout` → `Filtered`,
  - `HostUnreachable`/`NetworkUnreachable` → błąd celu,
  - pozostałe errno → `ScanError`, nie `Filtered`.
- [ ] `main` zwraca kod `0` dla sukcesu, `2` dla złego wejścia i `1` dla błędu wykonania.

### Limity zasobów

- [ ] Zsynchronizuj zmianę na soft `RLIMIT_NOFILE` (`get().0`).
- [ ] Odejmij stały zapas co najmniej 64 FD przed wyliczeniem współbieżności.
- [ ] Jeśli budżet FD jest za mały, zwróć czytelny błąd zamiast paniki.
- [ ] Nie przełączaj silnika po rozpoczęciu skanu; fallback jest dozwolony tylko przed pierwszym connectem.
- [ ] `ThreadEngine`: stała pula maksymalnie 512 workerów zamiast `cpu_count * 256` wątków.
- [ ] Użyj `thread::Builder::spawn`, aby błąd utworzenia workera zwracał `Result`.
- [ ] Dodaj test subprocessu z soft `RLIMIT_NOFILE=64`: skan kończy się wynikiem albo kontrolowanym błędem, nigdy paniką.

### Poprawny wybór `io_uring`

- [ ] Probe sprawdza opcodes `Connect` i `LinkTimeout`, nie tylko `IoUring::new(1)`.
- [ ] Utworzenie ringa o docelowej wielkości następuje przed wyborem silnika.
- [ ] Nieobsługiwany `io_uring` uruchamia `ThreadEngine` i jest widoczny w `--verbose`.
- [ ] Runtime error z `io_uring` jest raportowany; nie może zostać zamieniony na stan portu.

## P1 — clean Rust

### Granice modułów

- [ ] `args.rs`: wyłącznie surowe typy clap.
- [ ] `parser.rs`: walidacja wejścia i budowanie poprawnej konfiguracji.
- [ ] `config.rs`: tylko zwalidowane dane domenowe.
- [ ] `engine.rs`: `ScanEngine`, `TcpResult`, `ScanError`, wybór backendu.
- [ ] `threadengine.rs` i `uringengine.rs`: tylko implementacje backendów.
- [ ] `cli.rs`: orkiestracja i progress; bez logiki sieciowej.
- [ ] `output.rs`: wyłącznie serializacja i prezentacja.
- [ ] Przenieś logikę z binarki do `lib.rs`; `main.rs` ma tylko uruchomić aplikację i ustawić exit code.

### Zasady kodu

- [ ] Zamień `usize::MIN` na `0`.
- [ ] Popraw nazwy `concurreency`, `lateny` i pozostałe literówki.
- [ ] Przekazuj małe typy `Copy`, np. `u16`, przez wartość zamiast `&u16`.
- [ ] Nie zapisuj typów lokalnych, gdy kompilator jednoznacznie je inferuje.
- [ ] Każdy `unsafe` w `UringEngine` ma komentarz `SAFETY` opisujący lifetime socketów, adresów i timeoutu.
- [ ] Wydziel czystą funkcję mapującą CQE/`io::ErrorKind` na wynik; testuj ją bez sieci.
- [ ] Nie twórz na razie wrappera `Port`; walidacja `u16` na granicy CLI wystarcza.
- [ ] Usuń martwy `icmp.rs`, dopóki ICMP nie wróci do zakresu projektu.

### CI i toolchain

- [ ] Ustaw minimalną wersję Rust przez `rust-version` i `rust-toolchain.toml`.
- [ ] CI: `cargo fmt --check`.
- [ ] CI: `cargo clippy --all-targets --all-features -- -D warnings`.
- [ ] CI: `cargo test --all-targets`.
- [ ] Testy `UringEngine` pomijaj tylko wtedy, gdy kernel rzeczywiście nie obsługuje wymaganych opcode.

## P1 — testy poprawności

- [ ] Parser: granice `1`/`65535`, port `0`, pełny zakres, listy, duplikaty i błędy.
- [ ] `ThreadEngine`: lokalny IPv4 open i closed.
- [ ] `UringEngine`: ten sam lokalny zestaw open/closed.
- [ ] Oba silniki zwracają identyczne, posortowane wyniki dla tego samego celu.
- [ ] Timeout/DROP testuj w izolowanym network namespace z `nftables`; opóźnione `accept()` nie symuluje DROP.
- [ ] Discovery: tylko `Open` i `ConnectionRefused` oznaczają host alive.
- [ ] CLI: kody wyjścia `0/1/2`, JSON na stdout, błędy i progress na stderr.
- [ ] Żaden test automatyczny nie skanuje zewnętrznego hosta.

## P1 — UX CLI

- [ ] Popraw README: `scanr scan`, `scanr discover`, instalacja, Linux/IPv4-only i wymagana wersja Rust.
- [ ] Usuń niedziałające odwołania do `assets` albo dodaj rzeczywiste pliki.
- [ ] Dodaj `#[command(version, about)]` i konkretne opisy argumentów.
- [ ] Dokumentuj rzeczywiste wartości timeoutów profili.
- [ ] Sortuj porty i hosty przed każdym formatem outputu.
- [ ] Progress zapisuj na stderr; wynik na stdout.
- [ ] Dla discovery używaj etykiety `alive`, nie `open`.
- [ ] JSON kończy się newline i ma stabilny, przetestowany schemat.
- [ ] Dodaj `--verbose`: wybrany silnik, timeout, współbieżność i soft limit FD.
- [ ] Błąd zapisu/broken pipe nie może powodować panic backtrace.
- [ ] README zawiera krótką informację: skanuj wyłącznie systemy, na które masz zgodę.

## P2 — wiarygodne benchmarki

- [ ] Usuń wyniki odnoszące się do nieistniejącego `examples/uring_spike.rs`.
- [ ] Benchmarkuj aktualną binarkę `--release`, nie osobny prototyp.
- [ ] Zapisuj: commit, CPU, kernel, Rust, soft/hard NOFILE, engine, timeout, concurrency i target.
- [ ] Każdy wynik: minimum 20 przebiegów, mediana, min/max i odchylenie.
- [ ] Osobne scenariusze:
  - localhost/RST — narzut silnika,
  - kontrolowany DROP — timeout-bound,
  - znane open/closed/drop — poprawność,
  - LAN — zachowanie prawdziwego routera.
- [ ] Porównuj silniki przy identycznym timeout, współbieżności i oczekiwanym wyniku.
- [ ] Porównuj z `nmap -sT`; nie porównuj TCP connect scanera z raw SYN scannerem.
- [ ] Bramka dla claimu „1000 portów < 1 s”: mediana < 1 s na opisanym celu LAN i zero błędnie sklasyfikowanych portów.
- [ ] Claim o pełnym zakresie publikuj tylko z pełną komendą, warunkami i wynikiem poprawności.

## P2 — tuning po przejściu P0/P1

- [ ] Batchuj submit/drain CQE; ogranicz liczbę `submit_and_wait(1)`.
- [ ] Profiluj przed zmianą `HashMap`; optymalizuj wyłącznie potwierdzony hot path.
- [ ] Adaptywny timeout: próbka RTT, ograniczone minimum/maksimum i test false negatives.
- [ ] AIMD concurrency reaguje na resource errors i completion rate; zawsze respektuje soft NOFILE.
- [ ] Dodaj limit burst/rate, jeśli router gubi wyniki przy wysokiej współbieżności.
- [ ] Zachowaj profile jako prosty interfejs; parametry techniczne pokazuj w `--verbose`.

## Discovery — zamrożony zakres

- [ ] Użyj tego samego klasyfikatora błędów co skan portów.
- [ ] Zastąp tysiące wątków tą samą ograniczoną pulą workerów co `ThreadEngine`.
- [ ] Zachowaj wejściowy CIDR w `DiscoverConfig` i dodaj go do JSON/table.
- [ ] Ostrzegaj na stderr przy zakresie większym niż `/24`; bez interaktywnego potwierdzenia.
- [ ] Nie dodawaj ICMP ani ARP przed ukończeniem P0/P1.

## Poza zakresem

- IPv6.
- macOS, Windows i `MioEngine`.
- Raw SYN scan wymagający root/capabilities.
- ICMP, ARP i wykrywanie middleboxa przed stabilizacją TCP.
- TUI/ratatui przed stabilnym CLI.
- Hostname/DNS przed stabilnym skanem pojedynczego IPv4.

## Definition of done dla następnego wydania

- [ ] Brak `unwrap`/`expect` na błędach osiągalnych z inputu lub OS; każdy pozostały `expect` dokumentuje sprawdzony invariant.
- [ ] Brak paniców przy złym wejściu, niskim NOFILE i niedostępnym `io_uring`.
- [ ] Thread i Uring mają identyczną semantykę wyników.
- [ ] IPv6 jest odrzucane przez CLI.
- [ ] Port `0` jest odrzucany; `1-65535` działa.
- [ ] CI przechodzi fmt, clippy i wszystkie testy.
- [ ] README, `--help`, benchmarki i kod opisują te same zachowania.
