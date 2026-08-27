# g09.061 - Attention-Marker Policy Normalization

Status: ready - dispatched
Owner: Underlay maintainers
Depends on: `g09.060` (`complete`)

## Purpose

Make Underlay's attention-marker doctor check report actionable deferred work
instead of treating public deprecation metadata and ordinary prose as errors.

## Decision

The operator chose a green-doctor finish line: error checks must clear while
advisory warnings remain visible. Rust `#[deprecated]` and TypeScript
`@deprecated` surfaces are compatibility metadata, not generic deferred-work
markers. Their retirement follows Contract `023`, caller proof, and release
coordination.

Underlay owns this marker policy in `effigy.toml`:

- warning: `TODO`, `REVIEW`, `placeholder`
- high: `FIXME`, `HACK`, `workaround`, `tech debt`
- critical: `BUG:`, `SECURITY:`, `remove before release`

The colon-bearing critical markers preserve explicit action labels without
matching ordinary words such as “security” or “bug” in explanatory comments.

## Scope

- add `[scan.attention_markers]` to `effigy.toml` with the settled marker lists
- retain doctor integration, Git-ignore handling, and normal scan traversal
- verify the effective config and JSON scan envelope report the committed lists
- prove the current consumer-conformance prose, Rust deprecations, and test note
  are no longer findings
- update this roadmap and one lane execution log

## Out Of Scope

- deleting or changing any deprecated public API
- editing consumer repositories or release versions
- changing Effigy source, its CLI parser, or `.github/workflows/`
- disabling attention-marker scanning or doctor integration
- suppressing the god-file or comment-ratio inventories
- editing shared roadmap front doors; orchestrator closeout owns them after both
  doctor lanes merge

## Acceptance

- `effigy config --inspect --path scan.attention_markers` shows the settled
  marker lists and `doctor = true`
- `effigy scan attention-markers --json` reports those lists and no current
  finding
- real `TODO`, `FIXME`, `HACK`, `BUG:`, and `SECURITY:` labels remain detectable
- public deprecation metadata remains unchanged
- this lane removes the attention-marker doctor error without hiding other
  doctor findings

## Validation

- `effigy config --inspect --path scan.attention_markers`
- `effigy scan attention-markers --json`
- `effigy doctor --verbose` — expected to retain the `workspace-shape.ts`
  god-file error until `g09.062` merges
- `effigy health`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Evidence

- exact `main` doctor report at discovery commit `60ff292b`
- `docs/triage/20260827-223450-underlay-doctor-scan-backlog.md`
- current Effigy manifest schema and scanner matching behavior
- current six-consumer caller scan showing Compli Me still uses both deprecated
  pagination aliases

## Stop Conditions

Stop if the policy requires disabling the scanner, excluding broad source
trees, changing an Effigy binary, deleting a deprecated API, or deciding a
consumer compatibility window. Do not reword useful code comments merely to
evade substring matching.

## Consumer Upgrade Impact

- Impact class: internal tooling configuration
- Affected consumers: none
- Required action: none
- Compatibility window: none; public deprecations remain intact

## Dispatch Evidence

- planning base: `049fae4dd5f326bfbb08bc97b5e6ef7bfcd6c8b5`
- handoff:
  `docs/handoffs/20260827-224034-g09-061-attention-marker-policy-normalization.md`
- topology: parallel with `g09.062`; no shared implementation or lane-evidence
  files
- shared front doors remain orchestrator-owned after both reviewed merges

## Next Task

Launch the published handoff. Open one Underlay PR and stop for orchestrator
review; do not merge or update shared front doors.
