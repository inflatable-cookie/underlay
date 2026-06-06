# g06.209 - Storage Config Field Retirement

## Status

Complete.

## Scope

Close public-field compatibility boundaries for storage-related config structs:

- `underlay_aws::AwsConfig`
- `underlay_blob::BlobUploadConfig`
- `underlay_blob::LocalConfig`
- `underlay_blob::S3Config`

## Change

- Made storage config fields private.
- Kept existing constructors and builder-style setters.
- Added read-only accessors for retained config values.
- Updated Underlay blob and AWS internals and tests.

## Compatibility

Impact: breaking for unknown direct field users.

Known consumers use constructors and builder-style setters for these config
types. New apps must use constructors, builders, and accessors instead of direct
field reads or struct literals.

## Validation

- `cargo check -p underlay-blob -p underlay-aws`
- `cargo test -p underlay-blob`
- `cargo test -p underlay-aws`
- `cargo check -p acme-api -p acme-jobs`
- `cargo check -p cp-api -p cp-jobs`
- `cargo check -p compli-me-api`
- `cargo check -p farmyard-infra`
- `cargo check -p nursery-api`
- `cargo check -p composer-api`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
