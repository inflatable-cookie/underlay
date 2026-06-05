-- Underlay Jobs: LISTEN/NOTIFY support for efficient job wake-up
--
-- This migration adds a trigger that notifies waiting workers when new jobs
-- are inserted. Workers use PostgreSQL's LISTEN/NOTIFY to wake up immediately
-- instead of polling, dramatically reducing database load when idle.
--
-- How it works:
-- 1. Worker calls LISTEN 'underlay_job_notify'
-- 2. When a job is INSERTed, this trigger calls NOTIFY 'underlay_job_notify'
-- 3. Worker wakes up immediately and checks for claimable jobs
-- 4. A fallback poll (e.g., every 30s) catches any missed notifications
--
-- The notification payload includes the job_type to allow future optimization
-- where workers could filter notifications client-side.

-- =============================================================================
-- Notification trigger function
-- =============================================================================
CREATE OR REPLACE FUNCTION platform.notify_job_inserted()
RETURNS TRIGGER AS $$
BEGIN
    -- Send notification with job_type as payload
    -- Workers can use this to filter relevant notifications
    PERFORM pg_notify('underlay_job_notify', NEW.job_type);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- =============================================================================
-- Trigger on job insert
-- =============================================================================
-- Only notify for new pending jobs (not updates or status changes)
CREATE TRIGGER trigger_platform_job_notify
    AFTER INSERT ON platform.job
    FOR EACH ROW
    WHEN (NEW.status = 'pending')
    EXECUTE FUNCTION platform.notify_job_inserted();

-- =============================================================================
-- Documentation
-- =============================================================================
COMMENT ON FUNCTION platform.notify_job_inserted() IS
    'Sends pg_notify when a new job is inserted, waking up waiting workers';
