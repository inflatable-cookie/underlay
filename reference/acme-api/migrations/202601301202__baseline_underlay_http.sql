-- Underlay HTTP error logging schema (synced into app migrations).
-- Source: underlay/rust/crates/underlay-http/migrations/0001_create_error_log.sql

CREATE SCHEMA IF NOT EXISTS platform;

CREATE TABLE IF NOT EXISTS platform.error_log (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    occurred_at     TIMESTAMPTZ NOT NULL DEFAULT now(),

    -- HTTP request context
    endpoint        TEXT        NOT NULL,
    method          TEXT        NOT NULL,
    status_code     INTEGER     NOT NULL,

    -- Error details
    error_code      TEXT        NOT NULL,
    message         TEXT        NOT NULL,

    -- Request tracing
    correlation_id  TEXT        NOT NULL,

    -- Additional context (headers, body, etc.)
    context         JSONB       NOT NULL DEFAULT '{}'::jsonb
);

CREATE INDEX IF NOT EXISTS idx_platform_error_log_occurred_at
    ON platform.error_log (occurred_at DESC);

CREATE INDEX IF NOT EXISTS idx_platform_error_log_status_code
    ON platform.error_log (status_code);

CREATE INDEX IF NOT EXISTS idx_platform_error_log_error_code
    ON platform.error_log (error_code);

CREATE INDEX IF NOT EXISTS idx_platform_error_log_correlation_id
    ON platform.error_log (correlation_id);

CREATE INDEX IF NOT EXISTS idx_platform_error_log_endpoint
    ON platform.error_log (endpoint);
