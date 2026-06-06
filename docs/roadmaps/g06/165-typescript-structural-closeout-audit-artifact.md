# g06.165 Artifact - TypeScript Structural Closeout Audit

## Result

The TypeScript structural cleanup lane is closed.

Production source god-file warnings are clear. The only remaining doctor
warning is `scan.god-files`, with 9 warning-only test files documented in
`g06.164`.

## Validation

- `effigy doctor`: passed with warning-only `scan.god-files`
- `effigy qa:docs`: passed
- `effigy qa:northstar`: passed
- `git diff --check`: passed

## Source Closeout

The final TypeScript source cleanup was:

- `g06.160`: split RelationSelector helpers out of the Svelte component
- `g06.161`: split HTTP client support modules
- `g06.162`: split storage support modules
- `g06.163`: split pagination controllers
- `g06.164`: accepted remaining test-file warnings as intentional backlog

The public facades stayed stable. No consumer-app change is expected from these
source splits alone.

## Residual Backlog

Remaining warnings:

- `ts/tests/nightfire/utils.test.ts`
- `ts/tests/client/sveltekit.test.ts`
- `ts/tests/patterns/forms.test.ts`
- `ts/tests/patterns/i18n.test.ts`
- `ts/tests/nightfire/summary-transform.test.ts`
- `ts/tests/server/csp.test.ts`
- `ts/tests/patterns/slugify.test.ts`
- `ts/tests/client/http/auth.test.ts`
- `ts/tests/client/useAuth.test.ts`

These are test navigation costs, not production modularity blockers.

## Next Lane

Run a consumer surface compatibility sweep across the known app family. The goal
is to prove the current Underlay public facades remain usable and to identify
any consumer import drift before more architecture work lands.
