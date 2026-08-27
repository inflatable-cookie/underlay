# g09.051 - Composer Runtime And Access Rollout

Status: complete
Owner: Composer maintainers
Contracts: `024`, `025`, `026`, `031`
Found by: `g09.045`
Depends on: `g09.047`

## Purpose

Repair Composer's env authority, middleware/test seams, route-family source
ownership, and admin action placement under explicit deployment and
compatibility decisions.

## Promotion Gate

- [x] `g09.047` is merged and its exact reference proof is recorded
  (Underlay Reference PR5, merge commit `6af27837`)
- [x] Composer is clean and exactly aligned with `origin/main`
  (`b7cafd9c`, verified 2026-08-27)
- [x] deployment proxy topology and authoritative client-IP source are named
- [x] delete-batch restore/purge wire compatibility is decided
- [x] environment-specific mandatory secrets and malformed-config policy are named

## Settled Owner Policy

- Socket peer remains the authoritative client-IP source. No forwarding header
  is trusted until Composer declares a concrete proxy topology and hop policy.
- Move restore and purge atomically to
  `/v1/admin/delete-batches/{id}/restore` and
  `/v1/admin/delete-batches/{id}`. No in-repo client calls the legacy paths, so
  no dual-path compatibility alias is retained.
- `COMPOSER_DATABASE_URL` and the JWT keypair are startup-required in deployed
  environments. Selected blob/backend credentials remain conditional.
- Malformed layered config is fatal in dev, staging, production, and unknown
  environments. Local/effigy/test may retain bounded warnings where the local
  posture permits startup.

## Dispatch Evidence

- Target-owned handoff:
  `/Users/tom/Dev/projects/composer/docs/handoffs/20260827-124253-g09-051-runtime-access-rollout.md`
- Pushed Composer `main`: `d24aee5143ced20e279369c6c966bbbc1977c69c`
- Target docs/Northstar QA passed; no worker PR was open at dispatch.

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

## Completion Evidence

- Composer PR
  [#5](https://github.com/inflatable-cookie/loophole-composer/pull/5) merged on
  2026-08-27 as `4ec74ecd5f20ccbf5bae8e32b4c39810a1da904a`.
- Reviewed worker head: `35739d024dc6fc880c6b15df8aee199cc7c454e8`.
- Canonical review:
  [PR comment](https://github.com/inflatable-cookie/loophole-composer/pull/5#issuecomment-5439333774).
- Exact-head review passed env, conformance, and security QA, 99 API tests, 35
  API-client tests, test-plan inspection, and `git diff --check`.
- GitHub exposed no hosted checks. The recorded Docker Hub DNS failure was
  environmental; equivalent exact-head host checks passed.

## Next Task

Repair the Underlay Reference cross-tab CSRF prerequisite, then contribute this
reviewed merge evidence to `g09.054`.
