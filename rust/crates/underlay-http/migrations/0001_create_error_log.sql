-- Underlay HTTP: Error logging table
--
-- This migration defines persistent error logging for HTTP requests.
-- Applications should sync this migration into their own `migrations/` folder
-- (via `underlay-devtools sync-migrations`) and run a single sqlx migrator.
--
-- The error log captures failed HTTP requests with full context for debugging
-- and monitoring. Intended for 5xx errors by default, optionally 4xx.

CREATE SCHEMA IF NOT EXISTS infra;

-- =========================================
-- Error Log
-- =========================================

CREATE TABLE IF NOT EXISTS infra.error_log (
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

-- Index for time-based queries (most recent errors)
CREATE INDEX IF NOT EXISTS error_log_occurred_at_idx
    ON infra.error_log (occurred_at DESC);

-- Index for filtering by status code (500, 502, 503, etc.)
CREATE INDEX IF NOT EXISTS error_log_status_code_idx
    ON infra.error_log (status_code);

-- Index for filtering by error code (application-specific codes)
CREATE INDEX IF NOT EXISTS error_log_error_code_idx
    ON infra.error_log (error_code);

-- Index for tracing requests across services
CREATE INDEX IF NOT EXISTS error_log_correlation_id_idx
    ON infra.error_log (correlation_id);

-- Index for endpoint-specific error queries
CREATE INDEX IF NOT EXISTS error_log_endpoint_idx
    ON infra.error_log (endpoint);
