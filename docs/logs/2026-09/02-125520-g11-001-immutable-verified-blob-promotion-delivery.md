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

**Correction (see Review Repair below):** this original pass did not run
`cargo fmt --all --check`. Both hosted CI gates on this PR's exact head
failed that check. That gap — an omission, not a false "clean" claim about a
check that was actually run — is fixed in the repair below and `cargo fmt`
is now part of the validation list every time.

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

## Review Repair (orchestrator review on exact head `4d10534bd79c4f95c51adf1611c5f981edda140b`)

The orchestrator requested changes on PR #23
([review](https://github.com/inflatable-cookie/underlay/pull/23#issuecomment-5509885605)).
All five findings were in-bounds for this branch; the additive trait/default
shape and existing mutable APIs were preserved unchanged throughout.

1. **Local containment was still check-then-traverse.** `path_for_key()` +
   `create_dir_all(parent)` resolved a lexical path once, then acted on it
   later; a concurrent replacement of a not-yet-opened parent component with
   a symlink could redirect traversal outside `canonical_base`, and
   `O_NOFOLLOW` only ever protected the final component. Rewrote
   `get_bytes_bounded`/`put_bytes_create_only`'s local implementation to
   walk `canonical_base` to the key one component at a time using
   `openat(..., O_NOFOLLOW)` relative to the previous directory descriptor
   (`mkdirat` for missing intermediate directories on the create path). Each
   step's containment guarantee is that step's own syscall, not a check
   performed before it, so there is no window between "checked" and "used"
   for a concurrent swap to land in. On non-Unix platforms both methods fail
   closed with `BlobError::Unsupported` rather than falling back to the old,
   weaker resolution while still claiming no-follow behavior. New proof:
   `local_containment_race_tests.rs` — a parent component that is a symlink
   out of the base refuses before ever touching the real target it points
   to, for both read and create, including a two-level case where the old
   code would have `mkdir`ed inside the outside directory.
2. **Bounded local capture could still block on a raced FIFO.** The
   read path now opens with `O_NOFOLLOW | O_NONBLOCK` (open on a FIFO with
   `O_NONBLOCK` never blocks, even without a writer) and keeps the post-open
   `fstat` regular-file check before any `read`. The separate pre-open
   `lstat` check is gone — it was itself a check-then-act step; the
   open+fstat sequence is now the entire, atomic guard. New proof:
   `test_get_bytes_bounded_refuses_a_fifo_source_without_blocking` creates a
   real FIFO and asserts the call both refuses and completes inside a 5s
   timeout.
3. **The S3 create-only error path leaked raw backend detail.** Every
   non-409/412 response was rendered through `sdk_error_details` (provider
   code, message, and full `Debug` output) into the public error. Added
   `S3Adapter::redacted_transport_error`: full detail is now logged via
   `tracing::warn!` for operators, but the public `BlobError` carries only a
   fixed operation label and, when available, the HTTP status code.
   Applied to `put_bytes_create_only`'s non-collision path, `get_bytes_bounded`'s
   initial request, and its body-read streaming errors. New proof:
   `s3_redaction_tests.rs` — hostile 409/412/500/403 fixtures whose bodies
   contain `AWSAccessKeyId`/`StringToSign`/signature-shaped text, asserting
   the returned error's `Display`/`Debug` contain none of those markers.
4. **`promote_verified` trusted adapter-returned identity/size.** It only
   overwrote `content_type` on the returned `StoredObject`. Added a check
   after `put_bytes_create_only` returns: if `stored.key != destination_key`
   or `stored.size != bytes.len()`, return `BlobError::Internal` instead of
   a result that no longer actually binds the destination to the captured
   vector. New proof: a `LyingAdapter` test double that genuinely,
   exclusively writes the real bytes at the real key but reports a
   different key or size back; both cases are asserted as `Internal`
   errors, not silent success.
5. **Both required CI runs failed `cargo fmt --all --check`.** Ran
   `cargo fmt --all`; verified `--check` passes at the repaired head.

### Re-Validation At The Repaired Head

- `cargo test -p underlay-blob --all-features`: 66 passed, 0 failed (was 56;
  +10 from the four new test files above).
- `cargo fmt --all -- --check`: clean (was failing).
- `cargo check --workspace --all-features`: clean.
- `cargo clippy --workspace --all-features --all-targets -- -D warnings`: clean.
- `effigy qa:docs`, `effigy qa:northstar`: clean.
- `effigy doctor`: unchanged from the first pass — same one pre-existing
  `scan.comment-ratio` finding on `lib.rs` (still `ratio=2.95`, this repair
  touched no doc comments in that file); `scan.god-files` stayed
  warning-tier only (the new test files stayed under the error threshold).
- `git diff --check`: clean.
- `cargo test --workspace --all-features --no-fail-fast`: two pre-existing,
  unrelated flakes observed, neither touched by this branch (confirmed via
  `git status --short`):
  - `underlay-http-client::tests::invalid_user_agent_fallback_retains_default_timeout`
    (already documented, see above).
  - **New observation:** `underlay-http::http_config::tests::test_local_defaults`
    failed once in the full parallel workspace run, then passed both in
    isolation (`cargo test -p underlay-http --lib
    http_config::tests::test_local_defaults`) and as part of `underlay-http`'s
    own full local suite. Consistent with process-env-var test interference
    under parallel execution, not a regression from this branch. Not
    independently re-documented as a papercut; flagged here for the
    orchestrator's awareness only.

## Second Review Repair (re-review on exact head `1165ceee4ee4e1aecf18413c1ebf0a9f8a4fc439`)

The five first-round findings were confirmed materially repaired. Two
composed local-adapter cases remained blocking
([review](https://github.com/inflatable-cookie/underlay/pull/23#issuecomment-5510102046)),
both in-bounds. Additive trait/default shape and existing mutable APIs
preserved unchanged throughout.

1. **The base directory itself was still reopened through a mutable
   pathname.** `LocalAdapter` stored only `canonical_base: PathBuf`; every
   bounded read/create re-opened it by that pathname on every call
   (`libc::open(canonical_base, ...)`, no `O_NOFOLLOW`). Renaming the base
   directory after construction and replacing its old pathname with a
   symlink would make that reopen follow the replacement, silently
   redirecting every subsequent `openat(O_NOFOLLOW)` step to the wrong root.
   Fixed by pinning one owned base-directory descriptor
   (`LocalAdapter::base_dir: std::fs::File`, opened once in `new()`) and
   changing `get_bytes_bounded`/`put_bytes_create_only` to descend from a
   `try_clone()` duplicate of that descriptor instead of ever re-opening the
   base by path. A file descriptor keeps referring to the same directory
   inode across a rename (or even an unlink) of its pathname — this is
   guaranteed POSIX behavior, not a best-effort mitigation. New proof:
   `local_base_pin_tests.rs` — construct the adapter, rename the real base
   directory away, plant a symlink to an attacker directory at the old
   pathname, then prove both read and create still resolve inside the real
   (moved) directory and the attacker directory stays empty.
2. **Local create exposed and could strand a partial published object.**
   `openat(O_CREAT | O_EXCL)` created the final destination name before
   `write_all`/`sync_all`, so a concurrent reader could observe zero/partial
   bytes, and a write failure or crash between create and write left a
   destination name that every retry would only ever see as
   `DestinationExists`. Rewrote `create_only` to write and `fsync` an owned,
   unguessable same-directory temp file first
   (`.underlay-tmp.<pid>.<random-u64>`, `O_EXCL | O_NOFOLLOW`), then publish
   it to the final name with `linkat` (atomic, create-if-absent, same
   collision semantics as the old direct `O_CREAT | O_EXCL`), then unlink
   the temp name — on every path, including collision and write-failure, so
   only the caller's own temp is ever removed. New proof:
   `local_atomic_publish_tests.rs` — the destination is unreadable before
   the call and immediately fully readable after with no temp residue; a
   collision preserves the incumbent and leaves no temp residue; a
   pre-seeded stale temp file (simulating a hypothetical crashed prior
   attempt) is never touched and does not block a fresh publish, proving a
   retry after failure is not permanently poisoned; a concurrent 8-writer
   race still yields exactly one publisher with no temp residue on either
   path.

The reviewer noted the first round's static pre-existing-parent-symlink
tests were useful but not the actual swap-after-construction interleaving
requested. The new base-pin tests are the real thing: the symlink
replacement happens strictly after the adapter is constructed and the
pinned descriptor is already open, using a real `fs::rename` plus
`symlink`, not a pre-seeded static fixture.

### Re-Validation At This Repaired Head

- `cargo test -p underlay-blob --all-features`: 72 passed, 0 failed (was
  66; +6 from the two new test files above).
- `cargo fmt --all -- --check`: clean.
- `cargo check --workspace --all-features`: clean.
- `cargo clippy --workspace --all-features --all-targets -- -D warnings`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc -p underlay-blob --all-features --no-deps`:
  clean.
- `effigy qa:docs`, `effigy qa:northstar`: clean.
- `effigy doctor`: unchanged — same single pre-existing `scan.comment-ratio`
  finding on `lib.rs` (this repair touched no doc comments there);
  `scan.god-files` stayed warning-tier only.
- `git diff --check`: clean.
- `cargo test --workspace --all-features --no-fail-fast`: exactly one
  failure, the already-documented pre-existing
  `underlay-http-client::tests::invalid_user_agent_fallback_retains_default_timeout`
  flake. The `underlay-http::http_config::tests::test_local_defaults` flake
  noted in the first repair's re-validation did not reproduce this run,
  consistent with it being intermittent parallel-execution interference
  rather than a deterministic failure; still not caused by this branch
  (`git status --short` shows no `underlay-http`/`underlay-http-client`
  changes on this branch at any point in this PR).

## Third Review Repair (re-review on exact head `7e85740b21c0dab9824eb35f38752ebef48e1c91`)

Three coupled correctness claims remained open
([review](https://github.com/inflatable-cookie/underlay/pull/23#issuecomment-5510251518)).
Accepted work (S3, and the descriptor-relative per-key traversal added in
the previous repair) was preserved unchanged; additive trait/default shape
and existing mutable APIs unchanged throughout.

1. **Construction still pinned through a mutable lexical path.**
   `LocalAdapter::new` canonicalized the base path, then
   `open_pinned_base_dir` called a single `open(canonical_base, ...)` with
   no `O_NOFOLLOW` — any canonical-path component replaced with a symlink
   in the window between those two steps would be silently followed,
   pinning the attacker's directory instead of the real one. This was a
   real gap in the previous repair's own claim ("containment guarantee is
   each step's own syscall, not a check performed before it") — the
   *construction* step was still exactly that check-then-act pattern.
   Fixed by making `open_pinned_base_dir` walk the canonical absolute path
   one component at a time from an owned root (`/`) descriptor with
   `openat(O_DIRECTORY | O_NOFOLLOW)`, reusing the same
   `open_dir_component` helper the per-key traversal already used. New
   proof: `test_open_pinned_base_dir_refuses_a_component_swapped_between_canonicalize_and_open`
   reproduces the exact two-step sequence `LocalAdapter::new` performs
   (`canonicalize()`, then the pinning open) with a real symlink swap
   directly in between, and asserts the swapped call refuses rather than
   silently pinning the attacker's directory — a genuine construction-time
   interleaving, not a static fixture.
2. **The final `linkat` was atomic but not durably committed.** `sync_all`
   covered the temp file's bytes, but `linkat` only mutates the parent
   directory's metadata and was never itself `fsync`ed, so a crash right
   after a successful publish could still lose the new directory entry.
   Added an `fsync` of the parent directory after a successful `linkat`.
   Its failure is logged via `tracing::warn!`, never returned: the publish
   already succeeded and is visible the moment `linkat` returns 0 (POSIX
   local-filesystem `link()`/`linkat()` success/failure is authoritative —
   a nonzero return means no link was created), so treating a downstream
   `fsync` failure as a call failure would risk exactly the poisoned-retry
   hazard this design exists to avoid. This is now stated explicitly as a
   best-effort local-filesystem durability improvement, not a
   cross-filesystem (network/overlay) crash guarantee — genuine
   crash-survival cannot be proven by an in-process unit test, so the claim
   is scoped to what is actually verifiable rather than overstated.
3. **Temp cleanup was silently best-effort while the docs/log implied
   guaranteed no-residue.** `cleanup_temp` discarded every `unlinkat`
   result, but the module docs and the previous delivery log said the temp
   was "removed on every path" / left "no residue" without qualification.
   Corrected every claim (module doc, function docs, this log) to state
   plainly that cleanup is best-effort, and that a leftover temp file
   changes nothing about destination correctness, collision detection, or
   future retries, because both are keyed on the final name only, never a
   temp name. `cleanup_temp` now logs non-`ENOENT` failures via
   `tracing::warn!` for operator visibility. Confirmed by code inspection
   that a cleanup or `fsync` failure can never cause `create_only_sync` to
   report an error once `linkat` has already succeeded — both follow-up
   steps are called as bare statements after the point where the function
   commits to returning `Ok(())`, so their outcome cannot propagate into
   the returned `BlobResult` by construction. The existing
   `test_put_bytes_create_only_ignores_and_never_touches_a_stale_foreign_temp_file`
   (from the previous repair) remains the deterministic proof that a
   residue temp file never poisons a later retry.

### Re-Validation At This Repaired Head

- `cargo test -p underlay-blob --all-features`: 73 passed, 0 failed (was
  72; +1 construction-swap oracle).
- `cargo fmt --all -- --check`: clean.
- `cargo check --workspace --all-features`: clean.
- `cargo clippy --workspace --all-features --all-targets -- -D warnings`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc -p underlay-blob --all-features --no-deps`:
  clean.
- `effigy qa:docs`, `effigy qa:northstar`: clean.
- `effigy doctor`: unchanged — same single pre-existing `scan.comment-ratio`
  finding on `lib.rs`; `scan.god-files` gained `bounded.rs` as a new
  warning-tier (not error-tier) entry from this repair's growth, still
  warning-only overall.
- `git diff --check`: clean.
- `cargo test --workspace --all-features --no-fail-fast`: fully green this
  run (`exit 0`), including the previously-documented
  `underlay-http-client` timing flake, which is inherently intermittent —
  its earlier failure and this run's pass are both consistent with the
  same known pre-existing flakiness, not a change caused by this branch.

## Next Task

Push the repaired branch and report the new exact head to the orchestrator
for re-review. Release and consumer adoption stay blocked per `g11.001`.
