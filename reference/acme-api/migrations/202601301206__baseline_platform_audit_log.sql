-- Acme baseline: audit logging.
--
-- Farmyard reference keeps audit log in the platform schema.

CREATE SCHEMA IF NOT EXISTS platform;

CREATE TABLE IF NOT EXISTS platform.audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id UUID NULL REFERENCES auth.users(id),
    action TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id UUID NOT NULL,
    details JSONB NOT NULL DEFAULT '{}',
    correlation_id TEXT,
    ip_address TEXT
);

CREATE INDEX IF NOT EXISTS idx_platform_audit_log_occurred_at
    ON platform.audit_log (occurred_at DESC);

CREATE INDEX IF NOT EXISTS idx_platform_audit_log_user_id
    ON platform.audit_log (user_id)
    WHERE user_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_platform_audit_log_resource
    ON platform.audit_log (resource_type, resource_id);

CREATE INDEX IF NOT EXISTS idx_platform_audit_log_action
    ON platform.audit_log (action);
