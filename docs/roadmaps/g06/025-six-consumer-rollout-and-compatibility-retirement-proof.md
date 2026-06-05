# g06.025 - Six-Consumer Rollout And Compatibility Retirement Proof

## Why

`g06.021` through `g06.024` moved concrete Postgres runtime code out of mixed
contract crates and into adapter crates. The six known consumers were updated
as part of those execution batches, but the generation still needs one focused
closeout proof that no old compatibility imports or feature flags remain.

## Goal

Prove the current consumer family is aligned with the reference-grade Rust
adapter boundary.

## Scope

In scope:

- scan Underlay and the six consumers for retired jobs/media/auth adapter
  import paths and feature flags
- classify any remaining compatibility residue as intentional, deferred, or
  removed
- confirm all consumer manifests use explicit core contract crates plus adapter
  crates where concrete storage is required
- rerun targeted consumer checks only where the scan or lockfile state changed
- update release/compatibility notes

Out of scope:

- new adapter extraction beyond jobs/media/auth-state
- publishing or release execution
- unrelated Rust structural backlog repair
- TypeScript package-boundary work

## Contract References

- `001`: working rules
- `023`: release and compatibility rollout
- `030`: auth and session systems
- `040`: storage, blob, and media systems
- `060`: jobs, events, and operator systems
- `122`: Rust public API inventory

## Acceptance Criteria

- no named consumer depends on old `underlay-jobs` `postgres`, `scheduler`,
  `outbox`, or `full` feature flags
- no named consumer imports concrete jobs Postgres symbols from `underlay_jobs`
- no named consumer imports concrete media Postgres symbols from
  `underlay_media`
- no named consumer imports auth-state Postgres symbols from `underlay_auth`
- lockfile changes from adapter additions are committed with their consumer
  manifests
- validation passes or failures are classified
- the next closeout card remains accurate

## Consumer Upgrade Impact

Impact: compatibility proof.

No new breaking change is intended. Any discovered leftover breakage should be
fixed inside this card or explicitly moved to the next card with evidence.

## Current State

`g06.025` is ready.

## Next Task

Execute `g06.025`: six-consumer rollout and compatibility retirement proof.
