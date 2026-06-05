# g06.019 - Reference-Grade Architecture Reset Inventory

## Why

The fresh-start audit showed the ideal Underlay shape is stricter than the
current crate layout. The six known consumers are not in production, so this is
the right time to take controlled breaking changes instead of preserving
compatibility residue.

This card prepares the reset. It does not start moving code yet.

## Goal

Produce the current-to-target map needed to reshape Underlay into a
reference-grade platform without uncontrolled consumer breakage.

## Scope

In scope:

- map each Rust crate to the target platform, adapter, or tooling family
- classify current public roots as keep, narrow, move, or retire
- inventory current dependency edges between platform, adapter, and tooling
  crates
- inspect the six consumer roots for Underlay Rust crate usage and direct
  implementation ownership
- identify the first breaking batch that should update `underlay-reference`
  and any directly affected consumers
- update the architecture/package-map docs with the target reset posture

Out of scope:

- code movement
- consumer repo mutations
- package publishing or release execution
- TypeScript package resets unless they block the Rust/platform map

## Contract References

- `001`: working rules
- `023`: release and compatibility rollout
- `122`: Rust public API inventory
- `020-reference-grade-underlay-architecture`: target architecture

## Acceptance Criteria

- crate family map exists for current and target architecture
- consumer dependency matrix exists for the six known apps
- first breaking batch is named and bounded
- compatibility policy is explicit: shim, deprecate, or break-and-update
- docs QA passes

## Consumer Upgrade Impact

Impact: planning only.

No consumer code changes happen in this card. Later `g06` cards may be
breaking and must update affected consumers before completion.

## Current State

`g06.019` is complete.

Completed output:

- target architecture documented in
  `docs/architecture/020-reference-grade-underlay-architecture.md`
- `g06` continued as the active reference-grade reset lane
- `g06.018` superseded by the broader reset lane
- current-to-target crate family map recorded in
  `001-reference-grade-architecture-reset-inventory-artifact.md`
- initial six-consumer dependency readout recorded
- first breaking batch selected as public root diet plus exact consumer import
  matrix

## Next Task

Execute `g06.020`: Public Rust surface diet and consumer import matrix.
