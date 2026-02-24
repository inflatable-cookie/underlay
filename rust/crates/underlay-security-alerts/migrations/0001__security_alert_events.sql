-- Example migration for login security alerts.
-- Copy this into your app migration set and adjust schema/table names as needed.

CREATE TABLE IF NOT EXISTS auth.security_alert_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    alert_type TEXT NOT NULL,
    ip_address INET NOT NULL,
    window_started_at TIMESTAMPTZ NOT NULL,
    window_ended_at TIMESTAMPTZ NOT NULL,
    failed_attempts BIGINT NOT NULL DEFAULT 0,
    distinct_user_count BIGINT NOT NULL DEFAULT 0,
    lockout_count BIGINT NOT NULL DEFAULT 0,
    details JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE INDEX IF NOT EXISTS idx_auth_security_alert_events_created_at
    ON auth.security_alert_events (created_at DESC);

CREATE INDEX IF NOT EXISTS idx_auth_security_alert_events_lookup
    ON auth.security_alert_events (alert_type, ip_address, created_at DESC);
