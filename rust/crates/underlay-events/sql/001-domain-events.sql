-- Underlay: Domain events/outbox table (template)
--
-- Applications should copy/adapt this into their own migrations.
-- This is intentionally small and generic.

CREATE SCHEMA IF NOT EXISTS platform;

CREATE TABLE IF NOT EXISTS platform.domain_events (
  id uuid PRIMARY KEY,
  event_type text NOT NULL,
  payload jsonb NOT NULL,
  occurred_at timestamptz NOT NULL,
  processed_at timestamptz NULL
);

CREATE INDEX IF NOT EXISTS domain_events_unprocessed_idx
  ON platform.domain_events (occurred_at)
  WHERE processed_at IS NULL;
