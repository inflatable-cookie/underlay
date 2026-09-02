# g11.001 Underlay Immutable Verified Blob Promotion — Delivery

Date: 2026-09-02
Roadmap: `g11.001`, card 001
Handoff: `docs/handoffs/20260902-121455-g11-001-immutable-verified-blob-promotion.md`
Branch: `worker/g11-001-immutable-verified-blob-promotion`

## What Shipped

`underlay-blob` gains an additive immutable verified-promotion capability:

- `BlobAdapter::get_bytes_bounded(key, max_bytes)` — bounded capture (reads
  at most `max_bytes + 1` and refuses without buffering the remainder).
  Default implementation refuses with `BlobError::Unsupported`.
- `BlobAdapter::put_bytes_create_only(key, data, content_type)` — create-only
  byte write (exclusive create, never overwrite/truncate/follow). Default
  implementation refuses with `BlobError::Unsupported`.
- `BlobAdapterPromotionExt::promote_verified(staging_key, destination_key,
  declared_content_type, config)` (typed `BlobObjectKey` params) — composes
  the two primitives above: capture once, validate size/MIME/magic bytes,
  derive lowercase SHA-256, publish through exclusive create, preserve
  staging. Returns `VerifiedPromotionResult { object: StoredObject, sha256 }`.
- `BlobError::DestinationExists` (typed collision) and `BlobError::Unsupported`
  (typed fail-closed default / refused source).
- Built-in S3 adapter: one conditional `PutObject` (`If-None-Match: *`);
  409/412 map to `DestinationExists` via HTTP status, not string matching on
  provider error text. `get_bytes_bounded` streams and truncates at
  `max_bytes + 1` regardless of what response headers claim.
- Built-in local adapter: `get_bytes_bounded` refuses symlinks/non-regular
  sources via `lstat` before any `open`/`read` (no blocking on a FIFO), then
  a post-open `fstat` re-check, then a capped read. `put_bytes_create_only`
  uses `O_CREAT | O_EXCL` (+ `O_NOFOLLOW` on Unix) layered on the existing
  containment-safe path resolution.
- Old mutable APIs (`get_bytes`, `put_bytes`, `finalise_upload`,
  `finalise_upload_verified`, `initiate_upload_validated`) unchanged.

Docs: Contract 040 gets a new "Immutable verified promotion" rules block, the
public API inventory (`122`) row is updated, the media-upload-pipeline
pattern's Finalize phase now references `promote_verified`, and the
Unreleased changelog entry describes the surface.

## Stop-Condition Probes

Both required before implementation, both cleared:

1. **AWS SDK conditional destination create.** `aws-sdk-s3` resolves to
   `1.140.0` in the lockfile; its `PutObject` builder exposes `if_none_match`,
   documented to return `412 Precondition Failed` or `409
   ConditionalRequestConflict` on collision. Proved live (not just read from
   source) with an `aws-smithy-runtime` `StaticReplayClient` fixture: one
   test asserts the outgoing request carries `If-None-Match: *` and that
   exactly one request is sent; two more feed canned 412/409 responses back
   and assert `BlobError::DestinationExists`; a fourth feeds 500 and asserts
   it is *not* read as a collision.
2. **Local exclusive no-follow, containment-safe create.** Confirmed
   `tokio::fs::OpenOptions` exposes `custom_flags` natively (no
   `OpenOptionsExt` import needed) so `O_NOFOLLOW` composes with
   `create_new(true)` (`O_CREAT | O_EXCL`), layered on the pre-existing
   `joined_path_within_base` containment check. Proved with real filesystem
   tests: occupied file, existing directory, existing symlink (target
   untouched), and a genuine two-task concurrent race to the same
   destination (exactly one winner).

Neither stop condition triggered; no fallback plan was needed.

## Proof Summary (card + strict spec oracle)

All driven with real Rust tests, not just reasoning:

| Oracle | Proof |
| --- | --- |
| Captured bytes are the publication source | `promotion_tests.rs`: an in-memory `FakeAdapter` swaps staging to different same-size/same-MIME bytes *after* `get_bytes_bounded` returns but before create; published bytes/SHA-256 match the original capture, not the swap |
| Capture bounded before allocation completes | Local: reads a 10-byte file with `max_bytes=3`, observed `TooLarge(4, 3)` — stops at exactly max+1. S3: replay fixture with a 1024-byte body and `max_bytes=16` also stops at exactly 17 bytes, driven by streaming truncation, not response-header trust |
| Destination creation is exclusive | Local: two concurrent `tokio::spawn` writers to the same key — exactly one `Ok`, one `DestinationExists`, file holds one writer's bytes intact. S3: fixture proves the single conditional `PutObject` request shape |
| Collision cannot forge success | Local: occupied file/directory/symlink destinations all refuse via `DestinationExists`, original bytes untouched. S3: 412 and 409 fixtures both map to `DestinationExists` |
| Retry is explicit | No convergent retry was added (out of scope per "if the worker adds..."); a collision always returns the typed `DestinationExists`, never an unconditional overwrite fallback |
| Derived metadata is authoritative | `promote_verified` never accepts a client digest — the only inputs are the staging key, destination key, and declared MIME; SHA-256 and size come from the captured vector |
| Publication and selection atomic in app DB | Out of scope for this card (no consumer DB/app code touched) |

Additional adapter-boundary proof: `NoopAdapter` (this crate's own minimal
custom-adapter example) compiles unchanged and both new methods refuse via
`BlobError::Unsupported` through the trait defaults — proving third-party
adapters keep compiling without silently gaining unbounded-read or
overwrite behavior.

## Validation

- `cargo test -p underlay-blob --all-features`: 56 passed, 0 failed.
- `cargo check --workspace --all-features`: clean.
- `cargo clippy --workspace --all-features --all-targets -- -D warnings`:
  clean.
- `cargo test --workspace --all-features`: clean except one known
  pre-existing flaky test (below).
- `effigy qa:docs`, `effigy qa:northstar`: clean.
- `effigy doctor`: clean except one pre-existing-ratio finding (below).
- `git diff --check`: clean (fixed a trailing-blank-line artifact from an
  editing pass).

### Inherited / Pre-Existing Failures (Not This Branch)

1. **`underlay-http-client` flaky timeout test.** Already documented in
   `docs/logs/2026-09/01-091500-g10-001-northstar-instruction-and-language-audit.md`:
   `tests::invalid_user_agent_fallback_retains_default_timeout` races a
   paused-clock assertion against a real reqwest timeout. This branch does
   not touch `underlay-http-client`.
2. **TS `workspace-shape` fixture gap.** `ts/tests/tools/workspace-shape.test.ts`
   > "flags disposable leftover top-level package trees..." fails on a clean
   checkout because the `retired-top-level-package` fixture's top-level
   `app/` directory was never actually committed (git does not track empty
   directories). Confirmed via `git status --short` that this branch has no
   `ts/` changes. Recorded as a new open papercut in `PAPERCUTS.md`.
3. **`effigy doctor` `scan.comment-ratio` on `lib.rs`.** Reported
   `ratio=2.95 [high]` (59 comment / 20 code) on `underlay-blob/src/lib.rs`.
   Checked against `HEAD`: the pre-change file was already `53 comment / 18
   code` = ratio `2.94` — this branch's one-line "Verified Promotion" pointer
   addition left the ratio effectively unchanged. `AGENTS.md` already notes
   Underlay carries known doctor scan backlog; this crate-level doc comment
   was already at this ratio before `g11.001`.

This worktree also had no `node_modules` (never bootstrapped); `bun install`
was run once to make `effigy qa` runnable at all — a local bring-up step, not
a repo change.

## Public API Shape

New crate-root exports from `underlay-blob`: `BlobAdapterPromotionExt`,
`VerifiedPromotionResult`. New `BlobError` variants: `DestinationExists`,
`Unsupported`. New `BlobAdapter` trait methods (both with fail-closed
defaults): `get_bytes_bounded`, `put_bytes_create_only`. All existing public
items are unchanged.

## Consumer Upgrade Note

Additive, pre-1.0 minor. No consumer repo, dependency pin, or release was
touched — release stays blocked on `g11.001` card 002 per the roadmap. Once a
tag ships, a consumer's live upload-finalisation path should call
`promote_verified` at its finalize step and persist the returned destination
key plus server-derived SHA-256/size/MIME, rather than the presigned staging
key or a client-supplied digest. `finalise_upload_verified` remains available
unchanged and does not establish immutable publication — this is a distinct
addition, not a replacement.

## Next Task

Open one PR to `main` from this branch. Orchestrator reviews the exact head;
release and consumer adoption stay blocked per `g11.001`.
