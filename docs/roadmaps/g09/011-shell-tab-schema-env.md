# g09.011 - Effigy Shell-Tab Schema Env Propagation

Status: ready
Owner: repo maintainers

## Purpose

The bundle env schema reaches task-rendered processes but not the managed
dev session's `shell`/`lifecycle` role processes (they are not task
references). Operators running `cargo run -p cp-api` manually in the shell
tab get no `ENVIRONMENT` and hit the fail-closed prod posture (no CORS
mirror) — the same break the convergence fixed for task runs.

## Evidence

- effigy `crates/effigy-managed/src/plan/entries.rs`
  (`resolve_shell_process_run`) and spawn env path
- Audit item 11; `docs/logs/2026-08/03-104132-config-convergence.md`
  ("Known boundary")

## Planned Changes (effigy repo)

- [ ] Fold the resolved catalog env schema (with ancestor fallback) into
  the shell/lifecycle role process environment the same way it reaches
  task children.
- [ ] Regression test mirroring the env-schema ancestor-fallback tests.
- [ ] CHANGELOG entry; `cargo test`, fmt, clippy green.

## Consumer Upgrade Impact

Impact class: `additive`. Operators pick it up on effigy rebuild; manual
shell commands inherit `ENVIRONMENT=effigy`.

## Validation

- [ ] In a dev session shell tab: `echo $ENVIRONMENT` → `effigy`
- [ ] manual `cargo run -p cp-api` in the shell tab boots with CORS mirror

## Stop Conditions

None expected.

## Next Task

`g09.012` build-time ENVIRONMENT guard.
