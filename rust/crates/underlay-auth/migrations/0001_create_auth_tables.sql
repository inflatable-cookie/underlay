-- Underlay: Canonical auth schema (derived from Acowtancy).
--
-- This migration defines the canonical persistence layer for Underlay auth.
-- Applications should sync this migration into their own `migrations/` folder
-- (via `underlay-devtools sync-migrations`) and run a single sqlx migrator.

CREATE SCHEMA IF NOT EXISTS auth;

-- =========================================
-- Users
-- =========================================

CREATE TABLE IF NOT EXISTS auth.users (
    id UUID PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,

    -- Coarse primary role (mirrors common Underlay Principal roles).
    role TEXT NOT NULL DEFAULT 'student'
        CHECK (role IN ('student', 'tester', 'tutor', 'editor', 'admin', 'support', 'superadmin')),

    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'suspended', 'deleted')),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_auth_users_email ON auth.users (email);

-- =========================================
-- Credentials
-- =========================================

CREATE TABLE IF NOT EXISTS auth.credentials (
    id UUID PRIMARY KEY,

    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,

    type TEXT NOT NULL
        CHECK (type IN ('password', 'totp', 'passkey', 'oauth_google')),

    -- For password this stores a hash; for others it is an encrypted blob.
    secret_encrypted TEXT NOT NULL,

    -- JSON metadata matching Underlay CredentialMetadata shapes.
    metadata JSONB NOT NULL,

    verified BOOLEAN NOT NULL DEFAULT TRUE,

    -- Optional user-facing label (e.g. passkey device name).
    display_name TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_auth_credentials_user_id ON auth.credentials (user_id);
CREATE INDEX IF NOT EXISTS idx_auth_credentials_type ON auth.credentials (type);

-- Password/TOTP/OAuthGoogle should be unique per user.
-- Passkeys should allow multiple entries per user.
CREATE UNIQUE INDEX IF NOT EXISTS idx_auth_password_unique
    ON auth.credentials (user_id)
    WHERE type = 'password';

CREATE UNIQUE INDEX IF NOT EXISTS idx_auth_totp_unique
    ON auth.credentials (user_id)
    WHERE type = 'totp';

CREATE UNIQUE INDEX IF NOT EXISTS idx_auth_oauth_google_unique
    ON auth.credentials (user_id)
    WHERE type = 'oauth_google';

-- Passkey credential IDs should be globally unique.
CREATE UNIQUE INDEX IF NOT EXISTS idx_auth_passkey_credential_id_unique
    ON auth.credentials ((metadata->>'credentialId'))
    WHERE type = 'passkey';

-- =========================================
-- Sessions
-- =========================================

CREATE TABLE IF NOT EXISTS auth.sessions (
    id UUID PRIMARY KEY,

    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,

    -- Application roles snapshot (used when building principals).
    roles JSONB NOT NULL,

    is_active BOOLEAN NOT NULL,

    access_token_fingerprint TEXT NOT NULL,
    refresh_token_fingerprint TEXT NOT NULL,

    -- Refresh rotation state.
    refresh_token_id UUID NOT NULL,
    refresh_token_version INTEGER NOT NULL,

    access_token_expires_at TIMESTAMPTZ NOT NULL,
    refresh_token_expires_at TIMESTAMPTZ NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    ip_address TEXT,
    user_agent TEXT,

    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'revoked', 'expired')),
    revocation_reason TEXT,
    revoked_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_auth_sessions_user_id ON auth.sessions (user_id);
CREATE INDEX IF NOT EXISTS idx_auth_sessions_active ON auth.sessions (user_id, is_active);
CREATE INDEX IF NOT EXISTS idx_auth_sessions_refresh_id ON auth.sessions (refresh_token_id);

-- =========================================
-- Auth state (multi-step flows)
-- =========================================

CREATE TABLE IF NOT EXISTS auth.auth_state (
    id UUID PRIMARY KEY,
    user_id UUID NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    state_type TEXT NOT NULL,
    state JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_auth_auth_state_user ON auth.auth_state (user_id);
CREATE INDEX IF NOT EXISTS idx_auth_auth_state_expires ON auth.auth_state (expires_at);
CREATE INDEX IF NOT EXISTS idx_auth_auth_state_type ON auth.auth_state (state_type);

-- =========================================
-- TOTP replay protection + backup code hashes
-- =========================================

CREATE TABLE IF NOT EXISTS auth.totp_credential (
    credential_id UUID PRIMARY KEY REFERENCES auth.credentials(id) ON DELETE CASCADE,
    last_counter BIGINT NOT NULL DEFAULT 0,
    backup_code_hashes JSONB NOT NULL
);
