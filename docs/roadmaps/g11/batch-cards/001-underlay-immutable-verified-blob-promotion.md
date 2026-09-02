# 001 - Underlay Immutable Verified Blob Promotion

Status: ready
Owner: repo maintainers
Created: 2026-09-02
Roadmap: `g11.001`
Spec: `docs/specs/immutable-verified-blob-promotion.md`
Auto-start next card: no

## Objective

Add the smallest additive `underlay-blob` surface that captures staging bytes
once under a hard bound, validates and hashes those bytes, and publishes the
same bytes to a distinct destination through exclusive create.

## Scope

- `underlay-blob` public types and adapter/extension traits;
- S3, local, and no-op/default fail-closed behavior;
- focused unit, concurrent, mutable-adapter, and request-shape tests;
- Contract 040, upgrade guidance, changelog, public API inventory, and one log.

Do not edit a consumer repository, cut a release, or change an app DTO/schema.

## Acceptance Criteria

- [x] existing `BlobAdapter` implementors remain source-compatible;
- [x] built-in S3 and local adapters stop the source read at the configured
      limit plus one sentinel byte; oversized input is never fully buffered;
- [x] built-in S3 and local adapters create the destination only if absent;
- [x] the helper validates actual captured bytes, derives lowercase SHA-256,
      and returns destination identity plus derived metadata;
- [x] source mutation after capture cannot change destination bytes;
- [x] same-destination races yield one creator and no overwrite;
- [x] different-byte and hostile-metadata collisions fail closed;
- [x] retry behavior is typed, deterministic, and never uses unconditional
      overwrite as fallback — no convergent retry was added; a collision
      always returns the typed `BlobError::DestinationExists` and the caller
      decides;
- [x] old mutable methods behave unchanged;
- [x] focused tests, Rust checks, docs/Northstar QA, doctor, and diff check pass
      or record exact inherited failures;
- [x] one PR targets `main`; the worker does not merge.

## Review Oracle

Use `g11.001` and the strict spec. Drive an over-limit replacement, the
same-key/same-size staging swap,
two-writer destination race, pre-seeded different destination, crash/retry,
custom adapter using the default method, local symlink/non-regular target, and
S3 conditional-request counterexamples. Inspect emitted errors for backend or
credential leakage.

## Stop Conditions

Use `g11.001` stop conditions. Also stop if the S3 SDK lacks a destination
create precondition or local exclusive create cannot be made no-follow and
containment-safe with current dependencies.

## Next Task

Worker opens one PR. Orchestrator reviews its exact head; release stays blocked.
