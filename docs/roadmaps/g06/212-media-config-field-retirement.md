# g06.212 - Media Config Field Retirement

## Status

Complete.

## Scope

Close public-field compatibility boundaries for media generation config structs:

- `underlay_media::image::ThumbnailConfig`
- `underlay_media::renditions::RenditionConfig`

`StorageKeyConfig` already used private fields and accessors.

## Change

- Made thumbnail and rendition config fields private.
- Added read-only accessors for retained media generation values.
- Updated Underlay media internals and tests.
- Migrated known reference-style consumer thumbnail job handlers in:
  - `underlay-reference`
  - `contact-patch`

## Compatibility

Impact: coordinated breaking change.

Known consumer field reads were migrated. New apps must use constructors,
builders, and accessors instead of direct field reads or struct literals.

## Validation

- `cargo test -p underlay-media`
- `cargo check -p acme-jobs`
- `cargo check -p cp-jobs`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
