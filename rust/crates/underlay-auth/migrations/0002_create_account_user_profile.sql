-- Underlay: canonical account profile schema.
--
-- Underlay pattern:
-- - auth.* = authentication and credentials
-- - account.* = identity and personalization

CREATE SCHEMA IF NOT EXISTS account;

CREATE TABLE IF NOT EXISTS account.user_profile (
    user_id UUID PRIMARY KEY REFERENCES auth.users(id) ON DELETE CASCADE,

    -- Name fields follow an inclusive pattern:
    -- - full_name: user's full name as they wish to be known
    -- - display_name: short UI name (optional override)
    full_name TEXT,
    display_name TEXT,

    -- Locale
    country_code TEXT,
    time_zone TEXT,
    language TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_account_user_profile_time_zone
    ON account.user_profile (time_zone)
    WHERE time_zone IS NOT NULL;
