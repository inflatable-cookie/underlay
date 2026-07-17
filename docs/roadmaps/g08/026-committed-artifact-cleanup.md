# g08.026 - Committed-Artifact Cleanup

Status: done
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Remove tracked generated artifacts and dead scripts. `tarpaulin-report.html`
(692 KB) is committed and gitignored simultaneously - it is stale (April 15,
references a deleted `underlay-openapi` crate and an old machine path, reports
54% on a codebase that no longer exists). `ts/coverage/` (9 files incl.
`coverage-final.json`) is also tracked. `scripts/roadmap-015-016-closure.sh` is a
one-off closure script.

## Evidence

- `tarpaulin-report.html` (tracked + gitignored)
- `ts/coverage/` (tracked)
- `scripts/roadmap-015-016-closure.sh` (dead one-off)

## Governing References

- [120 Tooling, testing, and contract artifacts](../../contracts/120-tooling-testing-and-contract-artifacts.md)

## Planned Changes

- [x] `git rm --cached tarpaulin-report.html` (was tracked *and* already in
  `.gitignore` - the tracked copy overrode the ignore, which is why
  `git check-ignore` was silent) and deleted the stale working-tree copy.
- [x] `git rm -r ts/coverage/` (9 tracked files incl. `coverage-final.json`);
  added `ts/coverage/` and a general `coverage/` to `.gitignore` (only
  `tarpaulin-report.html` was covered before).
- [x] Removed `scripts/roadmap-015-016-closure.sh` (a spent 184-line one-off from
  the 2026-02 g01 closure; referenced now only in historical roadmap/log docs).
  Audited the other nine root `scripts/*.sh`: all live (`auth-*` regression/e2e
  tooling, `check-*` guardrails) and retained.
- [x] Coverage now regenerates on demand only and stays untracked (ignored).

## Consumer Upgrade Impact

Impact class: `none`.

## Validation

- [x] Artifacts untracked: `git ls-files` reports nothing for
  `tarpaulin-report.html`, `ts/coverage/`, or the closure script; `ts/coverage/`
  now resolves via `git check-ignore`.
- [x] `effigy validate` clean (svelte-check 0 errors, guardrails, 739 unit + 33
  component). No check referenced the removed script.

## Stop Conditions

None.

## Next Task

`g08.027` contract-sync decision.
