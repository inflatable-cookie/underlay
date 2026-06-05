# g06.015 - Rust Platform Transition Release-Note Handoff

## Why

`g06.014` confirmed the Rust platform-contract transition is ready for
release-note handoff.

The remaining work is not another broad refactor. It is to turn the completed
generation evidence into concise release and consumer-upgrade messaging.

## Goal

Prepare the release-note handoff for the `g06` Rust platform-contract
transition.

## Scope

In scope:

- summarize the Rust public API and contract changes
- call out additive safe-boundary APIs and internal module splits
- call out the one known breaking Rust trait surface
- state that the named six-consumer family needs no app update
- preserve validation evidence from `g06.014`
- classify remaining scanner findings as backlog

Out of scope:

- release execution or publishing
- another Rust refactor batch
- TS validation drift cleanup
- consumer feature migrations

## Contract References

- `001`: working rules
- `023`: release and compatibility rollout
- `030`: auth and session systems
- `040`: storage, blob, and media systems
- `120`: tooling, testing, and contract artifacts
- `122`: Rust public API inventory

## Acceptance Criteria

- release-note handoff says what changed and why
- breaking-change section names `SessionStore::rotate_session_if_current`
- consumer-impact section says no named app update is required
- validation section cites `g06.014`
- remaining scanner backlog is not presented as release-blocking

## Current State

`g06.015` is ready.

## Next Task

Execute `g06.015`: Rust platform transition release-note handoff.
