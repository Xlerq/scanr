# scanr — roadmap

## Cel produktu

- Szybki TCP connect scanner dla Linuksa, IPv4-only, bez uprawnień root.
- Prosty interfejs dla osób nietechnicznych: cel, porty, `--speed` i format wyniku.
- `UringEngine` jest głównym backendem; ograniczony `ThreadEngine` jest fallbackiem.
- Poprawność wyniku ma pierwszeństwo przed szybkością.
- Porty i ich liczba pozostają `u16`; dozwolony zakres to `1..=65535`.

## Zamrożone decyzje

- Brak publicznych opcji wyboru backendu, timeoutu i współbieżności.
- Fallback jest dozwolony tylko przed rozpoczęciem skanu, gdy wymagane `io_uring`
  nie jest dostępne.
- Błędy zasobów i błędy działającego backendu kończą skan; nie zmieniają backendu
  ani stanu portu.
- `HostUnreachable` i `NetworkUnreachable` pozostają osobnymi błędami celu.
- Stdout zawiera wynik; stderr zawiera progress, ostrzeżenia i diagnostykę.
- IPv6, inne systemy i nowe metody discovery są poza następnym wydaniem.

## Stan obecny

- [x] Komendy `scan` i `discover`.
- [x] Porty pojedyncze, listy i zakresy.
- [x] Profile `fast`, `normal`, `thorough`.
- [x] Format `table`, `json`, `csv` i nazwy usług.
- [x] Automatyczny wybór `UringEngine` lub `ThreadEngine`.
- [x] `UringEngine` podnosi soft `RLIMIT_NOFILE` dla własnego procesu, jeśli
  pozwala na to hard limit.
- [x] Aktualny screenshot pełnego skanu przez `UringEngine`.
- [~] Backendy działają, ale nie mają jeszcze wspólnej semantyki błędów.
- [~] Wybór `io_uring` nie sprawdza jeszcze wymaganych opcode.
- [~] Discovery działa, ale wymaga poprawienia klasyfikacji błędów i puli workerów.
- [ ] Port `0` nadal nie jest odrzucany i może zawiesić `UringEngine`.

## P0 — poprawność i brak paniców

### Wejście

- [ ] Odrzuć port `0`; obsłuż poprawnie pełny zakres `1-65535`.
- [ ] Dodaj typowane błędy portu, zakresu i CIDR.
- [ ] Zapewnij kod wyjścia `2` dla błędnego wejścia, w tym IPv6.
- [ ] Usuń niejawne i zawijające konwersje liczby portów.

### Wyniki i błędy

- [ ] Zmień silniki na `Result<_, ScanError>` i dodaj nadrzędny `AppError`.
- [ ] Ujednolić klasyfikację TCP dla obu backendów i discovery.
- [ ] Rozróżniaj `Open`, `Closed`, `Filtered` oraz błędy celu i wykonania.
- [ ] Usuń paniki osiągalne przez input, system operacyjny lub zapis outputu.
- [ ] Zwracaj kod `1` dla błędu wykonania bez publikowania częściowego wyniku.

### Zasoby i backendy

- [x] Podnoś tylko soft `RLIMIT_NOFILE` procesu; nie zmieniaj hard limitu.
- [ ] Zwróć kontrolowany błąd, gdy budżet FD jest zbyt mały.
- [ ] Zastąp masowe tworzenie wątków stałą pulą maksymalnie 512 workerów.
- [ ] Sprawdzaj `Connect`, `LinkTimeout` i docelowy rozmiar ringa przed wyborem
  `UringEngine`.
- [ ] Raportuj błędy runtime `io_uring` zamiast klasyfikować je jako port.

### CLI

- [ ] Utrzymuj progress na stderr i wynik na stdout dla każdego formatu.
- [ ] Dodaj `--verbose` z backendem, timeoutem, współbieżnością i soft limitem FD.
- [ ] Obsłuż broken pipe bez paniki i backtrace.
- [ ] Sortuj wyniki i ustabilizuj schemat JSON/CSV.

## P1 — czysty kod i testy

- [ ] Rozdziel surowe argumenty, parser, konfigurację, silniki, CLI i output.
- [ ] Przenieś aplikację do `lib.rs`; pozostaw w `main.rs` tylko exit code.
- [ ] Usuń martwy kod, literówki i nieuzasadnione `unsafe`.
- [ ] Dodaj testy parsera, CLI, obu backendów i ich identycznych wyników.
- [ ] Testuj timeout/DROP w izolowanym network namespace; bez zewnętrznych hostów.
- [ ] Dodaj CI dla fmt, Clippy i wszystkich testów oraz ustal wersję Rust.
- [ ] Uzgodnij README, `--help`, screenshoty i zachowanie programu.

## Discovery — zakres zamrożony

- [ ] Użyj wspólnego klasyfikatora TCP i ograniczonej puli workerów.
- [ ] Zachowaj wejściowy CIDR w wyniku.
- [ ] Ostrzegaj przy zakresie większym niż `/24`.
- [ ] Nie dodawaj ICMP ani ARP przed ukończeniem P0/P1.

## P2 — benchmarki i tuning

- [ ] Benchmarkuj aktualną binarkę `--release` w kontrolowanych scenariuszach.
- [ ] Porównuj backendy i `nmap -sT` przy tych samych warunkach i wyniku.
- [ ] Publikuj medianę co najmniej 20 przebiegów wraz ze środowiskiem testu.
- [ ] Claim „1000 portów < 1 s” wymaga mediany poniżej 1 s i braku błędów.
- [ ] Dopiero po profilowaniu rozważ batching CQE, adaptacyjny timeout i AIMD.

## Poza zakresem

- IPv6, macOS, Windows i `MioEngine`.
- Raw SYN scan, ICMP i ARP.
- Hostname/DNS, TUI i publiczne strojenie backendu.

## Kolejność najbliższych prac

1. Odrzucenie portu `0` i testy granic parsera.
2. `ParseError`, `ScanError`, `AppError` i kody wyjścia.
3. Wspólny klasyfikator wyników TCP.
4. Kontrolowane limity zasobów i stała pula workerów.
5. Pełny probe oraz testy integracyjne `io_uring`.
6. Porządek modułów, CI, dokumentacja i dopiero potem benchmarki.

Jednocześnie realizujemy jeden mały krok, możliwy do zamknięcia wraz z testami w
około godzinę. Szczegóły implementacji należą do rozmowy i testów, nie do roadmapy.

## Definition of Done następnego wydania

- [ ] Brak paniców i zawieszeń dla złego inputu, niskiego NOFILE i braku `io_uring`.
- [ ] Port `0` i IPv6 są odrzucane; pełny zakres działa.
- [ ] Oba backendy mają identyczną semantykę i posortowane wyniki.
- [ ] Fallback zachodzi wyłącznie przed pierwszym connectem.
- [ ] Domyślne CLI nie wymaga wiedzy o backendzie ani zasobach systemowych.
- [ ] CI, README, `--help` i wszystkie testy są zgodne z wydaniem.
