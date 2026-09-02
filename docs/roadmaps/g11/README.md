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

- Ready: `g11.001` card 001, the additive `underlay-blob` primitive.
- Blocked: release and all consumer adoption until card 001 merges.
- Blocked: fleet closeout until all applicable consumer lanes merge.

## Dependencies And Parallelism

The shared implementation and release are serial. Consumer adoption starts
only from a validated released tag. After that tag exists, independent consumer
repositories may run in parallel, subject to their own shared-authority and
same-repo merge ordering. Contact Patch Card 015 resumes on its retained
agent/workspace rather than creating a duplicate lane.

## Next Task

Execute `g11.001` card 001 and stop at its PR for exact-head review.
