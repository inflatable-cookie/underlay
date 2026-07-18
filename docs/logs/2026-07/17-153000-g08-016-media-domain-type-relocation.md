# 2026-07-17 - g08.016 media domain-type relocation

## Context

`MediaKind`/`MediaVisibility`/`MediaVersionState` lived in `underlay-db`, and
`underlay-media` depended on `underlay-db` only to re-export them - forcing
anything wanting media types onto `sqlx`. `MediaKind` was also a closed
`Image | Pdf`, so adding a kind for a future consumer was a breaking enum
change in the *db* crate.

## Changes

- Moved the enums (+ `MediaTypeParseError`, `detect_media_kind_from_mime_type`,
  and their tests) from `underlay-db/src/media_types.rs` to
  `underlay-media/src/types.rs`. `underlay-media` no longer depends on
  `underlay-db`; media types are importable without `sqlx`. No sqlx `Type`
  impls existed (enums bind as strings), so no feature gate was required -
  Postgres binding stays in `underlay-media-postgres` via the string reprs.
- `MediaKind` is now `#[non_exhaustive]`: adding video/audio later is not a
  breaking change; external `match` sites need a wildcard arm.
- Internally only `underlay-media/domain/kinds.rs` re-exported from db
  (repointed to `crate::types`); every other crate already imported from
  `underlay_media`.
- Contract `040` + guide `077` updated to name `underlay-media` as the enum
  home and record the import-path change.

## Consumer Rollout

Four apps imported from `underlay_db` (`underlay-reference`, `contact-patch`,
`compli-me`, `loophole/composer`, all `.../api/src/dto/media.rs`); repointed to
`underlay_media`. `acowtancy` already used `underlay_media`; `songsprout` has
no Rust media-enum usage. All six consumer api crates `cargo check` clean; none
had an exhaustive `MediaKind` match requiring a wildcard.

## Validation

- `cargo check --workspace --all-features`: clean.
- `cargo test --workspace --all-features`: green (73 suites, 0 failures).
- `underlay-media`/`underlay-media-postgres`/`underlay-db` suites green.
- Six consumer api crates: `cargo check` clean.
- TS surface untouched.

## Consumer Upgrade Notes

Impact class **behavioral** (import path). Consumers importing media enums from
`underlay_db` must import them from `underlay_media` instead (both crates are
already normal deps in every app). `MediaKind` being `#[non_exhaustive]` means
any future exhaustive match must add a wildcard arm; no current consumer had
one.

## Next

`g08.017` pagination collision resolution (one public pagination type per
semantic).
