### Changed

- `underlay-http-client`: `HttpClient::external()` now returns a dedicated
  `ExternalHttpClient` that does not implement `Deref<Target = reqwest::Client>`
  or expose `inner()` / `into_inner()`. Its request helpers (`request`, `get`,
  `post`, `put`, `patch`, `delete`, `head`, `execute`) validate `http`/`https`
  schemes and reject non-public IP literals before any network I/O, closing the
  IP-literal SSRF bypass where `reqwest` skips custom DNS resolvers.
- `underlay-ratelimit`: `PostgresBackend::check` now returns `allowed: false`
  when `max_requests == 0`, matching `InMemoryBackend::check` and
  `PostgresBackend::check_and_increment`.
- `underlay-ratelimit`: `InMemoryBackend::evict_if_full` now uses bounded
  sampled eviction (32 entries per pass) instead of an O(N) full-map `retain`
  scan on every insertion at capacity, and `InMemoryBackend::with_cleanup`
  clamps its interval to at least `MIN_CLEANUP_INTERVAL` (10ms) to prevent
  busy-spinning on `Duration::ZERO`.
- `underlay-http`: `CoalescedErrorResponse` now preserves `ErrorDetail`
  response extensions across single-flight error coalescing and bounds buffered
  error bodies to 64 KiB (clearing stale `Content-Length` if exceeded).
- `underlay-http`: `list_error_logs` clamps `ErrorLogFilters` `limit` to
  `1..=MAX_ERROR_LOG_LIMIT` (1,000) and `offset` to `>= 0`; `redact_query`
  percent-decodes parameter keys before matching `SENSITIVE_QUERY_KEYS`.
- `underlay-http`: `QueryParams::filter_fields()` now returns filters in a
  deterministic key order (`BTreeMap`), and `PagePaginationParams::wrap`
  computes `total_pages` with exact integer `div_ceil`.
- `underlay-audit`: `append_audit_log_to_table_async` now bounds concurrent
  background writes with a semaphore (`MAX_CONCURRENT_ASYNC_AUDIT_WRITES = 64`),
  dropping excess writes with a warning when saturated.
- `underlay-security-alerts`: `evaluate_alerts`, `evaluate_account_alerts`, and
  `evaluate_global_alerts` now ignore non-positive thresholds (`<= 0`) instead
  of triggering `0 >= 0` false positives.
- `underlay-testing`: `TestDb::new` runs Docker availability checks and
  `testcontainers` blocking startup on `tokio::task::spawn_blocking`, and stale
  `.await` doc examples in `lib.rs` and `test_server.rs` are fixed.

### Consumer Upgrade Impact (breaking; target `0.11.0`)

- Callers of `HttpClient::external()` now receive `ExternalHttpClient` instead
  of `HttpClient`. Replace `client.get(url)` calls with `client.get(url)?` (or
  the matching HTTP verb / `execute` helper) to handle SSRF validation errors.
- Validation: `effigy rust:clippy`, `effigy fmt:rust:check`, and the Part 2
  crate test suites.
- Rollback: pin the previous tag; no migration or data change is involved.
