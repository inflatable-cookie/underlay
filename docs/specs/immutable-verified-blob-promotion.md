# Immutable Verified Blob Promotion

Status: active
Owner: repo maintainers
Created: 2026-09-02
Roadmap: `g11.001`

## Required Surface

Keep `BlobAdapter` source-compatible by adding bounded-capture and create-only
byte-write methods with default typed fail-closed errors. Built-in S3 and local
adapters override both. Add a verified-promotion extension that accepts
distinct staging and destination `BlobObjectKey` values, a declared MIME, and
`BlobUploadConfig`.

The helper must:

1. reject equal staging and destination keys;
2. perform one bounded read capped at the configured maximum plus one sentinel
   byte; refuse an over-limit source without buffering the remainder;
3. retain the accepted returned byte vector as the sole verification and
   publication payload;
4. validate actual length, configured MIME allowlist, and magic bytes;
5. compute lowercase SHA-256 from that vector;
6. create the destination exclusively from that same vector;
7. return a public result containing the destination `StoredObject` and derived
   SHA-256;
8. preserve the staging object for app-owned cleanup/recovery policy.

The worker may improve exact names, but not weaken this behavior.

## Exclusive Create

- S3: one conditional PutObject to the destination (`If-None-Match: *` or the
  SDK's equivalent). Treat every documented precondition/conflict status as a
  typed collision. Never HEAD then PUT, retry without the condition, or render
  provider diagnostics containing secrets.
- Local: capture and traverse through pinned descriptors with no-follow
  semantics, write and sync an owned same-directory temporary file, then
  publish it atomically with create-only `linkat`. Never expose a partial final
  name, truncate an existing file, or follow a symlink. Post-commit directory
  sync and temp cleanup remain best-effort and may not turn a successful
  exclusive publish into a reported failure.
- Default/no-op/custom adapters: refuse as unsupported unless they implement
  genuine bounded capture and exclusive create.

## Bounded Capture

- S3: stream at most `max_bytes + 1` into the owned vector, then stop. Do not
  use an unbounded collect before the limit decision.
- Local: open and read through a descriptor under the same cap. Refuse
  symlinks and non-regular files without blocking.
- The old unbounded `get_bytes` method remains available for compatibility but
  is not used by verified promotion.

## Collision And Retry

A first create returns success. A destination collision never overwrites. The
API must distinguish collision from transport/internal failure. If the worker
adds convergent retry, it may accept only after comparing destination bytes to
the captured payload; different or unprovable bytes remain a conflict. Metadata
or ETag equality alone is not proof.

## Public Result

The result must bind:

- provider, bucket, and destination key;
- actual captured byte size;
- validated declared MIME;
- lowercase 64-character SHA-256 of the captured bytes;
- backend ETag only as supplemental metadata, never the cross-adapter identity.

## Compatibility

This is an additive pre-1.0 minor release. Existing mutable upload/read/finalise
methods and existing callers remain unchanged. The old verified helper stays
available; contracts and upgrade notes must state that it does not establish
immutable publication.

## Required Proof

- oversized S3/local source proves only max plus sentinel was retained;
- mutable staging adapter swaps bytes after capture;
- concurrent local destination writers;
- local existing file, symlink, directory, and containment attempts;
- S3 request fixture proves one conditional PUT and collision mapping;
- custom adapter compiles without implementing the new method and refuses;
- client-declared wrong digest is irrelevant to the shared helper because no
  client digest enters it;
- no raw backend error or credential-shaped fixture reaches the public error.

## Consumer Chain

After merge: cut the next validated tag; resume Contact Patch Card 015; then
adopt in Underlay Reference, Compli Me, Acowtancy, and Songsprout according to
their target-owned cards and real finalisation oracles.

## Next Task

Execute `g11.001` card 002: prepare, validate, and publish Underlay `v0.9.6`.
