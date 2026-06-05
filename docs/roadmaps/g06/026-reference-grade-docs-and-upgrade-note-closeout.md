# g06.026 - Reference-Grade Docs And Upgrade-Note Closeout

## Why

`g06.019` through `g06.025` reset the Rust public surface toward explicit
contracts and concrete adapter crates. The code and consumer rollout are now
proved, but the generation needs one final docs closeout so future consumers
learn the new shape without reading roadmap history.

## Goal

Close the reference-grade Rust adapter reset with current docs, upgrade notes,
and front-door guidance.

## Scope

In scope:

- sweep active Rust guides for stale contract/adapter language
- update package-map and architecture front doors if they still describe mixed
  contract/adapter crates
- keep roadmap artifacts as history, not active guidance
- ensure `190-upgrade-compatibility.md` points to the final adapter upgrade
  note
- run docs QA and targeted Rust checks if docs imply public examples

Out of scope:

- another Rust code movement batch
- consumer source edits unless a stale guide exposes a real missed migration
- release execution or publishing
- TypeScript package-boundary work

## Contract References

- `001`: working rules
- `023`: release and compatibility rollout
- `030`: auth and session systems
- `040`: storage, blob, and media systems
- `060`: jobs, events, and operator systems
- `122`: Rust public API inventory

## Acceptance Criteria

- active guides teach `underlay-jobs` plus `underlay-jobs-postgres`
- active architecture/package docs list extracted adapter crates accurately
- upgrade compatibility guide contains the final adapter-split note
- roadmap front doors mark `g06.026` complete or name the next real task
- validation passes or failures are classified

## Consumer Upgrade Impact

Impact: docs and upgrade-note closeout.

No new consumer code change is intended.

## Current State

`g06.026` is complete.

Active docs now teach the explicit contract-plus-adapter shape:

- `underlay-jobs` plus `underlay-jobs-postgres`
- `underlay-media` plus `underlay-media-postgres`
- `underlay-auth` plus `underlay-auth-postgres`

Updated surfaces:

- package map
- system inventory
- Rust API foundation
- jobs/events/operator contract
- background jobs guide
- auth security alerting guide
- upgrade compatibility guide

Validation passed:

- `effigy qa:docs`

## Next Task

Execute `g06.027`: post-reset Rust quality re-audit.
