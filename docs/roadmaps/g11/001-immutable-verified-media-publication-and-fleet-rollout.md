# g11.001 - Immutable Verified Media Publication And Fleet Rollout

Status: active
Owner: repo maintainers
Created: 2026-09-02
Depends on: `g10` closed
Governing refs: `docs/contracts/023-release-and-compatibility-rollout.md`,
`docs/contracts/040-storage-blob-and-media-systems.md`,
`docs/contracts/050-media-library-and-usage.md`

## Problem

The current verified-upload helper reads mutable storage and metadata through
separate operations. A same-key overwrite can preserve size and MIME while
changing bytes between inspection and publication. An optional ETag does not
close this for every adapter, and local storage has none. Applications can then
persist a client-supplied digest or publish bytes other than those inspected.

The v0.9.6 consumer proof exposed a second boundary: after exclusive create but
before an application database commit, intent, destination identity, and byte
equality cannot distinguish the application's object from a foreign incumbent.
Safe restart recovery needs positive ownership evidence attached atomically to
the destination.

## Invariant

A media version may become ready/current only from bytes captured once and
within a configured bound by the server, validated for size, declared MIME,
and magic bytes, hashed by the
server, and published under a distinct destination key using exclusive create.
The returned destination key, actual byte size, validated MIME, and lowercase
SHA-256 describe those same captured bytes. The client upload key is staging
identity only and never becomes the published object identity.

## Ordered Delivery

1. Add additive, fail-closed bounded-capture and create-only byte-write
   primitives plus a verified staging-to-published promotion helper for
   built-in S3 and local adapters. Preserve existing mutable APIs.
2. Review and merge the exact implementation head.
3. Cut and validate the next Underlay Git tag with upgrade notes.
4. Add positive, token-bound destination ownership proof and recovery after the
   v0.9.6 consumer counterexample; review and merge it without weakening
   ordinary collision refusal.
5. Cut and validate `v0.9.7` before any consumer relies on owned recovery.
6. Resume Contact Patch Card 015 on its retained worker. Adopt the released
   helper, derive the digest server-side, and make ready/current one database
   transition.
7. Apply the same boundary to Underlay Reference, Compli Me, Acowtancy, and
   Songsprout wherever their live upload-finalisation paths can publish mutable
   or client-described bytes.
8. Prove all five consumers are on the released tag and the invariant, then
   close this roadmap. No consumer may pin an unreleased commit or local path.

## Review Oracle

| Invariant | Counterexample | Expected response | Required proof |
| --- | --- | --- | --- |
| Captured bytes are the publication source. | Replace the staging key after server capture with same-size, same-MIME bytes. | Published destination still contains the captured bytes, or promotion refuses. | Mutable-adapter composition test. |
| Capture is bounded before allocation completes. | Replace staging with an object larger than the configured maximum. | Read at most max plus a sentinel and refuse before retaining the body. | S3/local bounded-read tests. |
| Destination creation is exclusive. | Two writers target the same new destination. | Exactly one creates it; no overwrite or unconditional fallback. | Concurrent local test plus S3 request/fixture proof. |
| Collision cannot forge success. | Seed the destination with different bytes or metadata. | Promotion refuses and preserves the existing object. | Built-in adapter tests. |
| Intent is not ownership. | Persist intent, crash before create, then plant a foreign identical incumbent. | Recovery refuses; intent, key, and byte equality cannot authorize adoption. | S3/local metadata fixtures plus consumer-shaped recovery test. |
| Owned restart is staging-independent. | Crash after exclusive create but before the app DB facts commit; remove or mutate staging. | Matching durable token recovers server-derived facts from destination head without staging access. | Owned-promotion restart test. |
| Retry is explicit. | Crash after destination creation but before the app DB commit. | The documented retry path either proves token-bound ownership and converges or returns a typed conflict; it never overwrites. | Promotion retry test and consumer composition test. |
| Derived metadata is authoritative. | Client submits a valid-shape but wrong digest, wrong MIME, or wrong length. | Persist only server-derived digest/size and validated MIME, or refuse. | Consumer handler/DB oracle. |
| Publication and selection are atomic in the app DB. | Ready update wins but current selection fails. | Neither state commits. | Consumer DB transaction test. |

## Compatibility And Release

Impact class: `additive`. Existing `BlobAdapter` implementations must continue
to compile through a fail-closed default for the new create-only operation.
Built-in S3 and local adapters implement the operation. Existing mutable
`put_bytes`, `get_bytes`, `finalise_upload`, and upload DTOs remain available.
Applications adopt only after the new tag exists.

## Consumer Upgrade Impact

All five current sites own the media family and are affected. Each signs a
client-writable key that can become the published key, and each needs a
server-derived digest plus one atomic ready/current transition. A consumer
with a live upload-finalisation path must move from publishing the presigned
upload key to persisting the immutable destination returned by promotion, and
must use its server-derived digest and metadata. Product DTOs need not change
unless a target repo proves that unavoidable and stops for planning.

| Consumer | Current pin | Confirmed adoption pressure |
| --- | --- | --- |
| Underlay Reference | `v0.9.5` | final key is signed directly; client digest and hard-coded provider/bucket are persisted; ready/current split |
| Contact Patch | `v0.9.4` | Bughunt Card 015 stopped on mutable verification; read failure and client digest can still publish; ready/current split |
| Compli Me | `v0.9.4` | mutable key and client digest are published; preserve Card 011 retry guarantees |
| Acowtancy | `v0.9.4` | repeated mutable reads; staging identity is not durable for cleanup; ready/current split |
| Songsprout | `v0.9.4` | final key is signed directly; client digest is persisted; ready/current split |

## Stop Conditions

- built-in storage cannot provide exclusive create without a new backend or
  infrastructure policy;
- the public trait change would break existing adapter implementations rather
  than remaining additive and fail closed;
- retry semantics require overwriting or trusting metadata without byte proof;
- bounded capture would first buffer an attacker-controlled oversized object;
- a consumer needs a public DTO, migration, retention, or cleanup-policy choice
  not already settled by its own authority;
- release validation or target DB/storage oracles cannot execute.

## Next Task

`v0.9.7` is released at `8a7ce84b`. Resume Contact Patch Card 015 and the
other consumer lanes from that tag, Underlay Reference first.
