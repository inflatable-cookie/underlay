# g06.027 - Post-Reset Rust Quality Re-Audit

## Why

`g06.019` through `g06.026` changed the Rust architecture shape: concrete
Postgres runtime code moved out of mixed contract crates, the six consumers
moved with it, and active docs now teach the new pattern.

The next useful step is not another planned extraction by default. It is to
re-audit the Rust codebase in its current shape and decide what still blocks
reference-grade quality.

## Goal

Run a fresh Rust quality audit after the adapter reset.

## Scope

In scope:

- inspect the current Rust crate graph and public API surface
- re-check modularity, dependency direction, safety boundaries, and extension
  points
- review security-sensitive areas for obvious construction or SQL risks
- classify remaining structural scanner findings as blocker, follow-up, or
  accepted backlog
- recommend the next bounded execution batch only from current evidence

Out of scope:

- code movement before the audit evidence exists
- consumer app edits unless the audit finds a current shared-surface break
- TypeScript/Svelte surface audit
- release execution or publishing

## Contract References

- `001`: working rules
- `020`: HTTP transport and server boundary
- `030`: auth and session systems
- `040`: storage, blob, and media systems
- `060`: jobs, events, and operator systems
- `122`: Rust public API inventory

## Acceptance Criteria

- audit findings are ordered by severity and tied to files/contracts
- remaining security risks are explicit
- remaining modularity risks are explicit
- consumer-facing risk is classified
- next bounded batch is opened only if justified

## Consumer Upgrade Impact

Impact: audit only.

No consumer code change is intended in this card.

## Current State

`g06.027` is complete.

The audit found no immediate high-severity security blocker. The next bounded
reference-grade batch is typed operator table config for audit and
security-alert helpers.

See
[`027-post-reset-rust-quality-re-audit-artifact.md`](027-post-reset-rust-quality-re-audit-artifact.md).

## Next Task

Execute `g06.028`: typed operator table config.
