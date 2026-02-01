-- Add missing auth columns and tables for full auth support.

-- Add missing columns to auth.users
ALTER TABLE auth.users
    ADD COLUMN IF NOT EXISTS display_name TEXT,
    ADD COLUMN IF NOT EXISTS failed_login_count INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS lockout_until TIMESTAMPTZ NULL;

CREATE INDEX IF NOT EXISTS idx_auth_users_lockout
    ON auth.users (lockout_until)
    WHERE lockout_until IS NOT NULL;

-- Login attempts tracking
CREATE TABLE IF NOT EXISTS auth.login_attempts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    ip_address INET NULL,
    attempted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    success BOOLEAN NOT NULL DEFAULT FALSE,
    failure_reason TEXT NULL CHECK (failure_reason IS NULL OR char_length(failure_reason) <= 128)
);

CREATE INDEX IF NOT EXISTS idx_auth_login_attempts_user_id
    ON auth.login_attempts (user_id, attempted_at DESC);

CREATE INDEX IF NOT EXISTS idx_auth_login_attempts_ip
    ON auth.login_attempts (ip_address, attempted_at DESC)
    WHERE ip_address IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_auth_login_attempts_user_failures
    ON auth.login_attempts (user_id, attempted_at DESC)
    WHERE success = FALSE;

-- Email TOTP codes
CREATE TABLE IF NOT EXISTS auth.email_totp_codes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    email TEXT NOT NULL,
    code_hash TEXT NOT NULL,
    purpose TEXT NOT NULL CHECK (purpose IN ('login', 'password_change', 'sensitive_action', 'password_reset')),
    expires_at TIMESTAMPTZ NOT NULL,
    attempts INT NOT NULL DEFAULT 0,
    max_attempts INT NOT NULL DEFAULT 5,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_email_totp_codes_user_purpose
    ON auth.email_totp_codes(user_id, purpose)
    WHERE used_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_email_totp_codes_expires_at
    ON auth.email_totp_codes(expires_at)
    WHERE used_at IS NULL;

-- Email TOTP rate limiting
CREATE TABLE IF NOT EXISTS auth.email_totp_rate_limits (
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    hour_bucket TIMESTAMPTZ NOT NULL,
    send_count INT NOT NULL DEFAULT 1,
    attempt_count INT NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, hour_bucket)
);

-- Verification sessions
CREATE TABLE IF NOT EXISTS auth.verification_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    purpose TEXT NOT NULL CHECK (purpose IN ('login', 'password_change', 'sensitive_action', 'password_reset')),
    method TEXT NOT NULL CHECK (method IN ('totp', 'passkey', 'email_totp')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_verification_sessions_user_purpose
    ON auth.verification_sessions(user_id, purpose, expires_at)
    WHERE used_at IS NULL;
