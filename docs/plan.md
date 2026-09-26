# Plan

Updated: 2026-09-26

## Now

1. **Verified media across the fleet** — every consumer's live upload
   finalisation derives the digest server-side and makes ready/current one
   database transition, using the released `v0.9.7` helper. Underlay Reference
   first, then the other four independently; each consumer pins a released tag.
   Then prove all five on a released tag in one fleet check. See Q-002.

## Next

- **Retire the Nightfire compatibility facades** — once every consumer imports
  `nightfire` directly, remove the `underlay-nightfire` crate-name facade and
  the `underlay/nightfire/*` subpaths in a minor release. Needs a caller
  inventory, consumer proof and release authorization. See Q-001.

## Not now

- The six proposed implementation ideas from early research (passkey hooks,
  AI runtime resilience, Zod validation, background jobs, migration
  verification, Nightfire slash commands) — see
  [triage](triage/20260926-000000-unexecuted-research-proposals.md).
