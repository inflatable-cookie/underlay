-- Captured emails table for development email capture.

CREATE TABLE IF NOT EXISTS platform.captured_email (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email_id UUID NOT NULL,
    to_addresses TEXT[] NOT NULL,
    from_address TEXT NOT NULL,
    reply_to TEXT NULL,
    cc_addresses TEXT[] NOT NULL DEFAULT '{}',
    bcc_addresses TEXT[] NOT NULL DEFAULT '{}',
    subject TEXT NOT NULL,
    text_body TEXT NULL,
    html_body TEXT NULL,
    headers_json JSONB NULL,
    captured_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    was_delivered BOOLEAN NOT NULL DEFAULT FALSE,
    delivery_error TEXT NULL
);

CREATE INDEX IF NOT EXISTS idx_captured_email_captured_at
    ON platform.captured_email (captured_at DESC);

CREATE INDEX IF NOT EXISTS idx_captured_email_to_addresses
    ON platform.captured_email USING GIN (to_addresses);
