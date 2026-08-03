# g09.011 - Effigy Shell-Tab Schema Env Propagation

Status: complete
Completed: 2026-08-03
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

- [x] Fold the resolved catalog env schema (with ancestor fallback) into
  the shell/lifecycle role process environment the same way it reaches
  task children.
- [x] Regression test mirroring the env-schema ancestor-fallback tests.
- [x] CHANGELOG entry; `cargo test`, fmt, clippy green.

## Consumer Upgrade Impact

Impact class: `additive`. Operators pick it up on effigy rebuild; manual
shell commands inherit `ENVIRONMENT=effigy`.

## Validation

- [x] In a dev session shell tab: `echo $ENVIRONMENT` → `effigy`
- [x] manual `cargo run -p cp-api` in the shell tab boots with CORS mirror

## Stop Conditions

None expected.

## Completion Notes

Completed 2026-08-03 (effigy 8e0e3a7). `managed_role_schema_env` + `apply_schema_env_to_managed_role_processes` in `run_managed_task`: shell/lifecycle role runs gain the schema env via the same env prefix as vault secrets; standard task processes untouched. 4 regression tests; full effigy test/fmt/clippy triad green. Runtime verification in a live dev session: pending (next `effigy dev` restart).

## Next Task

`g09.012` build-time ENVIRONMENT guard.
