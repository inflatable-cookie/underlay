# g08.001 - Session Response Token Exposure

Status: complete
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Break the first link of the XSS -> persistent-takeover chain. The auth session
contract returns `accessToken` and `refreshToken` in the JSON body of every auth
response, including a plain `GET session()`, and `useAuth.init()` writes them
into a JS-readable store. In the httpOnly-cookie SvelteKit flow the 7-day
refresh token is still reachable: any XSS can `fetch(sessionRoute)` and
exfiltrate it, turning transient XSS into persistent account takeover. httpOnly
buys nothing while the endpoint echoes the token.

## Evidence

- `ts/src/client/auth.ts:25-30`, `76-79`
- `ts/src/client/useAuth.ts:53-56`
- cookie-mode lifetime at `ts/src/client/sveltekit.ts:78`

## Governing References

- [030 Auth and session systems](../../contracts/030-auth-and-session-systems.md)
- [090 TS runtime and client orchestration](../../contracts/090-ts-runtime-and-client-orchestration.md)

## Planned Changes

- [x] Stop returning `refreshToken` from session GET; in cookie mode omit token
  fields from the session body entirely.
- [x] Hand access/refresh tokens out only at the login and refresh endpoints,
  not on session read.
- [x] Adjust `useAuth`/token-store consumers to source tokens from
  login/refresh, not session GET.
- [x] Update the `030` contract and TS client guide to state the token-exposure
  boundary explicitly.

## Consumer Upgrade Impact

Impact class: `behavioral`. Consumers reading tokens off the session response
must move to the login/refresh path. Requires six-consumer proof per `023`.

## Validation

- [x] unit test asserting session GET response carries no `refreshToken` (and no
  token fields in cookie mode)
- [x] `bun x vitest run` (client auth suite)
- [x] `effigy validate`

## Stop Conditions

Stop and surface if any consumer genuinely depends on session-GET tokens for a
flow that has no login/refresh equivalent; that is a contract decision, not a
silent behavior change.

## Completion Notes

Completed 2026-07-17. `SessionInfo` (token-free) split from `AuthSession`;
`session()` strips any echoed token fields; `useAuth.init()` no longer writes
to the token store on session read. Contract `030` and guide `080` state the
token-exposure boundary. Validated with `bun x tsc` and the client auth vitest
suites (auth, useAuth, sveltekit, patterns/auth - all green). Full
`effigy validate` is blocked by the pre-existing red navigation unit tests
tracked in `g08.014`.

## Next Task

`g08.002` editor preview sanitization.
