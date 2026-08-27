# 2026-08-27 22:49:08 - g09.061 Attention-Marker Policy Normalization

## Outcome

Committed Underlay's action-bearing attention-marker policy. Doctor no longer
treats ordinary security prose, public Rust deprecations, or an explanatory
test note as errors.

## Worktree

- root: `/Users/tom/.t3/worktrees/underlay/t3code-ddc27e76`
- branch: `t3code/normalize-attention-marker-policy`
- base: `origin/main` at `493aa4cbd8bb0981d56ba2d575eae5bf22205dbf`

## What Changed

Added `[scan.attention_markers]` to `effigy.toml`:

- warning: `TODO`, `REVIEW`, `placeholder`
- high: `FIXME`, `HACK`, `workaround`, `tech debt`
- critical: `BUG:`, `SECURITY:`, `remove before release`
- `doctor = true`
- `fail_on_findings = false`
- `respect_gitignore = true`

No include/exclude overrides. Public `#[deprecated]` and `@deprecated`
surfaces were not edited. Consumer repos were not edited.

## Proof

`effigy config --inspect --path scan.attention_markers` reports the committed
lists and doctor integration from `effigy.toml`.

Pre-change `effigy scan attention-markers --json` on this HEAD matched the
discovery shape: five errors and one warning.

- critical: `scripts/check-consumer-conformance.sh:6` “security shapes”
- high: `#[deprecated]` in `underlay-config`, `underlay-db`, `underlay-http`,
  and `underlay-query`
- warning: `Note:` in `underlay-ratelimit` Postgres tests

Post-change JSON scan: `finding_count` 0, 1017 files scanned, patterns equal
the committed lists.

An uncommitted probe in `scripts/` then proved `TODO`, `FIXME`, `HACK`,
`BUG:`, and `SECURITY:` still match. The probe was deleted before commit.

## Partial Doctor State

`effigy doctor --verbose` after the policy:

- attention-marker error gone
- remaining error: `scan.god-files` / `ts/src/tools/workspace-shape.ts`
  (559 code lines)
- remaining warning: comment-ratio, unchanged
- summary: `ok:18  warn:1  err:1`

The remaining error belongs to `g09.062`. This lane did not hide it.

## Validation

- `effigy config --inspect --path scan.attention_markers`
- `effigy scan attention-markers --json`
- `effigy doctor --verbose`
- `effigy health`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

No planning issue appeared. The CLI marker-override papercut remains upstream
and unfixed; validation used the committed manifest.

## Consumer Upgrade Notes

- Impact class: internal tooling configuration
- Affected consumers: none
- Required action: none
- Compatibility window: none; public deprecations remain intact

## Next Task

Open one Underlay PR and stop for orchestrator review. Do not merge or update
shared front doors.
