# g06.153 Artifact - Optimistic Tests Internal Split

## Summary

`ts/tests/patterns/optimistic.test.ts` was split into focused test files by
optimistic primitive.

Final test shape:

- `ts/tests/patterns/optimistic/list.test.ts`
- `ts/tests/patterns/optimistic/toggle.test.ts`
- `ts/tests/patterns/optimistic/value.test.ts`
- `ts/tests/patterns/optimistic/counter.test.ts`

Retained related tests:

- `ts/tests/patterns/optimistic-barrel.test.ts`
- `ts/tests/patterns/optimistic-helpers.test.ts`

## Coverage Impact

No coverage was intentionally removed.

The focused suite still covers:

- list initialization, add, remove, update, set, and pending IDs
- toggle initialization, toggle, set, confirm, and rollback
- value initialization, set, confirm, rollback, no-op, and custom equality
- counter initialization, increment, decrement, set, confirm, and rollback
- optimistic public barrel behavior
- optimistic helper functions

## Validation

Passed:

- `bun x vitest run ts/tests/patterns/optimistic/*.test.ts ts/tests/patterns/optimistic-barrel.test.ts ts/tests/patterns/optimistic-helpers.test.ts`
  - 63 tests passed

Doctor:

- `effigy doctor` still fails on standing attention-marker and comment-ratio
  scans
- god-file findings are now `14` total, `0` high
- high-severity god-file backlog is cleared

Remaining god-file findings are warning-only and led by:

- `ts/tests/nightfire/utils.test.ts`
- `ts/tests/client/sveltekit.test.ts`
- `ts/tests/patterns/forms.test.ts`
- `ts/src/patterns/RelationSelector/context.svelte.ts`
- `ts/tests/patterns/i18n.test.ts`

## Public API Impact

None.

This was test-only structural work.

## Decision

Queue `g06.154` as a TypeScript structural closeout audit to decide the next
doctor cleanup lane now that high-severity god-files are cleared.
