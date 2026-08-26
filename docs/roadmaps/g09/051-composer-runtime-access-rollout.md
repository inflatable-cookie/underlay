# g09.051 - Composer Runtime And Access Rollout

Status: planned
Owner: Composer maintainers
Contracts: `024`, `025`, `026`, `031`
Found by: `g09.045`
Depends on: `g09.047`

## Purpose

Repair Composer's env authority, middleware/test seams, route-family source
ownership, and admin action placement under explicit deployment and
compatibility decisions.

## Promotion Gate

- [ ] `g09.047` is merged and its exact reference proof is recorded
- [ ] Composer is clean and exactly aligned with `origin/main`
- [ ] deployment proxy topology and authoritative client-IP source are named
- [ ] delete-batch restore/purge wire compatibility is decided
- [ ] environment-specific mandatory secrets and malformed-config policy are named

## Scope

- add complete env and required-secret authority
- keep bearer-only auth and its valid no-CSRF posture
- reorder middleware to preserve baseline request/tracing/error context
- centralize policy-bearing client IP through the settled peer-aware seam
- add an app-owned test-state/direct-router integration seam
- move public product reads out of the admin source family
- place admin-gated delete-batch restore/purge under `/v1/admin/*` only through
  an approved client/server compatibility plan
- retain the valid lean crate profile, runtime endpoints, targeted rate limits,
  and path-only version policy

## Acceptance

- env/secret and workspace conformance pass
- middleware failures retain request-id/tracing/error context
- rate-limit and login IP cannot be influenced by untrusted forwarding headers
- public reads and admin operations have explicit family ownership
- admin-gated operator actions have an approved canonical path and retirement
  story
- representative runtime and route-family tests do not invoke `main()`

## Validation

- `effigy tasks`
- workspace and env/secret conformance
- targeted middleware, proxy, router, and compatibility tests
- `effigy test --plan`
- package and root health/validate/QA
- `git diff --check`

## Stop Conditions

Stop on unresolved proxy topology, secret requiredness, or delete-batch client
compatibility. Do not move a public path from source classification alone.

## Consumer Upgrade Impact

- Impact class: runtime hardening with a potentially breaking admin path cutover
- Affected consumer: Composer API, admin, front, and API client
- Required action: follow the approved dual-path or atomic cutover plan
- Compatibility window: must be chosen before implementation

## Next Task

Record reviewed merge evidence for `g09.053`.
