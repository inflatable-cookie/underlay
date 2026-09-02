# g11 - Immutable Verified Media Publication

Status: active
Owner: repo maintainers
Started: 2026-09-02

## Current Generation

`g11` owns the shared repair and five-consumer rollout for media finalisation.
Contact Patch Bughunt Card 015 proved that the current split `get_bytes` and
`finalise_upload`/`head` surface cannot bind inspected bytes to a later
ready/current database transition. The operator confirmed the invariant applies
to Underlay Reference, Contact Patch, Compli Me, Acowtancy, and Songsprout.

## Roadmap Sequence

1. [ ] [`g11.001`](001-immutable-verified-media-publication-and-fleet-rollout.md)
   — shared primitive, release, five consumer adoptions, and fleet closeout
   (`active`)

## Queue

- Complete: `g11.001` Card 001, merged in PR #23 at `27bde7b4`.
- Complete: Card 002, `v0.9.6` released at `4f6d7552`.
- Complete: Card 003, token-bound owned promotion recovery, merged as PR #25.
- Ready: Card 004, the approved `v0.9.7` release.
- Paused: affected consumer adoption until the validated `v0.9.7` tag exists.
- Blocked: fleet closeout until all applicable consumer lanes merge.

## Dependencies And Parallelism

The shared implementation and release are serial. Consumer adoption starts
only from a validated released tag. After that tag exists, independent consumer
repositories may run in parallel, subject to their own shared-authority and
same-repo merge ordering. Contact Patch Card 015 resumes on its retained
agent/workspace rather than creating a duplicate lane.

## Next Task

Execute Card 004 and publish `v0.9.7` only after exact-SHA CI and every
configured release gate pass.
