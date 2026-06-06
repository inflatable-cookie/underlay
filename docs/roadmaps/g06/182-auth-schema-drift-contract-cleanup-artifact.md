# g06.182 Artifact - Auth Schema Drift Contract Cleanup

Status: complete
Owner: repo maintainers
Completed: 2026-06-06

## Purpose

Close the stale `030` drift note that said the auth database schema reference no
longer matched the live Rust auth types.

## Result

The auth schema reference already matches the live boundary:

- `auth.users` does not store `display_name`
- `User.display_name` remains optional in the shared Rust type
- identity and personalization fields belong in `account.user_profile`
- sessions use `SessionStatus` plus revocation metadata
- refresh rotation state remains first-class in `auth.sessions`

The old drift hook was removed from
[`030-auth-and-session-systems`](../../contracts/030-auth-and-session-systems.md)
and recorded as resolved.

## Consumer Upgrade Impact

Impact class: `none`.

This is documentation and contract-state cleanup only.

## Validation

- `effigy doctor`
- `effigy qa:docs`
- `effigy qa:northstar`

## Next Task

No active roadmap task remains. Open a bounded roadmap card before starting the
next compatibility-retirement, TS boundary, or Rust hardening lane.
