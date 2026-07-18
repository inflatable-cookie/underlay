-- Example migration for the Postgres rate-limit backend.
-- Copy this into your app migration set and adjust schema/table names as
-- needed (pass the name to PostgresBackend::with_table).

CREATE TABLE IF NOT EXISTS auth.rate_limit_counters (
    key TEXT PRIMARY KEY,
    window_started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    count BIGINT NOT NULL DEFAULT 0
);

-- Optional maintenance: expired rows are reused in place by the upsert, but a
-- periodic sweep keeps the table small.
-- DELETE FROM auth.rate_limit_counters WHERE window_started_at < NOW() - INTERVAL '1 day';
