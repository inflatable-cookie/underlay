# g06.178 Artifact - Six-consumer Rust API Compatibility Sweep

## Result

The current consumer family is compatible with the final Rust hardening surface.

The sweep found no live usage of the retired Rust APIs:

- `AuthCookieConfig::with_domain`
- `AuthCookieConfig::with_cookie_prefix`
- `AuthCookieConfig::with_refresh_token_path`
- `PostgresMediaConfig::with_schema`

The only remaining `AuthCookieConfig` field-like references are the intentional
accessor calls in `underlay-reference` custom CSRF helpers.

## Consumer Validation

- `underlay-reference`: root `effigy health` passed.
- `contact-patch`: root `effigy health` passed.
- `compli-me`: root `effigy health` passed.
- `acowtancy`: root `effigy health` passed with the known non-failing
  `farmyard-migration` dead-code warning.
- `songsprout`: root `effigy health` passed.
- `loophole/composer`: root `effigy health` passed.

## Notes

`contact-patch`, `songsprout`, and `loophole/composer` refreshed local Bun
lockfiles during health checks, but their git worktrees remained clean after
the checks.

Historical roadmap artifacts still mention retired APIs as delivery history.
No active guide or source usage needs repair.

## Next Lane

Move to `g06.179`: Rust hardening release-note and upgrade-guidance closeout.
