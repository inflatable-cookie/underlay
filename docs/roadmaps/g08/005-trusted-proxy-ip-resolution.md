# g08.005 - Trusted-Proxy IP Resolution

Status: complete
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Repair the client-IP trust boundary that neutralizes both rate limiting and
security alerting. `extract_ip_address` trusts `CF-Connecting-IP` -> `X-Real-IP`
-> first `X-Forwarded-For` entry with no trusted-proxy allowlist. The login
limiter keys on `email:ip`, so rotating XFF yields a fresh counter per request
(unbounded per-account brute force), and the same spoofed IP feeds per-IP alert
thresholds, so rotation keeps every per-IP counter near 1 and no alert fires.
Per-user account lockout is the only working backstop.

## Evidence

- `rust/crates/underlay-http/src/context/parse.rs:22-50`
- `rust/crates/underlay-auth-password/src/service/login.rs:31-34`
- `rust/crates/underlay-security-alerts/src/store.rs:9-40`

## Governing References

- [020 HTTP transport and server boundary](../../contracts/020-http-transport-and-server-boundary.md)
- [030 Auth and session systems](../../contracts/030-auth-and-session-systems.md)
- [033 Error codes and operator audit](../../contracts/033-error-codes-and-operator-audit.md)

## Planned Changes

- [x] Derive client IP from a configured trusted-proxy boundary
  (trusted-hop count / rightmost-untrusted XFF hop), not the leftmost header.
- [x] Add an email-only login limit alongside `email:ip`.
- [x] Add per-account and global alerting so IP rotation cannot keep every
  counter below threshold.

## Consumer Upgrade Impact

Impact class: `configuration`. Consumers must declare their trusted-proxy
setup. Requires six-consumer proof per `023`.

## Validation

- [x] tests: spoofed XFF does not reset the limiter; email-only limit trips;
  per-account alert fires under rotation
- [x] `cargo test -p underlay-http -p underlay-auth-password -p underlay-security-alerts`
- [x] `effigy validate`

## Stop Conditions

Stop if consumers deploy behind heterogeneous proxy topologies that a single
config model cannot express; escalate to a contract decision.

## Completion Notes

Completed 2026-07-17. `TrustedProxyConfig` (None default / CloudflareHeader /
RealIpHeader / ForwardedFor{trusted_hops}) installed as a request extension;
`RequestContext` resolves the client IP through it with socket-peer fallback
(`ConnectInfo`). Default trusts no forwarding headers. Spoofed-XFF test
proves a forged prefix cannot change the resolved IP. Email-only login limit
added (`PasswordConfig::rate_limit_email_max_attempts`, default 30/window)
checked before the `email:ip` key. Security alerts gain per-account
(`LoginFailuresForAccount`, `DistributedFailuresForAccount`) and global
(`GlobalLoginFailureSurge`) signals with `evaluate_account_alerts` /
`evaluate_global_alerts`, scoped store helpers, and migration
`0002__security_alert_scopes.sql`. Contract `020` and guide `068` updated.
Validated: `cargo test -p underlay-http -p underlay-auth-password
-p underlay-security-alerts` green.

## Next Task

`g08.006` internal error-header leak.
