# g09.002 - Legacy Env-Var Deprecation Signal

Status: ready
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

- [ ] When the resolved value comes from the legacy var, emit a deprecation
  warning naming both vars (stderr/`eprintln!` — `resolve` can run before
  tracing init in some apps).
- [ ] Rate-limit to once per process (static `AtomicBool`) to avoid log
  spam across repeated resolves.
- [ ] Unit tests: legacy path warns once; primary path and unset path do
  not.

## Consumer Upgrade Impact

Impact class: `additive`. Deployments still on legacy vars get a boot-time
warning; no behavior change.

## Validation

- [ ] `cargo test -p underlay-observability`
- [ ] `effigy validate`

## Stop Conditions

None expected.

## Next Task

`g09.003` operator `local.toml` strip note.
