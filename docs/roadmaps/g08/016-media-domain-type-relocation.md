# g08.016 - Media Domain-Type Relocation

Status: complete
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Move media domain types out of the DB crate. `underlay-db/src/media_types.rs`
defines `MediaKind`, `MediaVersionState`, `MediaVisibility`, and `underlay-media`
depends on `underlay-db` only to re-export them, forcing anything wanting media
types onto sqlx. `MediaKind` is also hardcoded to `Image | Pdf`, an app-specific
constraint baked into the foundation, so adding video/audio for the next
consumer is a breaking enum change in the *db* crate.

## Evidence

- `rust/crates/underlay-db/src/media_types.rs:19-24`
- `underlay-media` Cargo.toml note ("Re-export enums from underlay-db")

## Governing References

- [040 Storage, blob, and media systems](../../contracts/040-storage-blob-and-media-systems.md)
- [050 Media library and usage](../../contracts/050-media-library-and-usage.md)
- [010 Foundation primitives and envelopes](../../contracts/010-foundation-primitives-and-envelopes.md)

## Planned Changes

- [x] Move the enums into `underlay-media` (or a small `underlay-media-types`),
  keeping sqlx `Type` impls behind a feature or in the `-postgres` adapter.
- [x] Make `MediaKind` `#[non_exhaustive]` or open-ended so new kinds are not a
  breaking change.
- [x] Drop the `underlay-media -> underlay-db` type dependency.

## Consumer Upgrade Impact

Impact class: `behavioral`. Import path for media enums changes. Requires
six-consumer proof per `023`.

## Validation

- [x] media types importable without sqlx; adding a `MediaKind` variant does not
  break match sites gated by `#[non_exhaustive]`
- [x] `cargo test -p underlay-media -p underlay-media-postgres`
- [x] `effigy validate`

## Stop Conditions

None expected.

## Completion Notes

Completed 2026-07-17.
- Moved `MediaKind`/`MediaVisibility`/`MediaVersionState`/`MediaTypeParseError`/
  `detect_media_kind_from_mime_type` from `underlay-db/src/media_types.rs` to
  `underlay-media/src/types.rs` (tests moved too). `underlay-media` no longer
  depends on `underlay-db` - media types are now importable without pulling
  `sqlx`. No sqlx `Type` impls existed (enums bind as strings), so no feature
  gate was needed; Postgres binding stays in `underlay-media-postgres` via the
  string reprs.
- `MediaKind` is now `#[non_exhaustive]`, so adding video/audio for a future
  consumer is not a breaking enum change. Internally only
  `underlay-media/domain/kinds.rs` re-exported the enums (repointed to
  `crate::types`); `underlay-media-postgres` and everything else already
  imported from `underlay_media` and had no exhaustive `MediaKind` match.
- Contract `040` and guide `077` updated to state `underlay-media` as the
  enum home and the import-path change; the `040` source-map pointer now
  points at `underlay-media/src/types.rs`.

## Consumer Rollout

Import path changed for four apps that imported from `underlay_db`
(`underlay-reference`, `contact-patch`, `compli-me`, `loophole/composer` - all
in `.../api/src/dto/media.rs`): repointed to `underlay_media::{MediaKind,
MediaVisibility}`. `acowtancy` already imported from `underlay_media`;
`songsprout` has no Rust media-enum usage. All six consumer api crates
`cargo check` clean; none had an exhaustive `MediaKind` match needing a
wildcard.

Validated: `cargo check --workspace --all-features` clean;
`cargo test --workspace --all-features` green (73 suites, 0 failures);
`underlay-media`/`underlay-media-postgres`/`underlay-db` test suites green.
TS surface untouched.

## Next Task

`g08.017` pagination collision resolution.
