# g06.185 Artifact - Media Repository Usage Trait Split

Status: complete
Owner: repo maintainers
Completed: 2026-06-06

## Purpose

Resolve the `040` drift hook where `MediaRepository` mixed media lifecycle
operations with older simple usage tracking while `050` promoted a richer
usage-edge direction.

## Result

The Rust media repository boundary is now split:

- `MediaRepository` owns media lifecycle, version, rendition, trash, and batch
  lifecycle operations.
- `MediaUsageRepository` owns the retained older simple usage model:
  `MediaUsage`, `track_usage`, `remove_usage`, `list_usages`,
  `is_media_used`, `get_usage_count`, and `sync_usages`.
- `underlay_media::sync::MediaUsageSyncRepository` remains the generalized
  usage-edge sync seam for `050`.

`PostgresMediaRepository` implements both `MediaRepository` and
`MediaUsageRepository`.

## Consumer Upgrade Impact

Impact class: `breaking`.

Consumers that call simple usage methods through the shared media repository
trait must import `MediaUsageRepository` or implement it separately from
`MediaRepository`.

Current six-consumer scan found no direct implementation of Underlay's
`MediaRepository` and no direct calls to the affected Underlay simple-usage
trait methods. Acowtancy/Farmyard has app-local equivalent media repository
methods; production code compiled without changes.

## Validation

- `cargo check -p underlay-media -p underlay-media-postgres --all-features`
- `cargo test -p underlay-media -p underlay-media-postgres --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy check:exports`
- six-consumer source scan for direct Underlay `MediaRepository`
  implementations and simple-usage calls
- `cargo check -p farmyard-domain -p farmyard-api` in
  `acowtancy/farmyard`

Known external validation gap:

- `cargo check -p farmyard-domain -p farmyard-api --all-targets` still fails
  on existing Farmyard API test drift in `crates/api/src/main.rs` and
  `crates/api/src/dto/content.rs`, unrelated to the media trait split.

## Next Task

No active roadmap task remains. Open a bounded roadmap card before starting the
next compatibility-retirement, TS boundary, or Rust hardening lane.
