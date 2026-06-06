# g07.003 - Runtime Import Guidance Cleanup

Status: ready
Owner: repo maintainers
Updated: 2026-06-06

## Context

`g07.002` kept the current `runtime/*` subpaths and found no source API drift.
It did find active docs and source JSDoc examples that still teach
`@decodelabs/underlay/patterns` for helpers whose preferred public path is now
`runtime/*` or `utils/*`.

This card cleans teaching-surface drift only.

## Goals

- [ ] update active usage/guides docs to teach the preferred runtime, client,
  patterns, templates, and utils import paths
- [ ] update source JSDoc examples that teach stale `patterns` imports
- [ ] keep true retained pattern-root examples intact
- [ ] avoid changing package exports or implementation behavior

## Non-Goals

- changing public exports
- editing consumer apps
- retiring the root `runtime` barrel
- changing historical roadmap/log evidence
- refactoring runtime or pattern implementations

## Execution Plan

- [ ] update stale `patterns` imports for form helpers, storage/browser helpers,
  formatting/slug helpers, media helpers, list/data helpers, and keyboard
  helpers in active docs
- [ ] update stale source JSDoc examples in pattern implementation files
- [ ] preserve `patterns` imports for true retained workflow shells such as
  `LoginPage`, `ForgotPasswordFlow`, `PasswordRequirements`, `SpaFormShell`,
  and explicitly retained pattern-root helpers
- [ ] rerun docs, Northstar, export, and targeted import scans

## Acceptance Criteria

- [ ] active guide examples prefer `runtime/*`, `client/*`, `templates`, or
  `utils/*` where those are the retained paths
- [ ] `patterns` examples remain only where the pattern root is the retained
  public surface
- [ ] source JSDoc no longer contradicts current package guidance for runtime
  helpers
- [ ] no public export or implementation behavior changes

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy check:exports`
- targeted `rg` scans for stale `@decodelabs/underlay/patterns` guidance

## Consumer Upgrade Impact

None.

This is docs/JSDoc guidance cleanup only.

## Next Task

Execute this runtime import guidance cleanup.
