# g07.008 Artifact - TS Testing And Guardrail Support Gap Inventory

Status: complete
Owner: repo maintainers
Updated: 2026-06-06

## Scope

This artifact records the test and guardrail support posture for the retained TS
runtime, client, pattern, template, testing, and tools surfaces after `g07.008`.

## Effigy Support

Retained TS validation tasks:

- `effigy check:exports` validates every `package.json` export target exists
- `effigy check:types` runs `tsc -p ./ts/tsconfig.json`
- `effigy check` runs `svelte-check --tsconfig ./ts/tsconfig.json`
- `effigy check:guardrails` runs the shared guardrail scanner over `ts/src`
- `effigy check:component-test-hygiene` enforces shared component-test cleanup
  wiring and fake-timer hygiene
- `effigy test:components` runs component tests through
  `vitest.component.config.ts`
- `effigy validate` runs health, Svelte check, type check, and component tests

`effigy test --plan` currently detects two suites for the repo:

- `bun x vitest run`
- `cargo nextest run`

## Existing TS Test Coverage

Current TS test count: 99 `*.test.ts` files.

Breakdown by top-level test area:

| Area | Count | Posture |
| --- | ---: | --- |
| `patterns` | 55 | strong direct coverage for retained workflow helpers |
| `nightfire` | 16 | strong direct coverage for retained Nightfire surfaces |
| `client` | 14 | solid direct coverage for lower client helpers |
| `utils` | 5 | focused direct coverage |
| `components` | 4 | component-level coverage for retained visible exceptions |
| `templates` | 2 | focused but thin coverage for retained template shells |
| `testing` | 1 | direct coverage for `createMockHttpClient()` |
| `server` | 1 | focused direct coverage |
| package compatibility | 1 | thin smoke coverage for selected public barrels |

Component support:

- 6 component tests
- 23 Svelte harness fixtures
- shared component setup file enforced by `check:component-test-hygiene`

## Surface Posture

| Surface | Support Posture | Gap |
| --- | --- | --- |
| `client/*` | direct unit tests plus type/export checks | `client/suggestions` behavior is tested through `patterns/selection-history`, not through its preferred public path |
| `runtime/*` | implementation tests through `patterns/*` and indirect root runtime smoke | focused runtime subpath package-compatibility tests are thin |
| `patterns/*` | strong direct tests | compatibility-only re-exports need clearer test/import posture before retirement |
| `templates/*` | focused tests and component harnesses | broader template package-compatibility smoke would help |
| `testing/*` | direct `createMockHttpClient()` tests | adequate for current narrow surface |
| `tools/*` | live scanner task plus export checks | no unit tests for scanner, suppressions, config loading, or template import behavior |

## Compatibility-Only Paths

Current compatibility-only concern:

- suggestion-param helpers re-exported from `patterns/selection-history.ts`

Preferred public paths:

- `@decodelabs/underlay/client/suggestions` for suggestion request helpers
- `@decodelabs/underlay/runtime/data` for `createSelectionHistory()`

Decision:

- do not remove compatibility exports in this inventory card
- add direct preferred-path tests before any retirement sweep
- use the later consumer import compatibility sweep to decide whether removal
  is safe

## Guardrail Gaps

The guardrail scanner is executable and part of `health`, but it lacks direct
self-tests for:

- module-scope browser API detection
- suppression parsing
- Svelte script-block scanning
- template loading through package-style template refs
- banned API template behavior

These are support-layer tests, not new policy.

## Follow-On Work

`g07.009` should add bounded reinforcement:

- direct `client/suggestions` tests through the preferred source/public path
- package-compatibility smoke tests for retained `runtime/*`, `client/*`,
  `templates`, `testing`, and `tools` public paths
- guardrail self-tests for scanner/config/template behavior

Keep the work additive. Do not change public exports or consumer imports in
`g07.009`.
