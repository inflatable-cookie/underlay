# g08.014 - Red Unit Suite And Test Gate

Status: complete
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

The unit vitest suite is red on main and is not gated. `bun x vitest run` fails
4 tests in 2 files because `contextualBackLabel` now returns the trimmed label
("Articles") while the tests still expect the "Back to Articles" prefix. The
behavior changed and the tests were not updated. `effigy validate` runs
health + svelte-check + tsc + component tests only, with no task running the
118-file / 720-test unit suite, which is exactly why this drifted unnoticed.

## Evidence

- failing tests `ts/tests/patterns/navigation.test.ts:140`,
  `ts/tests/patterns/navigation-back-info.test.ts`
- behavior source `ts/src/patterns/navigation-back-info.ts:22-26`
- pipeline gap: `effigy.toml` `validate` has no unit `test` task

## Governing References

- [120 Tooling, testing, and contract artifacts](../../contracts/120-tooling-testing-and-contract-artifacts.md)
- [022 Testing posture and shared harnesses](../../contracts/022-testing-posture-and-shared-harnesses.md)

## Planned Changes

- [x] Fix the 4 failing tests to match the intended trimmed-label behavior (or
  restore the prefix if the change was unintended).
- [x] Add a `test = "bun x vitest run"` task to `effigy.toml`.
- [x] Include it in the `validate` sequence so the unit suite gates.

## Consumer Upgrade Impact

Impact class: `none`.

## Validation

- [x] `bun x vitest run` green
- [x] `effigy validate` runs and passes the unit suite

## Stop Conditions

None.

## Completion Notes

Completed 2026-07-17. The trimmed-label behavior is intended; the 4
navigation tests updated to match. `effigy.toml` gains `test = "bun x vitest
run"` and `validate` now runs it before component tests. `bun x vitest run`:
119 files / 735 tests green. `effigy validate` end-to-end green (health,
svelte-check 0 errors, tsc, unit suite, component suite).

## Next Task

Lane B complete. `g08.015` (Lane C) error taxonomy.
