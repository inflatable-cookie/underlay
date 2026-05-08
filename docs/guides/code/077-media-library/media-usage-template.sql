-- Underlay consumer template: media.media_usage
--
-- Copy this shape into your app-local migration history.
-- Do not treat this file as a shared migration artifact with global ordering.

CREATE TABLE media.media_usage (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    media_id uuid NOT NULL REFERENCES media.media(id) ON DELETE CASCADE,
    used_by_type text NOT NULL,
    used_by_id uuid,
    owner_field text,
    content_kind text NOT NULL,
    locator_kind text NOT NULL,
    locator_key text NOT NULL,
    usage_role text NOT NULL,
    provenance_kind text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE (
        media_id,
        used_by_type,
        used_by_id,
        owner_field,
        locator_kind,
        locator_key,
        provenance_kind
    )
);

CREATE INDEX idx_media_usage_media_id
    ON media.media_usage(media_id);

CREATE INDEX idx_media_usage_used_by_scope
    ON media.media_usage(used_by_type, used_by_id, provenance_kind);

CREATE INDEX idx_media_usage_owner_field
    ON media.media_usage(used_by_type, used_by_id, owner_field, provenance_kind);
