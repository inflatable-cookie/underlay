-- Underlay consumer template: migration.migrated_attachment_binding
--
-- Copy this shape into your app-local migration history.
-- Do not treat this file as a shared migration artifact with global ordering.

CREATE TABLE migration.migrated_attachment_binding (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    source_system text NOT NULL,
    source_attachment_type text NOT NULL,
    source_attachment_id text NOT NULL,
    source_owner_type text NOT NULL,
    source_owner_id text NOT NULL,
    field_or_purpose text NOT NULL,
    sha256 text NOT NULL,
    bundle_digest text NOT NULL,
    media_id uuid NOT NULL REFERENCES media.media(id) ON DELETE CASCADE,
    media_version_id uuid NOT NULL REFERENCES media.media_version(id) ON DELETE CASCADE,
    import_status text NOT NULL,
    imported_at timestamptz NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX migrated_attachment_binding_identity_sha_idx
    ON migration.migrated_attachment_binding (
        source_system,
        source_attachment_type,
        source_attachment_id,
        source_owner_type,
        source_owner_id,
        field_or_purpose,
        sha256
    );

CREATE INDEX migrated_attachment_binding_media_idx
    ON migration.migrated_attachment_binding (media_id, media_version_id);

CREATE INDEX migrated_attachment_binding_bundle_idx
    ON migration.migrated_attachment_binding (bundle_digest);
