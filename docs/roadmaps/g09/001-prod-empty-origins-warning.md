# g09.001 - Prod-Empty-Origins Boot Warning

Status: ready
Owner: repo maintainers

## Purpose

`admin_cors_layer` with an empty origin list outside local dev silently
denies all cross-origin access. Fail-closed, but a misconfigured prod
(missing `CORS_ORIGINS`) now boots quietly where the pre-convergence code
panicked. Operators should get a loud signal without a boot failure
(server-to-server-only APIs legitimately have no browser origins).

## Evidence

- `rust/crates/underlay-http/src/cors.rs` (`admin_cors_config`)
- Audit item 1, `docs/logs/2026-08/03-104132-config-convergence.md`
  ("Behavior deltas")

## Planned Changes

- [ ] In `admin_cors_config` (or layer construction), log a `tracing::warn`
  when `environment` is `Prod`/`Staging` and `explicit_origins` is empty:
  "no explicit CORS origins — all cross-origin browser requests will be
  denied; set CORS_ORIGINS if this API serves browsers".
- [ ] Unit test: warning path exercised for Prod/Staging, not for
  `is_local_dev()` environments or non-empty origins.

## Consumer Upgrade Impact

Impact class: `additive`. No consumer change required; all consumers
inherit the warning via the underlay crate.

## Validation

- [ ] `cargo test -p underlay-http`
- [ ] `effigy validate`

## Stop Conditions

None expected.

## Next Task

`g09.002` legacy env-var deprecation signal.
