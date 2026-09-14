# g13 - Poodle 0.4.1 Producer Release

Owner: repo maintainers
Started: 2026-09-14

## Current Generation

`g13` is the operator-directed producer lane for published Poodle
`0.4.1`. Acowtancy Market `g05.070` is blocked until an accepted
Underlay tag exists whose package metadata declares Poodle `0.4.1`;
Underlay `v0.9.8` still declares `poodle-svelte` `0.3.0`. This
generation moves the one committed pin, regenerates the root lock,
absorbs only forced compatibility fallout, and ships the validated
release tag Market needs.

This README owns the `g13` roadmap and approved frontier. The
`g13.NNN` files below are the sole executable planning units; there is
no milestone wrapper or nested `batch-cards/` hierarchy.

## Task Sequence

1. [ ] [`g13.001`](001-poodle-0-4-1-producer-release.md) — pin
   `poodle-svelte` exact `0.4.1`, regenerate `bun.lock`, absorb forced
   fallout, release `v0.9.9` with upgrade notes (`ready`)

## Approved Frontier

Ready: `g13.001`. Nothing else is queued in this generation; it closes
once the `v0.9.9` tag is validated and its tag, commit, and package
bytes are reported back to Market `g05.070`.

## Dependencies And Parallelism

`g13` is independent of `g11` media rollout and `g12` Nightfire
extraction: no shared mutable paths, no shared acceptance oracle.
`g11.002` (five-consumer adoption from `v0.9.7`) and `g12.001`
(Card 278 lane) run in parallel and are unaffected — this lane changes
no media or Nightfire semantics. Market `g05.070` is serial behind the
produced tag; its blocked worker unblocks itself once the tag exists.
Longhorn's `0.4.1` adoption is a parallel producer lane in its own
repository with no edge in either direction; the Desktop join happens
through Market, not through this lane.

## Next Task

Run `g13.001`.
