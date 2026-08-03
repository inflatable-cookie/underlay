# g09.008 - Config Model Front-Door Guide

Status: complete
Completed: 2026-08-03
Owner: repo maintainers

## Purpose

The config model (effigy dev environment, bundle env schema, config
overlays, canonical helpers, shared dev credentials) is currently recorded
across three logs, sweep 021, and 070 — no single front door. New
consumers and agents should have one guide to follow.

## Evidence

- `docs/logs/2026-08/03-104132-config-convergence.md`
- `docs/sweeps/021-consumer-security-convergence.md` §1
- `docs/architecture/070-consumer-drift-prevention.md` B3
- Audit item 8

## Planned Changes

- [x] New guide `docs/guides/19N-config-model.md` covering: the `effigy`
  identifier and where it comes from (bundle env schema, ancestor
  fallback); `ENVIRONMENT` primary + legacy fallbacks via
  `Environment::resolve`/`resolve_name`; overlay layering
  (`default → <env> → local`) and what belongs in each; CORS via
  `admin_cors_layer`; dev seeds and the shared credential set; how to
  bootstrap a new consumer onto the model.
- [x] Link it from the guide index, 191 admin checklist, and 070.

## Consumer Upgrade Impact

Impact class: `documentation`.

## Validation

- [x] `effigy qa:docs` (front-door link checks)

## Stop Conditions

None expected.

## Completion Notes

Completed 2026-08-03. `docs/guides/192-config-model.md` — the front door for the converged config model (effigy identifier, resolve/resolve_name, overlay layering, canonical CORS, dev seeds/credentials, new-consumer bootstrap). Linked from 191 and 070. `effigy qa:docs` green.

## Next Task

`g09.009` songsprout config seam.
