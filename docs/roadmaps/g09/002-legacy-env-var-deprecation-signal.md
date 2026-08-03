# g09.002 - Legacy Env-Var Deprecation Signal

Status: complete
Completed: 2026-08-03
Owner: repo maintainers

## Purpose

`Environment::resolve(primary, legacy)` silently honors the legacy var
(`ACME_ENV`, `CP_ENV`, `COMPLI_ENV`, `COMPOSER_ENV`, `ENVIRONMENT_NAME`)
when primary is unset. That was the migration escape hatch, but silent
means invisible: a deployment that only sets the legacy var will never be
noticed and never cleaned up. The convergence plan called for a deprecation
signal; it shipped without one.

## Evidence

- `rust/crates/underlay-observability/src/tracing_init.rs` (`resolve`)
- Audit item 2

## Planned Changes

- [x] When the resolved value comes from the legacy var, emit a deprecation
  warning naming both vars (stderr/`eprintln!` — `resolve` can run before
  tracing init in some apps).
- [x] Rate-limit to once per process (static `AtomicBool`) to avoid log
  spam across repeated resolves.
- [x] Unit tests: legacy path warns once; primary path and unset path do
  not.

## Consumer Upgrade Impact

Impact class: `additive`. Deployments still on legacy vars get a boot-time
warning; no behavior change.

## Validation

- [x] `cargo test -p underlay-observability`
- [x] `effigy validate`

## Stop Conditions

None expected.

## Completion Notes

Completed 2026-08-03. `Environment::resolve` now prints a one-per-process stderr deprecation warning when the value comes from the legacy var (`legacy_env_var_warning` + AtomicBool guard). Tests: warning names both vars; existing resolve/resolve_name tests green.

## Next Task

`g09.003` operator `local.toml` strip note.
