-- Underlay Jobs: Background job queue tables
--
-- This migration defines the canonical persistence layer for underlay-jobs.
-- Applications should sync this migration into their own `migrations/` folder
-- (via `underlay-devtools sync-migrations`) and run a single sqlx migrator.
--
-- Schema: Uses `platform` schema by default. Modify if your app uses a different schema.

-- =============================================================================
-- Ensure platform schema exists
-- =============================================================================
CREATE SCHEMA IF NOT EXISTS platform;

-- =============================================================================
-- Core job queue table
-- =============================================================================
CREATE TABLE platform.job (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Job identification
    job_type TEXT NOT NULL,

    -- Status: pending | claimed | running | succeeded | failed | cancelled
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending', 'claimed', 'running', 'succeeded', 'failed', 'cancelled')),

    -- Job-specific data
    payload JSONB NOT NULL DEFAULT '{}',

    -- Retry configuration
    attempts INT NOT NULL DEFAULT 0,
    max_attempts INT NOT NULL DEFAULT 1,

    -- Scheduling
    scheduled_for TIMESTAMPTZ,  -- NULL = run immediately when claimed
    priority INT NOT NULL DEFAULT 0,  -- Higher = more urgent

    -- Execution tracking
    claimed_at TIMESTAMPTZ,
    claimed_by TEXT,  -- Worker identifier
    started_at TIMESTAMPTZ,
    finished_at TIMESTAMPTZ,
    heartbeat_at TIMESTAMPTZ,

    -- Progress tracking (for long-running jobs)
    -- Schema: { "current": 50, "total": 100, "message": "Processing item 50/100" }
    progress JSONB,

    -- Error tracking
    last_error TEXT,
    -- Array of { "attempt": 1, "error": "...", "at": "2026-01-27T12:00:00Z" }
    error_history JSONB NOT NULL DEFAULT '[]',

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for efficient job claiming: find pending jobs that are due
-- Uses partial index to only index claimable jobs
CREATE INDEX idx_platform_job_claimable
    ON platform.job (scheduled_for NULLS FIRST, priority DESC, created_at)
    WHERE status = 'pending';

-- Index for finding running/claimed jobs (stall detection)
CREATE INDEX idx_platform_job_running
    ON platform.job (heartbeat_at)
    WHERE status IN ('claimed', 'running');

-- Index for listing jobs by type
CREATE INDEX idx_platform_job_type ON platform.job (job_type, created_at DESC);

-- Index for listing recent jobs
CREATE INDEX idx_platform_job_created ON platform.job (created_at DESC);

-- Index for finding jobs by status
CREATE INDEX idx_platform_job_status ON platform.job (status, created_at DESC);

-- Trigger to update updated_at
CREATE OR REPLACE FUNCTION platform.job_update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_platform_job_updated_at
    BEFORE UPDATE ON platform.job
    FOR EACH ROW
    EXECUTE FUNCTION platform.job_update_timestamp();

-- =============================================================================
-- Scheduled task definitions (recurring jobs)
-- =============================================================================
CREATE TABLE platform.scheduled_task (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Task identification (unique name for upsert)
    name TEXT NOT NULL UNIQUE,
    job_type TEXT NOT NULL,

    -- Cron schedule expression (e.g., "0 3 * * *" for 3am daily)
    schedule TEXT NOT NULL,

    -- Task configuration
    payload JSONB NOT NULL DEFAULT '{}',
    max_attempts INT NOT NULL DEFAULT 1,
    timeout_seconds INT,  -- NULL = no timeout
    allow_overlap BOOLEAN NOT NULL DEFAULT FALSE,
    priority INT NOT NULL DEFAULT 0,

    -- State
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    last_scheduled_at TIMESTAMPTZ,  -- When we last created a job for this task
    last_completed_at TIMESTAMPTZ,  -- When the last job completed successfully

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for finding enabled tasks to schedule
CREATE INDEX idx_platform_scheduled_task_enabled
    ON platform.scheduled_task (enabled, name)
    WHERE enabled = TRUE;

-- Trigger to update updated_at
CREATE TRIGGER trigger_platform_scheduled_task_updated_at
    BEFORE UPDATE ON platform.scheduled_task
    FOR EACH ROW
    EXECUTE FUNCTION platform.job_update_timestamp();

-- =============================================================================
-- Job history table (archived completed jobs for visibility)
-- =============================================================================
CREATE TABLE platform.job_history (
    id UUID PRIMARY KEY,
    job_type TEXT NOT NULL,
    status TEXT NOT NULL,
    payload JSONB,
    attempts INT NOT NULL,
    started_at TIMESTAMPTZ,
    finished_at TIMESTAMPTZ,
    duration_ms BIGINT,
    last_error TEXT,
    created_at TIMESTAMPTZ NOT NULL
);

-- Index for querying history by type
CREATE INDEX idx_platform_job_history_type
    ON platform.job_history (job_type, created_at DESC);

-- Index for querying recent history
CREATE INDEX idx_platform_job_history_created
    ON platform.job_history (created_at DESC);

-- =============================================================================
-- Helper function to archive completed jobs
-- =============================================================================
CREATE OR REPLACE FUNCTION platform.archive_completed_jobs(older_than_interval INTERVAL)
RETURNS BIGINT AS $$
DECLARE
    archived_count BIGINT;
BEGIN
    -- Insert into history
    INSERT INTO platform.job_history (id, job_type, status, payload, attempts, started_at, finished_at, duration_ms, last_error, created_at)
    SELECT
        id,
        job_type,
        status,
        payload,
        attempts,
        started_at,
        finished_at,
        EXTRACT(EPOCH FROM (finished_at - started_at)) * 1000,
        last_error,
        created_at
    FROM platform.job
    WHERE status IN ('succeeded', 'failed', 'cancelled')
      AND finished_at < NOW() - older_than_interval;

    GET DIAGNOSTICS archived_count = ROW_COUNT;

    -- Delete from main table
    DELETE FROM platform.job
    WHERE status IN ('succeeded', 'failed', 'cancelled')
      AND finished_at < NOW() - older_than_interval;

    RETURN archived_count;
END;
$$ LANGUAGE plpgsql;

-- =============================================================================
-- Helper function to purge old job history
-- =============================================================================
CREATE OR REPLACE FUNCTION platform.purge_job_history(older_than_interval INTERVAL)
RETURNS BIGINT AS $$
DECLARE
    purged_count BIGINT;
BEGIN
    DELETE FROM platform.job_history
    WHERE created_at < NOW() - older_than_interval;

    GET DIAGNOSTICS purged_count = ROW_COUNT;
    RETURN purged_count;
END;
$$ LANGUAGE plpgsql;

-- =============================================================================
-- Comments for documentation
-- =============================================================================
COMMENT ON TABLE platform.job IS 'Background job queue for async task processing';
COMMENT ON TABLE platform.scheduled_task IS 'Definitions of recurring scheduled tasks';
COMMENT ON TABLE platform.job_history IS 'Archived completed jobs for visibility and debugging';

COMMENT ON COLUMN platform.job.status IS 'Job lifecycle: pending -> claimed -> running -> succeeded|failed|cancelled';
COMMENT ON COLUMN platform.job.scheduled_for IS 'When to run the job. NULL means run immediately when claimed.';
COMMENT ON COLUMN platform.job.priority IS 'Higher priority jobs are claimed first. Default 0.';
COMMENT ON COLUMN platform.job.heartbeat_at IS 'Last heartbeat from worker. Used to detect stalled jobs.';
COMMENT ON COLUMN platform.job.progress IS 'Progress for long-running jobs: {current, total, message}';

COMMENT ON COLUMN platform.scheduled_task.schedule IS 'Cron expression: minute hour day month weekday';
COMMENT ON COLUMN platform.scheduled_task.allow_overlap IS 'If false, skip scheduling if previous instance is still running';
