# g06.062 - HTTP Cookies Internal Split

## Why

`g06.061` found that `underlay-http/src/cookies.rs` is stable app-facing auth
and CSRF cookie surface with a safe internal split shape.

Consumers use both `underlay_http::cookies::*` and crate-root exports, so the
split must preserve both front doors.

## Goal

Split `underlay-http/src/cookies.rs` into focused private modules while
preserving cookie config, validation, header string behavior, extractors, and
public exports.

## Scope

In scope:

- split `SameSite` policy helpers
- split typed cookie value wrappers
- split `AuthCookieConfig`
- split individual cookie header builders
- split cookie extractors
- split response header append helpers
- preserve `AuthCookieError` export from the existing validation module
- preserve `underlay_http::cookies::*` compatibility
- preserve crate-root cookie exports
- update tests only where module parent imports need to become explicit

Out of scope:

- changing auth or CSRF cookie names, paths, or clear behavior
- changing SameSite/Secure/domain/path/name validation behavior
- retiring raw config setters
- hiding public `AuthCookieConfig` fields
- changing query, pagination, CORS, or error logging behavior
- consumer rollout unless public imports or raw setters move

## Acceptance Criteria

- `cookies.rs` becomes a small module front door
- public exports remain source-compatible
- `underlay-http` tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal module split. If public imports, raw setters, public config
fields, cookie strings, or validation behavior must move, stop and re-enter
planning.

## Current State

`g06.062` is next after `g06.061`.

## Next Task

Execute `g06.062`: HTTP cookies internal split.
