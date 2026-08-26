# g10.014 - HTTP Client Bounded Constructor Fallback

Status: planned
Owner: repo maintainers
Contract: `020-http-transport-and-server-boundary.md`
Found by: `g10.011`

## Purpose

Keep the Rust outbound HTTP client timeout guarantee true when an infallible
constructor encounters invalid builder input.

## Scope

- remove raw `reqwest::Client::new()` fallback paths from `HttpClient::new()`
  and `HttpClient::with_user_agent()`
- preserve the existing infallible and fallible public constructor families
- ensure fallback behavior retains Underlay connect and total timeouts
- add focused invalid-user-agent and fallback-path tests

## Acceptance

- every client returned by an Underlay infallible constructor retains bounded
  connect and total timeouts
- `try_new()` and `try_with_user_agent()` still surface builder errors
- invalid custom user-agent input cannot silently produce an unbounded client
- external-profile SSRF behavior and redirect limits remain unchanged

## Validation

- `effigy rust:test`
- `effigy rust:clippy`
- `effigy health`
- `git diff --check`

## Stop Conditions

Stop if preserving the guarantee requires a breaking constructor signature.
Compile consumer impact before changing the public API.

## Consumer Upgrade Impact

- Impact class: compatible hardening
- Affected consumers: Rust callers using `HttpClient::with_user_agent()`
- Required action: none unless they depended on invalid user-agent strings being
  accepted through the raw fallback

## Next Task

Promote after the envelope and page-list repairs.
