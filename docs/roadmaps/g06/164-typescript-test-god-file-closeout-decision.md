# g06.164 - TypeScript Test God-File Closeout Decision

## Why

`g06.163` cleared the last TypeScript source god-file warning. `effigy doctor`
now passes with only test-file god-file warnings.

The lane needs a deliberate decision: split test files now, defer them as
acceptable warning-only backlog, or close the TypeScript structural work with a
clear rationale.

## Goal

Classify the remaining test-file warnings and decide whether they block the
reference-grade TypeScript closeout.

## Scope

In scope:

- inspect the 9 remaining test-file `scan.god-files` warnings
- classify which tests should remain large and which may need future splits
- decide whether to continue test splitting or close the TypeScript structural
  lane
- record final doctor state

Out of scope:

- source file splitting
- public API changes
- consumer-app changes
- Rust cleanup

## Acceptance Criteria

- every remaining test-file warning is classified
- any required split is represented as a bounded follow-up card
- intentional warning backlog is documented with rationale
- `effigy doctor` state is recorded

## Consumer Upgrade Impact

Expected impact: none.

This is a decision batch.

## Current State

`g06.164` is ready.

## Next Task

Execute `g06.164`: TypeScript test god-file closeout decision.
