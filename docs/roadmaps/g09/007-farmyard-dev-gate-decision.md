# g09.007 - Farmyard Dev Gate Decision

Status: complete
Completed: 2026-08-03
Owner: repo maintainers

## Purpose

farmyard's dev gates are `Local | Effigy | Dev | Test`. Now that its local
stack environment is `effigy` (overlay renamed), `Dev` in those gates is
either (a) needed for a deployed environment literally named `dev`, or
(b) vestigial. Pick one deliberately rather than leaving it ambiguous.

## Evidence

- `farmyard/crates/infra/src/config.rs` (12 gate sites)
- `farmyard/crates/api/src/main.rs`, `crates/jobs/src/main.rs`
- Audit item 7

## Planned Changes

- [x] Determine whether any deployed farmyard environment is named `dev`
  (check deploy configs, ledger docs, uat/staging inventories).
- [x] If none: trim gates to `is_local_dev()` (or `is_local_dev() || Dev`
  only where a deployed dev exists), with a comment recording the decision.
- [x] If a deployed dev exists: document it at the gate sites and close the
  card as accepted.

## Consumer Upgrade Impact

Impact class: `additive` or none (behavior change only if (b) and a
deployed dev env is later discovered — the investigation prevents that).

## Validation

- [x] `cargo check --workspace --all-features --all-targets` (farmyard)
- [x] `cargo test -p farmyard-infra`
- [x] acowtancy parent bump committed

## Stop Conditions

None expected.

## Completion Notes

Completed 2026-08-03. Investigation: no deployed farmyard environment is named `dev` (infra/render has uat only; production overlay separate; `dev` was only ever the local stack, now `effigy`). All 12 gate sites (config.rs ×7, api/main.rs ×4, jobs/main.rs ×1) trimmed from `Local|Effigy|Dev|Test` to `is_local_dev()`; now-unused Environment import dropped. cargo check 0 warnings, farmyard-infra 11/11.

## Next Task

`g09.008` config model guide.
