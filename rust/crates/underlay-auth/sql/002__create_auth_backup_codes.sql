-- Underlay: Backup codes table (template)
--
-- Applications should copy this file into their migration directory.
-- Rename with timestamp prefix: YYYYMMDDHHMMSS__create_auth_backup_codes.sql
--
-- This table stores one-time recovery codes for TOTP (Time-based One-Time Password).
-- Backup codes are hashed before storage and marked as used when consumed.

CREATE TABLE IF NOT EXISTS auth.backup_codes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    code_hash VARCHAR(255) NOT NULL UNIQUE,
    used_at TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_auth_backup_codes_user_id ON auth.backup_codes(user_id);
CREATE INDEX IF NOT EXISTS idx_auth_backup_codes_used_at ON auth.backup_codes(used_at) WHERE used_at IS NULL;

COMMENT ON TABLE auth.backup_codes IS 'One-time recovery codes for TOTP authentication. Codes are hashed before storage.';
COMMENT ON COLUMN auth.backup_codes.code_hash IS 'SHA-256 hash of the backup code (never store plain codes)';
COMMENT ON COLUMN auth.backup_codes.used_at IS 'Timestamp when code was used, NULL if unused';
