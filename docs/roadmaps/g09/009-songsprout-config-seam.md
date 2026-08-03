# g09.009 - Songsprout Config Seam Alignment

Status: complete
Completed: 2026-08-03
Owner: repo maintainers

## Purpose

songsprout predates the overlay model: dev-stack constants live in
`config/default.toml` (which should be safe defaults), its two hand-rolled
loaders (`config/mod.rs`, `config/behavior.rs`) hardcode a vestigial
`dev.toml` layer nothing provides, and its README documents a
`config/local.toml` layer no loader reads. Align it with the fleet shape.

## Evidence

- `songsprout/nursery/crates/infra/src/config/mod.rs:37-48`,
  `config/behavior.rs:61-68`
- `songsprout/config/default.toml`
- Overlay assessment report (2026-08-03, overlay agent)
- Audit item 9

## Planned Changes

- [x] Thread the resolved environment name into both loaders and replace
  the hardcoded `dev.toml` entries with `config/<env>.toml` (same
  `resolve_name` pattern as the other consumers).
- [x] Move dev-stack constants from `config/default.toml` into committed
  `config/effigy.toml`; reset `default.toml` to safe cross-env defaults.
- [x] Fix the README's `local.toml` claim (or add the real layer).
- [x] Verify boot in-stack: `env: Effigy`, overlay loaded.

## Consumer Upgrade Impact

Impact class: `configuration` (file moves; values unchanged). UAT/prod
deployments that relied on dev constants in `default.toml` must be checked
first — confirm nothing deployed reads those values as defaults.

## Validation

- [x] `cargo check --workspace --all-features --all-targets` (nursery)
- [x] stack boot + `/v1/health` with env Effigy (spot-check harness)

## Stop Conditions

If a deployed environment is found to depend on the dev constants in
`default.toml`, split the card: loaders first, constants second.

## Completion Notes

Completed 2026-08-03. Loaders layer default -> config/<env> (Environment::resolve_name, ENVIRONMENT primary) -> config/local.toml; dev constants moved from default.toml into committed config/effigy.toml; default.toml is cross-env safe (no [server] environment — bare runs fail closed). No deployment configs exist, so the stop condition did not trigger. Runtime verification (stack, ENVIRONMENT=effigy): boot log shows env Effigy, db configured, cors_origins=2 from the overlay; seeds upsert clean; login with shared fleet creds + corrected TOTP secret works end-to-end; /health and /v1/admin/billing/subscriptions 200. One finding fixed in flight: overlay's email adapter is now noop — smtp is not wired in the current build (pre-existing: the old default.toml was equally unbootable without EMAIL_ADAPTER=noop).

## Next Task

`g09.010` farmyard seed-bundle credentials.
