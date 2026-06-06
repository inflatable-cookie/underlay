# g06.184 Artifact - Auth Runtime Pattern Boundary Assessment

Status: complete
Owner: repo maintainers
Completed: 2026-06-06

## Purpose

Close the stale `030` drift hook around `runtime/auth` re-exporting
pattern-layer auth helpers and the broader internal auth workflow folder.

## Result

Keep the current split.

`runtime/auth` is the preferred browser auth runtime front door for:

- auth configuration
- auth-state helpers
- passkey helpers
- profile helpers
- protected data loading

The retained auth workflow UI surface remains pattern-owned:

- `LoginPage`
- `ForgotPasswordFlow`
- `PasswordRequirements`
- supporting `auth-workflows/` components and types

This matches `090`, `100`, and the `g07.002` runtime subpath audit. No public
export movement is needed.

## Consumer Upgrade Impact

Impact class: `none`.

This is documentation and contract-state cleanup only.

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy check:exports`

## Next Task

No active roadmap task remains. Open a bounded roadmap card before starting the
next compatibility-retirement, TS boundary, or Rust hardening lane.
