# g06.156 - TypeScript Warning Backlog Decision

## Why

`g06.155` cleared TypeScript doctor errors. `effigy doctor` now passes, but it
still reports warning-only TypeScript structural backlog:

- attention markers
- comment-heavy files
- large files

The lane needs a deliberate stop-or-continue decision before spending more
cycles on warning-level cleanup.

## Goal

Classify the remaining TypeScript warning backlog and decide which findings
must be cleaned now for reference-grade quality, which can remain intentional,
and which should become bounded follow-up cards.

## Scope

In scope:

- inspect the remaining doctor warning reports
- classify each warning family by risk and extensibility impact
- decide whether any warning should be promoted into executable cleanup
- record the final TypeScript structural state

Out of scope:

- changing runtime behavior
- broad TypeScript package redesign
- Rust structural cleanup
- consumer-app changes

## Acceptance Criteria

- remaining TypeScript warning families are classified
- any required cleanup is represented as bounded follow-up cards
- intentional warning backlog is documented with rationale
- `effigy doctor` state is recorded

## Consumer Upgrade Impact

Expected impact: none.

This is a classification batch. Any later consumer-visible change needs its own
impact section.

## Current State

`g06.156` is ready.

## Next Task

Execute `g06.156`: TypeScript warning backlog decision.
