# g06.169 - Rust Runtime Security Surface Re-Audit

## Why

The TypeScript structural lane and fleet compatibility proof are closed. The
next reference-grade pressure is the Rust runtime/security side: auth, HTTP,
config, jobs, media/blob, audit, and construction-boundary APIs.

Earlier g06 work split major Rust structure. This card re-audits the current
state after those changes and after the consumer proof.

## Goal

Identify the next bounded Rust runtime/security hardening target without
starting a broad rewrite.

## Scope

In scope:

- inspect current Rust crate boundaries and public APIs
- review security-sensitive crates for construction-boundary and extension
  risks
- check current Rust validation and doctor state
- classify any remaining risks as Underlay-side, consumer-side, or warning-only
- choose the next bounded implementation card

Out of scope:

- broad crate reorganization
- new consumer migrations unless a Rust compatibility issue is proven
- TypeScript source splitting
- release publishing

## Acceptance Criteria

- Rust runtime/security surfaces are classified by risk
- any public API or construction-boundary weakness is named with file/crate
  evidence
- next implementation card is bounded
- validation state is recorded

## Consumer Upgrade Impact

Expected impact: audit only.

Any breaking Rust change must get its own compatibility card.

## Current State

`g06.169` is ready.

## Next Task

Execute `g06.169`: Rust runtime security surface re-audit.
