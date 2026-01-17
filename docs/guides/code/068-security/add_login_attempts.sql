-- Example migration: Add login attempt tracking for account lockout
-- 
-- This migration adds the necessary columns and tables to track failed
-- login attempts and implement account lockout.
--
-- IMPORTANT: Always fully-qualify objects (e.g., auth.users, auth.login_attempts)
-- Do not use SET search_path in migrations.

-- Add lockout columns to users table
ALTER TABLE auth.users
  ADD COLUMN IF NOT EXISTS failed_login_count INTEGER NOT NULL DEFAULT 0,
  ADD COLUMN IF NOT EXISTS lockout_until TIMESTAMPTZ;

-- Create login attempts table for auditing
-- This provides visibility into attack patterns and helps with security monitoring
CREATE TABLE IF NOT EXISTS auth.login_attempts (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  
  -- User reference (nullable for attempts against non-existent accounts)
  user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
  
  -- Always record the email used (even if user doesn't exist)
  email TEXT NOT NULL,
  
  -- Client information for analysis
  ip_address TEXT NOT NULL,
  user_agent TEXT,
  
  -- Outcome
  success BOOLEAN NOT NULL,
  failure_reason TEXT,  -- 'invalid_password', 'account_locked', 'rate_limited', etc.
  
  -- Timestamp
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for querying recent attempts
-- Used for security monitoring dashboards
CREATE INDEX IF NOT EXISTS idx_login_attempts_user_created
  ON auth.login_attempts(user_id, created_at DESC)
  WHERE user_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_login_attempts_ip_created
  ON auth.login_attempts(ip_address, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_login_attempts_email_created
  ON auth.login_attempts(email, created_at DESC);

-- Index for finding failed attempts (for alerting)
CREATE INDEX IF NOT EXISTS idx_login_attempts_failed_recent
  ON auth.login_attempts(created_at DESC)
  WHERE success = false;

-- Comment the table for documentation
COMMENT ON TABLE auth.login_attempts IS 
  'Audit log of all login attempts for security monitoring and lockout tracking';

COMMENT ON COLUMN auth.login_attempts.failure_reason IS 
  'Reason for failure: invalid_password, account_locked, rate_limited, user_not_found, mfa_required, mfa_failed';
