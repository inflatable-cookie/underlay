-- Underlay Jobs: LISTEN/NOTIFY support for efficient job wake-up.
-- Source: underlay/rust/crates/underlay-jobs/migrations/0002_add_job_notify.sql

CREATE OR REPLACE FUNCTION platform.notify_job_inserted()
RETURNS TRIGGER AS $$
BEGIN
    PERFORM pg_notify('underlay_job_notify', NEW.job_type);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_platform_job_notify
    AFTER INSERT ON platform.job
    FOR EACH ROW
    WHEN (NEW.status = 'pending')
    EXECUTE FUNCTION platform.notify_job_inserted();

COMMENT ON FUNCTION platform.notify_job_inserted() IS
    'Sends pg_notify when a new job is inserted, waking up waiting workers';
