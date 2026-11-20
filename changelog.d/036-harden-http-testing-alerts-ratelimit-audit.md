### Fixed

- `underlay-http`: `ApiError::with_context` now merges JSON object contexts instead of overwriting existing keys, so `internal_db_error` preserves both `cause` and `diagnostic`.
- `underlay-http`: `extract_refresh_token`, `extract_csrf_token`, `if_none_match_matches`, and `if_match_matches` now iterate all header values via `headers.get_all(...)` for HTTP/2 multi-header support.
- `underlay-http`: `HttpServerConfig::from_env` and `try_from_env` now default `public_host` to `"localhost"` in local dev when `HOST` and `PUBLIC_HOST` are unset (including when `PORT` is set).
- `underlay-http-client`: `HttpClient::external()` now returns an error (rather than a silent 3xx response) on blocked redirects and installs an async `reqwest::dns::Resolve` implementation that validates resolved IPs at connect time.
- `underlay-testing`: `Fixtures::email`, `username`, and `password` now use the random suffix of UUIDv7 instead of the colliding 32-bit timestamp prefix; `TestDb::load_fixture` and `run_migrations` execute multi-statement SQL via `sqlx::raw_sql`; `TestServer`'s `RequestBuilder` stores headers in a `HeaderMap` for multi-value, case-insensitive header support.
- `underlay-security-alerts`: replaced panicking `row.get(...)` calls in `store.rs` with `row.try_get(...)?` and added atomic transaction-advisory-locked cooldown deduplication helpers (`insert_alert_event_if_not_in_cooldown` and `insert_scoped_alert_event_if_not_in_cooldown`).
- `underlay-ratelimit`: `PostgresBackend::check_and_increment` stops inflating the counter once at `max_requests()` (matching `InMemoryBackend`) and propagates `elapsed` decode errors; `InMemoryBackend` updates `window_duration` when config changes and caps tracked keys (`DEFAULT_MAX_KEYS` / `with_max_keys`) against unbounded key-churn memory growth.
- `underlay-audit`: `AuditLogFilters::with_pagination` (and `list_audit_logs_from_table`) now clamp `limit` to `1..=MAX_AUDIT_LOG_LIMIT` and `offset` to `>= 0`.
