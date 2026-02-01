-- Add additional columns to account.user_profile for full profile support.

ALTER TABLE account.user_profile
    ADD COLUMN IF NOT EXISTS avatar_url TEXT,
    ADD COLUMN IF NOT EXISTS region_code TEXT,
    ADD COLUMN IF NOT EXISTS currency_preference TEXT,
    ADD COLUMN IF NOT EXISTS email_marketing_opt_in BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN IF NOT EXISTS email_transactional_opt_in BOOLEAN NOT NULL DEFAULT TRUE,
    ADD COLUMN IF NOT EXISTS email_frequency TEXT NOT NULL DEFAULT 'normal' CHECK (email_frequency IN ('low', 'normal', 'high')),
    ADD COLUMN IF NOT EXISTS cookie_consent JSONB,
    ADD COLUMN IF NOT EXISTS data_processing_consent_version TEXT;
