# g06.102 - Auth Password Service Internal Split

## Why

`g06.101` found that `underlay-auth-password/src/service.rs` mixes public
config, repository traits, service construction, password policy checks,
set-password behavior, login/rate-limit/lockout behavior, password changes,
and reset behavior in one security-sensitive production file.

The next split should reduce reasoning load without changing password auth
APIs or semantics.

## Goal

Split the auth password service into focused internal modules while preserving
all public exports and behavior.

## Scope

In scope:

- keep `service.rs` as the small module front door
- move `PasswordConfig` and `CompromisedPasswordStrategy` into a config module
- move `PasswordAuthRepository` and `FailedLoginAttempt` into a repository
  module
- move `PasswordAuthService` construction into a focused core module
- move compromised-password and password-policy helper logic into a focused
  policy module
- move set/change/reset password flows into a focused password operations
  module
- move login, rate-limit, account-status, credential, and lockout flow into a
  focused login module
- preserve existing service tests

Out of scope:

- changing auth password public APIs
- changing password hashing semantics
- changing compromised-password behavior
- changing login, lockout, or reset behavior
- changing consumer apps

## Acceptance Criteria

- `service.rs` becomes a small module front door
- responsibility groups live in focused modules
- crate-root service exports remain stable
- password auth tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal split. If public exports, method names, or password auth
semantics must change, stop and re-enter planning.

## Current State

`g06.102` is ready.

## Next Task

Execute `g06.102`: auth password service internal split.
