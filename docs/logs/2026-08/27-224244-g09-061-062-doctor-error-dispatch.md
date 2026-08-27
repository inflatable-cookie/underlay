# 2026-08-27 22:42:44 - g09.061-062 Doctor Error Dispatch

## Outcome

Published one worker handoff for each independent doctor-error roadmap. Both
lanes are ready to launch in parallel.

## Planning Base

- commit: `049fae4dd5f326bfbb08bc97b5e6ef7bfcd6c8b5`
- local `HEAD` matched `origin/main` before handoff creation
- `effigy health`, `effigy qa:docs`, `effigy qa:northstar`, and
  `git diff --check` passed on the promoted runway

## Handoffs

- `g09.061`:
  `docs/handoffs/20260827-224034-g09-061-attention-marker-policy-normalization.md`
- `g09.062`:
  `docs/handoffs/20260827-224035-g09-062-workspace-shape-internal-modularization.md`

Each handoff declares `worker-pr-loop`, implementation worker mode, and
orchestrator dispatch authority. The planning base intentionally predates the
handoff commit.

## Parallel Safety

- `g09.061` owns `effigy.toml`, its roadmap, and its execution log
- `g09.062` owns workspace-shape source/tests, its roadmap, and its execution
  log
- workers do not edit shared front doors or `docs/logs/README.md`
- full doctor closeout waits for both independently reviewed merges

## Boundaries

- no deprecated API deletion, consumer migration, or release work
- no zero-warning sweep, threshold retune, or broad scan suppression
- no package export, stable diagnostic, or CLI behavior change
- no worker merges

## Next Task

Launch both handoffs in parallel. Review each PR independently and merge only
with explicit operator authorisation.
