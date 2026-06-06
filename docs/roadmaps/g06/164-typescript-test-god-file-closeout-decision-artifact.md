# g06.164 Artifact - TypeScript Test God-File Closeout Decision

## Result

The remaining TypeScript god-file warnings are test-only and accepted as
warning backlog.

## Evidence

`effigy doctor` reports 9 `scan.god-files` warnings:

- `393 code lines (469 total)`: `ts/tests/nightfire/utils.test.ts`
- `371 code lines (428 total)`: `ts/tests/client/sveltekit.test.ts`
- `370 code lines (441 total)`: `ts/tests/patterns/forms.test.ts`
- `339 code lines (405 total)`: `ts/tests/patterns/i18n.test.ts`
- `299 code lines (328 total)`: `ts/tests/nightfire/summary-transform.test.ts`
- `292 code lines (381 total)`: `ts/tests/server/csp.test.ts`
- `280 code lines (361 total)`: `ts/tests/patterns/slugify.test.ts`
- `277 code lines (339 total)`: `ts/tests/client/http/auth.test.ts`
- `253 code lines (294 total)`: `ts/tests/client/useAuth.test.ts`

All production TypeScript source god-file warnings were cleared by `g06.160`
through `g06.163`.

## Classification

- Nightfire utility and summary-transform tests are behavior matrices. Split
  only when Nightfire source work changes them.
- SvelteKit client tests are integration-style adapter tests. Split only with
  adapter source work.
- Forms and i18n tests are broad data-driven behavior suites. Split only if
  future source structure gives them a clearer boundary.
- CSP tests stay grouped because security-header behavior is easier to audit as
  one matrix.
- Slug tests stay grouped because generation, validation, reserved-word checks,
  and integration cases form one behavior contract.
- HTTP auth and `useAuth` tests stay grouped because they cover auth refresh,
  concurrency, and state-machine behavior.

## Decision

No test split is needed now. The reference-grade blocker was production source
structure, and that is clear.

Future splits should be coupled to source work or a stronger doctor policy, not
done as standalone churn.
