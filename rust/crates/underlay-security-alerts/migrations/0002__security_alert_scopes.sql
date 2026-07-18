-- Example migration for scoped (account/global) login security alerts.
-- Copy this into your app migration set and adjust schema/table names as needed.
--
-- Adds a generic scope key so alerts can be deduped per account or globally,
-- not only per IP, and relaxes ip_address for alerts that have no single IP.

ALTER TABLE auth.security_alert_events
    ALTER COLUMN ip_address DROP NOT NULL;

ALTER TABLE auth.security_alert_events
    ADD COLUMN IF NOT EXISTS scope_key TEXT NOT NULL DEFAULT '';

UPDATE auth.security_alert_events
    SET scope_key = host(ip_address)
    WHERE scope_key = '' AND ip_address IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_auth_security_alert_events_scope_lookup
    ON auth.security_alert_events (alert_type, scope_key, created_at DESC);
