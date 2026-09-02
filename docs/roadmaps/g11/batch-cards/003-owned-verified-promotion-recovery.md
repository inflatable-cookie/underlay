# 003 - Owned Verified Promotion Recovery

Status: complete — PR #25 merged at `c8378e6b` (2026-09-02)
Owner: blob worker
Created: 2026-09-02
Roadmap: `g11.001`
Spec: `docs/specs/immutable-verified-blob-promotion.md`
Depends on: `v0.9.6` at `4f6d7552`
Auto-start next card: no

## Objective

Add positive, token-bound proof that an immutable destination was created for a
specific durable consumer version, so restart recovery can succeed without
rereading staging or adopting a foreign incumbent.

## Scope

- additive owned create, promotion, and recovery types and extension methods in
  `underlay-blob`;
- S3 reserved metadata on the same conditional PutObject;
- local reserved metadata attached to the unpublished temp inode before atomic
  link, plus head support for that metadata;
- fail-closed defaults for existing third-party adapters;
- Contract 040, public API inventory, upgrade guidance, changelog, focused
  tests, and one lane log.

Do not cut a release, edit consumers, weaken ordinary collision refusal, expose
tokens in diagnostics, or change existing method behavior.

## Review Oracle

- Pre-create intent crash plus a foreign identical destination: recovery
  refuses and preserves the incumbent.
- Post-create/pre-database crash: with staging missing, mutated, symlinked, or
  unreadable, a matching durable token recovers facts from destination head.
- Wrong token, provider, bucket, key, missing metadata, malformed digest or
  size, and inconsistent MIME: typed refusal, no staging read, no disclosure.
- Copied reserved metadata or object at another key/provider/bucket: recovery
  under the new authority refuses. The same token used on two destinations
  produces distinct verifiers and recovers only at each bound key.
- Oversized or unreadable local reserved xattr: `head`/`exists` keep v0.9.6
  success; owned recovery and owned create collision stay `DestinationExists`.
- Two writers and ordinary retries: one creator; every unproven collision stays
  `DestinationExists`.
- S3: one conditional PUT carries reserved verifier and derived facts; no
  HEAD-then-PUT or fallback.
- Local: metadata attaches before final-link visibility; readers see the object
  and complete ownership facts together, or neither.
- Existing adapters compile unchanged and refuse the owned path when
  unsupported; all v0.9.6 behavior remains green.

## Stop Conditions

- stop if S3 or local cannot atomically publish bytes and reserved metadata;
- stop if the design stores the raw token on the object, leaks it through
  errors/logs/DTOs, trusts key secrecy, or accepts byte equality as ownership;
- stop on a breaking trait change, infrastructure policy, consumer schema
  change, or inability to test local and S3 atomicity;
- stop if the public API cannot remain additive and fail closed.

## Validation

- focused `underlay-blob` owned-promotion, S3, and local tests
- workspace Rust check, Clippy with denied warnings, and tests
- `effigy qa`
- `effigy doctor` with inherited findings identified
- `git diff --check`

## Next Task

Complete. Card 004 released `v0.9.7` at `8a7ce84b`.
Delivery log: `docs/logs/2026-09/02-222309-g11-003-owned-verified-promotion-recovery.md`.
