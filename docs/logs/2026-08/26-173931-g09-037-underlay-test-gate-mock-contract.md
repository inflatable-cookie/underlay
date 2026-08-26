# g09.037 Underlay Test Gate And Mock Contract

Date: 2026-08-26
Roadmap: `g09.037`

## Outcome

- removed the Vitest-backed workspace-shape check from `health`
- retained workspace-shape proof in `validate` through `test:unit`
- made `MockHttpClient` extend the exported `HttpClient` contract directly
- preserved all mock runtime behavior and public methods

## Validation

- `effigy --json doctor health` — passed with no routing warning
- `effigy health` — passed without starting Vitest
- `effigy check:types` — passed
- focused `effigy test:unit` — 3 files, 17 tests passed
- `effigy validate` — 126 unit files / 782 tests and 12 component files / 49
  tests passed

Broad `effigy doctor` still reports the existing attention-marker and god-file
scan backlog. Neither finding is caused by this batch.

## Promotion Check

Underlay Reference `main` is clean and exactly aligned with `origin/main`.
`effigy system status --system dev` reports no running task, so a disposable
PostgreSQL target is not positively identified. `g09.038` remains planned.

## Consumer Upgrade Notes

Compatibility is unchanged. Consumers of `createMockHttpClient()` need no
runtime change. The remaining Cattle Grid compatibility-cast cleanup belongs to
`g09.043`.

## Next Task

Establish and positively identify the disposable local PostgreSQL target for
Underlay Reference, then promote `g09.038`.
