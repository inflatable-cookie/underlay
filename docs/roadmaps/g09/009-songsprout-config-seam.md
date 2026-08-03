# g09.009 - Songsprout Config Seam Alignment

Status: ready
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

- [ ] Thread the resolved environment name into both loaders and replace
  the hardcoded `dev.toml` entries with `config/<env>.toml` (same
  `resolve_name` pattern as the other consumers).
- [ ] Move dev-stack constants from `config/default.toml` into committed
  `config/effigy.toml`; reset `default.toml` to safe cross-env defaults.
- [ ] Fix the README's `local.toml` claim (or add the real layer).
- [ ] Verify boot in-stack: `env: Effigy`, overlay loaded.

## Consumer Upgrade Impact

Impact class: `configuration` (file moves; values unchanged). UAT/prod
deployments that relied on dev constants in `default.toml` must be checked
first — confirm nothing deployed reads those values as defaults.

## Validation

- [ ] `cargo check --workspace --all-features --all-targets` (nursery)
- [ ] stack boot + `/v1/health` with env Effigy (spot-check harness)

## Stop Conditions

If a deployed environment is found to depend on the dev constants in
`default.toml`, split the card: loaders first, constants second.

## Next Task

`g09.010` farmyard seed-bundle credentials.
