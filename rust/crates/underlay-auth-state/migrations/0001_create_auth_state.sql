-- Canonical Underlay auth-state table (shared across projects).
--
-- Stores short-lived state for multi-step auth flows (2FA login, passkeys, oauth callbacks).

CREATE SCHEMA IF NOT EXISTS accounts;

CREATE TABLE IF NOT EXISTS accounts.auth_state (
    id UUID PRIMARY KEY,
    user_id UUID NULL,
    state_type TEXT NOT NULL,
    state JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_accounts_auth_state_user ON accounts.auth_state (user_id);
CREATE INDEX IF NOT EXISTS idx_accounts_auth_state_expires ON accounts.auth_state (expires_at);
CREATE INDEX IF NOT EXISTS idx_accounts_auth_state_type ON accounts.auth_state (state_type);

COMMENT ON TABLE accounts.auth_state IS 'Short-lived auth state for setup/login flows.';
