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

`g06.164` is complete.

`effigy doctor` reports 9 remaining `scan.god-files` warnings, all in test
files:

- `ts/tests/nightfire/utils.test.ts`: broad Nightfire value/block/form-data
  behavior matrix; defer until Nightfire source work changes this surface.
- `ts/tests/client/sveltekit.test.ts`: integration-style SvelteKit adapter
  coverage; defer until client adapter work changes this surface.
- `ts/tests/patterns/forms.test.ts`: form state/enhance/helper behavior
  coverage; defer until forms source work changes this surface.
- `ts/tests/patterns/i18n.test.ts`: data-driven formatting matrix; keep as
  one audit file unless i18n source structure changes.
- `ts/tests/nightfire/summary-transform.test.ts`: summary transform behavior
  matrix; defer until Nightfire editor work changes this surface.
- `ts/tests/server/csp.test.ts`: CSP/security-header behavior matrix; keep
  together for security audit readability.
- `ts/tests/patterns/slugify.test.ts`: slug generation/validation/reserved-word
  matrix; keep as one behavior file unless slug source structure changes.
- `ts/tests/client/http/auth.test.ts`: auth refresh/concurrency matrix; keep
  paired with the split HTTP source modules for now.
- `ts/tests/client/useAuth.test.ts`: auth-store state-machine coverage; defer
  until auth-store source work changes this surface.

Decision: no immediate test-file split is required for reference-grade
TypeScript source closeout. The remaining warnings are intentional
warning-only backlog, not source modularity blockers. Split them only when the
paired source surface changes, when a file becomes hard to extend, or if future
doctor policy makes test-size warnings blocking.

## Next Task

Execute `g06.165`: TypeScript structural closeout audit.
