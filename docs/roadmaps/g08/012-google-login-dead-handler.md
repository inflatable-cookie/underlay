# g08.012 - Google Login Dead Handler

Status: complete
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

"Continue with Google" fires nothing. `LoginGoogleTab` passes `onclick=` to the
Poodle `Button`, which expects `onClick` and overrides the forwarded native
`onclick` with its own handler. All sibling call sites use `onClick`. The
`check:poodle-prop-names` guardrail exists but did not catch this instance.

## Evidence

- `ts/src/patterns/auth-workflows/LoginGoogleTab.svelte:33`

## Governing References

- [100 Shared patterns and workflow shells](../../contracts/100-shared-patterns-and-workflow-shells.md)

## Planned Changes

- [x] Rename the prop to `onClick`.
- [x] Audit auth-workflow tabs for the same `onclick`/`onClick` mismatch.
- [x] Confirm `check:poodle-prop-names` covers event-handler props; extend it if
  the miss was a coverage gap.

## Consumer Upgrade Impact

Impact class: `none`. Bug fix.

## Validation

- [x] component test: Google tab click invokes the handler
- [x] `effigy check:poodle-prop-names`
- [x] `effigy validate`

## Stop Conditions

None.

## Completion Notes

Completed 2026-07-17. `LoginGoogleTab` renamed to `onClick`. Guardrail gap
was real and double: `check:poodle-prop-names` matched only
`@poodle/svelte-primitives|composites` imports (missing plain
`@poodle/svelte`) and had no event-handler casing rule. Both fixed (plus an
overreaching import regex that spanned statements). The extended check found
and fixed three more dead handlers: `EntityDetailPage` Retry button, guide
`097` Button examples, guide `186` EditableList `onsubmit`/`oncancel`.
Component test: Google tab click invokes handler. Guardrail green.

## Next Task

`g08.013` media validation bypass and upload cancellation.
