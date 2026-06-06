# g06.186 Artifact - Blob Upload Config Boundary Split

Status: complete
Owner: repo maintainers
Completed: 2026-06-06

## Purpose

Resolve the `040` drift hook where `underlay_blob::MediaConfig` mixed blob
upload-size policy with thumbnail/rendition policy.

## Result

The public config boundary is now split:

- `underlay_blob::BlobUploadConfig` owns blob upload-size policy.
- `underlay_media::renditions::RenditionConfig` owns thumbnail and rendition
  generation policy.
- `underlay_blob` no longer exports `MediaConfig`.

This keeps blob storage generic and prevents the storage crate from owning
media-specific derivative behavior.

## Consumer Upgrade Impact

Impact class: `breaking`.

Consumers importing `underlay_blob::MediaConfig` must switch by use case:

- upload size checks: `underlay_blob::BlobUploadConfig`
- thumbnail or rendition generation: `underlay_media::renditions::RenditionConfig`

Current-family scan found direct use in:

- `underlay-reference/acme-api`
- `contact-patch/cp-api`
- `loophole/composer/composer-api`

Those consumers were updated in this batch. `compli-me`, `acowtancy`,
`songsprout`, and `loophole/composer` outside the child `composer-api` had no
direct `underlay_blob::MediaConfig` call sites in the scanned Rust source.

## Validation

- `cargo test -p underlay-blob --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- consumer source scan for remaining `underlay_blob::MediaConfig`
- targeted consumer `cargo check` runs for the affected Rust packages

## Next Task

No active roadmap task remains. Continue with bounded drift repairs only, or
re-enter planning before opening a new Rust hardening lane.
