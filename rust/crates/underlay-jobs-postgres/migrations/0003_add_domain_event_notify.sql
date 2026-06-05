-- Domain Event LISTEN/NOTIFY support
--
-- This creates a trigger that notifies 'underlay_domain_event_notify' when
-- events are inserted into platform.domain_events, enabling efficient
-- wake-up of outbox processors.

-- Create notify function
CREATE OR REPLACE FUNCTION platform.notify_domain_event_inserted()
RETURNS trigger AS $$
BEGIN
    PERFORM pg_notify('underlay_domain_event_notify', NEW.event_type);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger (drop first to make migration idempotent)
DROP TRIGGER IF EXISTS domain_event_inserted ON platform.domain_events;
CREATE TRIGGER domain_event_inserted
    AFTER INSERT ON platform.domain_events
    FOR EACH ROW
    EXECUTE FUNCTION platform.notify_domain_event_inserted();
