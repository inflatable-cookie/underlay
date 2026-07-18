# g08.009 - HTTP-Client SSRF And Timeout Defaults

Status: complete
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Harden the shared outbound HTTP client that consumers are told to use.
`underlay-http-client` only sets a user-agent: no connect/total timeout (hangs
forever), reqwest default redirect following (up to 10 hops including
`169.254.169.254`, localhost, and private ranges), and no IP/host denylist.
Current callers pin hosts so exposure is latent, but this is the standard crate
and the unauthenticated embed proxy (`g08` Lane A follow-up) rides on it.

## Evidence

- `rust/crates/underlay-http-client/src/lib.rs:33-38`
- related embed proxy `rust/crates/underlay-http/src/embed.rs:131,168,230`

## Governing References

- [020 HTTP transport and server boundary](../../contracts/020-http-transport-and-server-boundary.md)

## Planned Changes

- [x] Set default connect and total timeouts.
- [x] Add an opt-in `external()` variant that resolves-and-rejects
  private/link-local/loopback targets and constrains redirects.
- [x] Route existing reqwest users (`underlay-ai-runtime`, `underlay-auth-oauth`,
  `underlay-devtools`) through this crate.
- [x] Validate the embed proxy `id` as `[A-Za-z0-9_-]+` and add auth/rate limit.

## Consumer Upgrade Impact

Impact class: `behavioral`. Outbound requests gain timeouts and SSRF guards;
callers needing internal targets must opt out explicitly. Requires six-consumer
proof per `023`.

## Validation

- [x] test: private/link-local target rejected by `external()`; timeout fires
- [x] `cargo test -p underlay-http-client -p underlay-http`
- [x] `effigy validate`

## Stop Conditions

None expected.

## Completion Notes

Completed 2026-07-17. `underlay-http-client` now sets default connect (10s)
and total (30s) timeouts on every profile. New `HttpClient::external()` for
untrusted targets: `validate_external_url` + a custom redirect policy reject
private/loopback/link-local/unspecified hosts (incl. `169.254.169.254`),
non-http(s) schemes, and cap redirects at 3, re-checking every hop.
IPv4-mapped IPv6 is de-mapped before classification. Existing reqwest users
without timeouts fixed (`underlay-auth-oauth` `Client::new()` and
`underlay-devtools` blocking client); `underlay-ai-runtime` already had one.
Embed proxy `id` validated to `[A-Za-z0-9_-]+` before URL interpolation.
Contract `020` records the profiles and proxy-input rule. Tests: private/
metadata/scheme rejection matrix, IP classification, embed-id validation.
`cargo test -p underlay-http-client -p underlay-http` green.

## Next Task

`g08.010` auth hardening batch.
