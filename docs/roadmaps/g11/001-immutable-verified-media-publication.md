# g11.001 - Immutable Verified Media Publication

Status: complete
Owner: repo maintainers
Created: 2026-09-02
Closed: 2026-09-02
Depends on: `g10` closed
Governing refs: `docs/contracts/023-release-and-compatibility-rollout.md`,
`docs/contracts/040-storage-blob-and-media-systems.md`,
`docs/contracts/050-media-library-and-usage.md`,
`docs/specs/immutable-verified-blob-promotion.md`

## Outcome

The shared additive, fail-closed bounded-capture and create-only
byte-write primitives plus the verified staging-to-published promotion
helper (with positive token-bound destination ownership proof and
recovery) exist for the built-in S3 and local adapters, existing
mutable APIs are preserved, and the surface is published as the
validated `v0.9.7` tag before any consumer relies on owned recovery.

## Delivery

- Shared primitive: PR #23 merged at `27bde7b4` (exact head
  `7f8a0a81`).
- `v0.9.6` released at `4f6d7552`; consumer proof repointed at the tag.
- Owned-promotion recovery: PR #25 merged at `c8378e6b`.
- `v0.9.7` released at `8a7ce84b`.
- Prior milestone wrapper `g11.001` and its completed batch cards
  001–004 collapsed here; their full bodies remain in git history.
  Logs: `docs/logs/2026-09/02-194057-g11-002-underlay-v0-9-6-release.md`,
  `docs/logs/2026-09/02-222309-g11-003-owned-verified-promotion-recovery.md`,
  `docs/logs/2026-09/02-232800-g11-004-underlay-v0-9-7-release.md`.

## Invariant

A media version may become ready/current only from bytes captured once
and within a configured bound by the server, validated for size,
declared MIME, and magic bytes, hashed by the server, and published
under a distinct destination key using exclusive create. The returned
destination key, actual byte size, validated MIME, and lowercase
SHA-256 describe those same captured bytes. The client upload key is
staging identity only and never becomes the published object identity.

## Compatibility and release

Impact class: `additive`. Existing `BlobAdapter` implementations kept
compiling through a fail-closed default for the new create-only
operation. Built-in S3 and local adapters implement it. Existing
mutable `put_bytes`, `get_bytes`, `finalise_upload`, and upload DTOs
remain available. Applications adopt only after the new tag exists.

## Limits

Ordinary collision refusal was not weakened; every unproven collision
stays `BlobError::DestinationExists` and the caller decides. No
convergent retry was added.

## Next task

Run `g11.002` for five-consumer adoption from `v0.9.7`, Underlay
Reference first. `g11.003` closes the fleet once all consumer lanes
merge.
