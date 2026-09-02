# g11.001 Card 003 Owned Verified Promotion Recovery — Delivery

Date: 2026-09-02
Roadmap: `g11.001`, card 003
Handoff: `docs/handoffs/20260902-205715-owned-verified-promotion-worker.md`
Branch: `worker/owned-verified-promotion-v097`
PR: https://github.com/inflatable-cookie/underlay/pull/25
Implementation: `3c20cd5c1ed0bfcebba3696195187acf0c8371cc`

## What Shipped

`underlay-blob` gains additive token-bound destination ownership:

- `OwnershipToken` — opaque high-entropy secret, redacted `Debug`, no
  `Display`. Minimum 32 bytes. Never written to object metadata, errors,
  logs, URLs, or DTOs.
- `OwnedDestinationAuthority` — provider, bucket, and destination key the
  consumer persists before create.
- `OwnedPublicationFacts` — one-way verifier plus server-derived SHA-256,
  size, and validated MIME.
- `BlobAdapter::put_bytes_create_only_owned` — fail-closed default
  `Unsupported`. Built-in S3 and local adapters implement it.
- `BlobAdapterPromotionExt::promote_verified_owned` — same capture/validate
  path as `promote_verified`, then owned exclusive create.
- `BlobAdapterPromotionExt::recover_owned_publication` — `head` only. Matching
  verifier plus complete facts succeed. Every unproven incumbent is
  `BlobError::DestinationExists`. Wrong provider/bucket is `InvalidKey`.
  Missing destination is `NotFound`. Never reads staging.

S3: reserved user metadata rides on the same conditional `PutObject`
(`If-None-Match: *`). No HEAD-then-PUT.

Local: xattrs (`user.underlay.owned.v1.*`) attach to the unpublished temp
inode after the write and before `fsync`/`linkat`. `head` returns those
facts in `ObjectInfo.metadata`. A filesystem that cannot set xattrs refuses
the owned path as `Unsupported` and does not publish.

Existing v0.9.6 methods, `NoopAdapter`, and third-party adapters remain
source-compatible.

## Stop-Condition Probes

Both required before implementation, both cleared:

1. **S3 atomic metadata + exclusive create.** `PutObject` already exposes
   `.metadata(k, v)` and `.if_none_match("*")` on one request. Existing
   `initiate_upload` already sends user metadata. Fixture proof:
   `put_bytes_create_only_owned_sends_one_conditional_put_with_reserved_metadata`
   asserts exactly one PUT, `If-None-Match: *`, and the four
   `x-amz-meta-underlay-owned-v1-*` headers. Recovery fixture is HEAD-only.
2. **Local atomic metadata + exclusive create.** On this APFS volume,
   `fsetxattr` on an unpublished fd succeeds; the final name is still
   absent; `link` then exposes the same inode and the same xattr. Committed
   as `xattrs_are_visible_on_the_unpublished_inode_before_the_final_name`.
   Production path sets xattrs before `sync_all` and `linkat`; xattr failure
   cleans up the temp name and returns `Unsupported` without publishing.

Neither stop condition triggered.

## Proof Summary (Card 003 review oracle)

| Oracle | Proof |
| --- | --- |
| Pre-create intent + foreign identical destination | Fake + local: unowned create of identical PNG bytes; recovery with a live token is `DestinationExists`; incumbent bytes and empty metadata preserved |
| Post-create/pre-database crash; staging missing, mutated, or symlinked | Fake: delete staging, recover from `head` only (`bounded_reads == 0`). Local: delete staging, then replace it with a symlink to hostile bytes; recovery still returns the published SHA-256 and destination bytes |
| Wrong token, provider, bucket, key, missing/malformed/inconsistent metadata | Fake: wrong token and incomplete/malformed digest/size/MIME → `DestinationExists` without token disclosure. Wrong provider/bucket → `InvalidKey` without calling `head`. Wrong missing key → `NotFound`. S3: wrong-token HEAD fixture → `DestinationExists` |
| Two writers and ordinary retries | Local: eight concurrent owned creates, exactly one winner with complete metadata, seven `DestinationExists`. Fake: second `promote_verified_owned` is `DestinationExists`; matching token then recovers |
| S3 one conditional PUT with reserved facts | Replay fixture: one PUT, `If-None-Match: *`, four reserved metadata headers, no prior HEAD |
| Local metadata before final-link visibility | xattr unit test: facts readable on the unpublished inode while the final name does not exist; after hard-link, same facts. Adapter test: `head` and bounded read both succeed after owned create |
| Existing adapters compile and refuse; v0.9.6 stays green | `NoopAdapter` owned create is `Unsupported`. Full `underlay-blob --all-features` suite: 102 passed, including every pre-existing promotion/S3/local test |

## Public API Shape

New crate-root exports: `OwnershipToken`, `OwnedDestinationAuthority`,
`OwnedPublicationFacts`. New `BlobAdapter` method with fail-closed default:
`put_bytes_create_only_owned`. New `BlobAdapterPromotionExt` methods:
`promote_verified_owned`, `recover_owned_publication`. No new `BlobError`
variants. Existing public items unchanged.

## Consumer Upgrade Notes

- Impact class: `additive`
- Affected consumers: live upload-finalisation paths that need restart
  recovery after exclusive create (Underlay Reference first; other four
  after `v0.9.7`)
- Required actions:
  - wait for Card 004 to publish `v0.9.7`
  - persist a ≥32-byte opaque token and destination authority before create
  - call `promote_verified_owned`; on process loss call
    `recover_owned_publication` instead of adopting identical bytes
- Validation: consumer compile against the released tag; do not pin this
  commit
- Deprecation/removal date: n/a
- Reference docs: Contract 040 owned-recovery section; media-upload-pipeline
  Finalize phase; this log

## Validation

- `cargo fmt --all --check` and `git diff --check` passed
- focused `underlay-blob --all-features` tests: 102 passed (v0.9.6 suite still green)
- `effigy rust:check` and `effigy rust:clippy` (`-D warnings`) passed
- `effigy rust:test` stopped on the inherited `underlay-http-client` timeout flake;
  `cargo test --workspace --all-features -- --skip invalid_user_agent_fallback_retains_default_timeout` passed
- `effigy qa` passed after `effigy bootstrap:deps` (`bun install` in a clean worktree)
- `effigy doctor`: inherited `scan.comment-ratio` error on `underlay-blob/src/lib.rs`;
  `scan.god-files` remains warnings only after splitting owned promotion tests

## Inherited / Pre-Existing Failures

1. **`effigy doctor` `scan.comment-ratio`** on
   `rust/crates/underlay-blob/src/lib.rs` (`ratio=2.86`, 63 comment / 22 code).
   Inherited crate docs; `AGENTS.md` already notes Underlay's doctor scan
   backlog. This branch added a short owned-promotion pointer and slightly
   lowered the ratio from 2.95.
2. **`effigy doctor` `scan.god-files`** warnings. Unrelated files plus existing
   blob adapter modules. Owned promotion tests were split so they do not
   introduce a new error-threshold file.
3. **`underlay-http-client` timeout flake.**
   `tests::invalid_user_agent_fallback_retains_default_timeout` failed twice
   with `started.elapsed() >= DEFAULT_TIMEOUT` in ~0s. Documented in the
   g11.001 delivery log; this branch does not touch that crate.

## Review Repair (PR #25 exact-head `9ec8e2ee`)

Three findings from the posted review, all repaired on this branch:

1. **Verifier now binds destination authority.** SHA-256 over
   `domain || u32be(len)||provider || u32be(len)||bucket || u32be(len)||key || u32be(len)||token`.
   Copied metadata cannot recover under a new key/provider/bucket. Same token
   on two destinations produces distinct verifiers. Concatenation ambiguity
   (`("ab","c")` vs `("a","bc")`) is refused. Per-publication fresh tokens of
   at least 32 bytes are stated as operational hygiene; uniqueness is not the
   proof.
2. **Local xattr reads are bounded.** `getxattr` `ERANGE` is treated as
   unproven, never as a size. Oversized reserved xattrs omit that key so
   `head`/`exists` keep v0.9.6 success and recovery/owned-create stay
   `DestinationExists`.
3. **Unix cfg fallback compiles as a complete function** on
   `not(macos/ios/linux/android)` and returns `Unsupported`. The fallback
   bodies are type-checked in
   `unsupported_unix_cfg_arm_typechecks_and_fails_closed`. This is not a
   FreeBSD/illumos kernel execution claim.

## Next Task

Re-review of PR #25. Card 004 stays serial until that PR merges.
Do not cut `v0.9.7` from this lane.
