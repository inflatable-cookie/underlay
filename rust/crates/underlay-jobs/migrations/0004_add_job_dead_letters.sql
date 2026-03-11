-- Underlay Jobs: dead-letter persistence for failed jobs
--
-- Adds a separate table for exhausted or permanent job failures so operators can
-- inspect them, archive them, and retry them as fresh jobs without overloading
-- the main queue table.

CREATE TABLE platform.job_dead_letter (
    id UUID PRIMARY KEY,
    original_job_id UUID NOT NULL REFERENCES platform.job(id) ON DELETE CASCADE,
    job_type TEXT NOT NULL,
    payload JSONB NOT NULL,
    attempts INT NOT NULL,
    max_attempts INT NOT NULL,
    priority INT NOT NULL DEFAULT 0,
    last_error TEXT NOT NULL,
    error_history JSONB NOT NULL DEFAULT '[]',
    failed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retried_at TIMESTAMPTZ,
    retried_job_id UUID REFERENCES platform.job(id),
    archived_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_platform_job_dead_letter_failed_at
    ON platform.job_dead_letter (failed_at DESC);

CREATE INDEX idx_platform_job_dead_letter_job_type
    ON platform.job_dead_letter (job_type, failed_at DESC);

CREATE INDEX idx_platform_job_dead_letter_active
    ON platform.job_dead_letter (archived_at, retried_at, failed_at DESC);

CREATE TRIGGER trigger_platform_job_dead_letter_updated_at
    BEFORE UPDATE ON platform.job_dead_letter
    FOR EACH ROW
    EXECUTE FUNCTION platform.job_update_timestamp();

COMMENT ON TABLE platform.job_dead_letter IS 'Dead-letter records for jobs that exhausted retries or failed permanently';
COMMENT ON COLUMN platform.job_dead_letter.retried_job_id IS 'Fresh job created when the dead letter is requeued';
