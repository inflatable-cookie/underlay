# g06.176 Artifact - Auth Cookie Unchecked String Setter Retirement

## Result

`AuthCookieConfig` no longer exposes unchecked string setters for domain, cookie
prefix, or refresh-token path.

The remaining public construction surface is either fallible or typed:

- `try_with_domain`
- `try_with_cookie_prefix`
- `try_with_refresh_token_path`
- `with_cookie_domain`
- `with_refresh_cookie_path`

## Change

- Removed `with_domain`.
- Removed `with_cookie_prefix`.
- Removed `with_refresh_token_path`.
- Updated Underlay cookie tests to use checked builders.
- Invalid cookie config tests now assert early builder errors instead of late
  header-build errors.

## Consumer Impact

Classification: breaking public API tightening with completed current-family
rollout.

`g06.172` migrated current consumers to accessors and checked builders. The
post-change scan found no stale calls across Underlay or the current consumer
family.

## Validation

- `cargo test -p underlay-http`: passed.
- Current-family stale-call scan for unchecked cookie setters: clean.

## Next Lane

Move to `g06.177`: Rust hardening lane closeout and next architecture checkpoint.
