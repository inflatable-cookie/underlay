# g07.004 Artifact - Pattern Helper Ownership Diet

## Result

The root `@inflatable-cookie/underlay/patterns` barrel is smaller and now matches the
retained workflow-shell posture more closely.

Removed from the pattern root:

- `selection-mode-controller.svelte`
- `selection-transform-state`
- `reorder-session.svelte`

These helpers remain public through `@inflatable-cookie/underlay/runtime/data`.

## Retained Pattern Root

| Export family | Classification | Disposition | Evidence |
| --- | --- | --- | --- |
| `LoginPage` | retained workflow shell | keep on `patterns` | Active auth callers in `underlay-reference`, `contact-patch`, `compli-me`, and `acowtancy`. |
| `ForgotPasswordFlow` | retained workflow shell | keep on `patterns` | Active auth callers in the same consumer family. |
| `PasswordRequirements` | retained auth policy adapter | keep on `patterns` | Active account/register callers in `underlay-reference`, `contact-patch`, `compli-me`, and `acowtancy`. |
| `SpaFormShell` | retained SPA form shell | keep on `patterns` | Active direct shell callers in `acowtancy` and `compli-me`; templates also consume the result contract. |
| `SpaFormResult`, `SpaSubmitHandler`, `SpaNavigateFn` | retained SPA form contract types | keep on `patterns` | Heavy active `SpaFormResult` use across `underlay-reference`, `contact-patch`, `compli-me`, `acowtancy`, and `loophole/composer`. |
| `createContextActionController` and context action types | retained workflow helper | keep on `patterns` | Active `acowtancy/dairy` caller and active template usage. |

## Removed Root Exports

| Export family | Classification | New public path | Consumer evidence | Disposition |
| --- | --- | --- | --- | --- |
| `createSelectionModeController` | runtime-owned lower data helper | `runtime/data` | No known active consumer imports from `patterns`; `acowtancy/dairy` already imports related transform helper from `runtime/data`. | Remove from root. |
| `buildSelectionTransformState` | runtime-owned lower data helper | `runtime/data` | No known active consumer imports from `patterns`; active caller uses `runtime/data`. | Remove from root. |
| `createLocalReorderSession`, `createLoadedReorderSession` | runtime-owned lower data helpers | `runtime/data` | No known active consumer imports from `patterns`. | Remove from root. |

## Consumer Scan

Roots scanned:

- `underlay-reference`
- `contact-patch`
- `compli-me`
- `acowtancy`
- `songsprout`
- `loophole/composer`

Findings:

- Known live pattern-root imports are auth workflows, `PasswordRequirements`,
  `SpaFormShell`, `SpaFormResult`, and `createContextActionController`.
- No active code imports the removed selection/reorder helper families from the
  pattern root.
- Some historical docs, guardrail messages, and Vite optimize-deps entries still
  mention `@inflatable-cookie/underlay/patterns`; these are not source imports of the
  removed helpers.

## Consumer Upgrade Impact

Breaking for unknown callers that imported the removed helpers from
`@inflatable-cookie/underlay/patterns`.

No known consumer app update is required. The retained public path for those
helpers is `@inflatable-cookie/underlay/runtime/data`.

## Follow-on

`g07.005` should decide whether duplicated auth-aware fetch orchestration in
runtime/list/data helpers needs consolidation or clearer split guidance.

## Validation Inputs

- inspected `ts/src/patterns/index.ts`
- compared with contracts `090`, `100`, and `117`
- scanned active Underlay docs and source examples
- scanned the six-consumer family for exact pattern-root imports
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy check:exports`

## Next Task

Move to `g07.005`: duplicated auth-aware fetch orchestration decision.
