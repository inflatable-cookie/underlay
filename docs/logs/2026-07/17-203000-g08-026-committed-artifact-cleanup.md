# 2026-07-17 - g08.026 committed-artifact cleanup

## Context

The repo tracked generated coverage artifacts and a spent one-off script:
`tarpaulin-report.html` (stale April-15 report referencing a deleted crate and an
old machine path), the `ts/coverage/` tree (9 files), and
`scripts/roadmap-015-016-closure.sh`.

## Changes

- **`tarpaulin-report.html`** - was tracked *and* already listed in `.gitignore`;
  the tracked copy silently overrode the ignore (why `git check-ignore` reported
  nothing). `git rm --cached` + deleted the stale working-tree file.
- **`ts/coverage/`** - `git rm -r` the 9 tracked files. `.gitignore` only covered
  `tarpaulin-report.html`, so added `ts/coverage/` and a general `coverage/`.
  Coverage now regenerates on demand and stays untracked.
- **`scripts/roadmap-015-016-closure.sh`** - removed. It was a 184-line one-off
  from the 2026-02 g01 roadmap-015/016 closure (hardcoded four consumer paths,
  pre-dating the current six-app family); referenced now only in historical
  roadmap/log docs, which remain as archival evidence. Audited the other nine
  root `scripts/*.sh` - all live (`auth-live-e2e-readiness`,
  `auth-webauthn-regression`/`-manual-dossier`, and the `check-*` guardrails) and
  retained.

## Validation

- `git ls-files` reports nothing for the three removed paths; `ts/coverage/`
  resolves via `git check-ignore`.
- `effigy validate`: clean - svelte-check 0 errors (2472 files), guardrails,
  component hygiene, 739 unit + 33 component tests. No check referenced the
  removed script.

## Consumer Upgrade Notes

Impact class **none**. Repo-hygiene only; no code, contract, or exported surface
changed.

## Next

`g08.027` contract-sync decision.
