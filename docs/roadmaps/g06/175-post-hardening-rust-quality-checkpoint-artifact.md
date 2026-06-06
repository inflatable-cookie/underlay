# g06.175 Artifact - Post-hardening Rust Quality Checkpoint

## Result

The Rust hardening lane should continue with one more bounded API-tightening
card.

The security-sensitive findings from `g06.169` are handled, and `g06.174`
removed the last public panic-on-invalid constructor found in the media Postgres
config surface. The remaining clear footgun is narrower: `AuthCookieConfig`
still exposes unchecked string setters even though the consumer family now uses
fallible or typed builders.

## Evidence

- Production panic scan now shows only invariant panics, inline-test residue, or
  docs/test examples.
- `PostgresMediaConfig::with_schema` is gone.
- Current consumer family has no direct `AuthCookieConfig` field access.
- Current consumer family has no unchecked `with_domain`,
  `with_cookie_prefix`, or `with_refresh_token_path` calls.
- Remaining unchecked cookie setter calls are only Underlay cookie tests.

## Decision

Queue `g06.176` to retire the unchecked string setters from
`AuthCookieConfig`.

Keep:

- `try_with_domain`
- `try_with_cookie_prefix`
- `try_with_refresh_token_path`
- typed setters such as `with_cookie_domain` and `with_refresh_cookie_path`

Remove:

- `with_domain`
- `with_cookie_prefix`
- `with_refresh_token_path`

## Validation

Checkpoint scans:

- production panic scan: complete
- current-family unchecked cookie setter scan: clean except Underlay tests
- current-family direct `AuthCookieConfig` field scan: clean except internal
  Underlay crate access and reference-app accessor calls

## Next Lane

Move to `g06.176`: auth cookie unchecked string setter retirement.
