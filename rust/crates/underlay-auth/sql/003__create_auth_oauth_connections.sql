-- Underlay: OAuth connections table (template)
--
-- Applications should copy this file into their migration directory.
-- Rename with timestamp prefix: YYYYMMDDHHMMSS__create_auth_oauth_connections.sql
--
-- This table stores external OAuth provider connections (e.g., Google Sign-In).
-- Tokens are encrypted at rest using AES-256-GCM.
--
-- Note: Some apps may prefer to store OAuth connections in auth_credentials
-- with type='oauth_google' and use this table for additional token management.
-- This table provides a dedicated structure for OAuth-specific fields.

CREATE TABLE IF NOT EXISTS auth.oauth_connections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    provider VARCHAR(50) NOT NULL CHECK (provider IN ('google')),
    provider_user_id VARCHAR(255) NOT NULL,
    access_token_encrypted TEXT NOT NULL,
    refresh_token_encrypted TEXT NULL,
    token_expires_at TIMESTAMPTZ NULL,
    scopes TEXT[] NULL,
    connected_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ NULL
);

CREATE INDEX IF NOT EXISTS idx_auth_oauth_connections_user_id ON auth.oauth_connections(user_id);
CREATE INDEX IF NOT EXISTS idx_auth_oauth_connections_provider_user_id ON auth.oauth_connections(provider, provider_user_id);
CREATE INDEX IF NOT EXISTS idx_auth_oauth_connections_provider ON auth.oauth_connections(provider);

COMMENT ON TABLE auth.oauth_connections IS 'External OAuth provider connections (e.g., Google Sign-In). Tokens are encrypted at rest.';
COMMENT ON COLUMN auth.oauth_connections.provider_user_id IS 'The OAuth provider''s unique identifier for this user';
COMMENT ON COLUMN auth.oauth_connections.access_token_encrypted IS 'AES-256-GCM encrypted access token';
COMMENT ON COLUMN auth.oauth_connections.refresh_token_encrypted IS 'AES-256-GCM encrypted refresh token (if provided by provider)';
COMMENT ON COLUMN auth.oauth_connections.scopes IS 'Array of OAuth scopes granted to this connection';
