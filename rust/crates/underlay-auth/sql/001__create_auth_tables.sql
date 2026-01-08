-- Underlay: Auth tables (template)
--
-- Applications should copy this file into their migration directory.
-- Rename with timestamp prefix: YYYYMMDDHHMMSS__create_auth_tables.sql
--
-- This schema is intentionally generic. Apps can add columns as needed.

CREATE SCHEMA IF NOT EXISTS auth;

-- Users table: stores user accounts
CREATE TABLE IF NOT EXISTS auth.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    display_name VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'suspended', 'deleted')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Credentials table: stores authentication methods
CREATE TABLE IF NOT EXISTS auth.credentials (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    type VARCHAR(50) NOT NULL CHECK (type IN ('password', 'totp', 'passkey', 'oauth_google')),
    secret_encrypted TEXT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}',
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_auth_credentials_user_id ON auth.credentials(user_id);
CREATE INDEX IF NOT EXISTS idx_auth_credentials_type ON auth.credentials(type);

-- Sessions table: stores active sessions
CREATE TABLE IF NOT EXISTS auth.sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    access_token_fingerprint VARCHAR(64) NOT NULL,
    refresh_token_fingerprint VARCHAR(64) NOT NULL,
    access_token_expires_at TIMESTAMPTZ NOT NULL,
    refresh_token_expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ip_address INET,
    user_agent TEXT,
    revoked BOOLEAN NOT NULL DEFAULT FALSE,
    revocation_reason VARCHAR(100),
    revoked_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_auth_sessions_user_id ON auth.sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_auth_sessions_access_fingerprint ON auth.sessions(access_token_fingerprint);
CREATE INDEX IF NOT EXISTS idx_auth_sessions_refresh_fingerprint ON auth.sessions(refresh_token_fingerprint);
CREATE INDEX IF NOT EXISTS idx_auth_sessions_expires_at ON auth.sessions(access_token_expires_at);

-- Audit log table: stores auth events for security review
CREATE TABLE IF NOT EXISTS auth.audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type VARCHAR(100) NOT NULL,
    user_id UUID REFERENCES auth.users(id) ON DELETE SET NULL,
    session_id UUID REFERENCES auth.sessions(id) ON DELETE SET NULL,
    ip_address INET,
    user_agent TEXT,
    success BOOLEAN NOT NULL,
    details JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_auth_audit_log_user_id ON auth.audit_log(user_id);
CREATE INDEX IF NOT EXISTS idx_auth_audit_log_event_type ON auth.audit_log(event_type);
CREATE INDEX IF NOT EXISTS idx_auth_audit_log_created_at ON auth.audit_log(created_at);
CREATE INDEX IF NOT EXISTS idx_auth_audit_log_ip_address ON auth.audit_log(ip_address);

-- Rate limiting table: prevents brute force attacks
CREATE TABLE IF NOT EXISTS auth.rate_limits (
    key VARCHAR(255) PRIMARY KEY,
    count INTEGER NOT NULL DEFAULT 0,
    window_start TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_auth_rate_limits_expires_at ON auth.rate_limits(expires_at);

-- Updated at trigger function
CREATE OR REPLACE FUNCTION auth.update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply updated_at trigger to tables
CREATE TRIGGER update_auth_users_updated_at
    BEFORE UPDATE ON auth.users
    FOR EACH ROW
    EXECUTE FUNCTION auth.update_updated_at_column();

CREATE TRIGGER update_auth_credentials_updated_at
    BEFORE UPDATE ON auth.credentials
    FOR EACH ROW
    EXECUTE FUNCTION auth.update_updated_at_column();
