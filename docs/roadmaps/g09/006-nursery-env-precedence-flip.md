# g09.006 - Nursery ENVIRONMENT_NAME Precedence Flip

Status: complete
Completed: 2026-08-03
Owner: repo maintainers

## Purpose

nursery resolves `Environment::resolve("ENVIRONMENT_NAME",
Some("ENVIRONMENT"))` — legacy-var-primary, inverted vs the fleet
convention (`ENVIRONMENT` primary). Harmless today (`ENVIRONMENT_NAME` is
never set in-stack) but a trap the first time someone sets both.

## Evidence

- `songsprout/nursery/crates/infra/src/config/mod.rs:125`,
  `config/transport.rs:62`
- Audit item 6

## Planned Changes

- [x] Flip both call sites to `resolve("ENVIRONMENT",
  Some("ENVIRONMENT_NAME"))`.
- [x] Verify no in-repo script, doc, or deploy config sets
  `ENVIRONMENT_NAME` for nursery (if one does, update it to `ENVIRONMENT`).

## Consumer Upgrade Impact

Impact class: `additive` in practice (precedence only changes when both
vars are set, which nothing does today).

## Validation

- [x] `cargo check --workspace --all-features --all-targets` (nursery)
- [x] grep: no remaining `ENVIRONMENT_NAME`-primary resolution

## Stop Conditions

None expected.

## Completion Notes

Completed 2026-08-03. Both nursery call sites now resolve `ENVIRONMENT` primary with `ENVIRONMENT_NAME` as the deprecated fallback (config/mod.rs, config/transport.rs). grep confirms no script/doc/deploy config sets ENVIRONMENT_NAME for nursery. cargo check green (1 pre-existing unused-import warning).

## Next Task

`g09.007` farmyard `Dev` gate decision.
