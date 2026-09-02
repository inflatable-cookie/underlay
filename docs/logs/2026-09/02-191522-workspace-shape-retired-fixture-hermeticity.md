# 2026-09-02 - Workspace-Shape Retired Fixture Hermeticity

Date: 2026-09-02
Roadmap: `g11.001`, Card 002 blocked at validate; this is not a release mutation
Branch: `worker/restore-workspace-shape-fixture`

## Outcome

The release `validate` gate failed at
`ts/tests/tools/workspace-shape.test.ts:183` because the
`retired-top-level-package` fixture had no top-level `app/` tree after
checkout. Git cannot store the disposable leftover the current test needs.

`loadFixture("retired-top-level-package")` now creates `app/node_modules/`
after copy, matching the existing `nested-git` synthesis. Production checker
behavior is unchanged.

## Why not restore history

Commit `0baceed7` tracked `app/package.json` as `{"name":"@fixture/app-leftover"}`.
That file was dropped from squash `4f4beda7` and is not gitignored. Restoring
it flags inspect/relocate, not disposable leftover, so the current assertions
still fail. Classification was not broadened.

## Changes

- `ts/tests/tools/workspace-shape.test.ts` — synthesize disposable leftover
  for that one fixture name
- `PAPERCUTS.md` — close the 2026-09-02 fixture gap
- `docs/logs/README.md` — point the current evidence window at this log

## Validation

- focused test failed before (`toHaveLength(1)` got `0`) and passed after
- `effigy test:unit ts/tests/tools/workspace-shape.test.ts` — 19 passed
- `effigy validate` — 813 unit + 49 component tests passed
- `effigy qa:docs`, `effigy qa:northstar` — passed
- `git diff --check` — clean
- `effigy --json release status --check-gates` — `ready: true`, `blockers: []`,
  `next_version: 0.9.6`; gates `version-sync`, `validate`, `clippy`, `rust`
  all passed. No prepare/execute.

## Boundaries

- no `app/package.json` restore
- no production checker or classification edit
- no workflow/CI expansion
- no Card 002 authorization change
- no release prepare/execute, tag, consumer edit, or merge

## Next Task

Orchestrator review of the PR. Card 002 still needs explicit operator
authorization after merge; this lane does not cut `v0.9.6`.
