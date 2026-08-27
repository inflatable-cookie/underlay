# g09.048 - Contact Patch Runtime And Access Rollout

Status: complete
Owner: Contact Patch maintainers
Contracts: `024`, `025`, `026`, `030`, `031`
Found by: `g09.045`
Depends on: `g09.047`

## Purpose

Adopt the reference env, router, CSRF, proxy, and declared-version posture in
Contact Patch while preserving its product-specific Book input and front family.

## Promotion Gate

- [x] `g09.047` is merged and its exact reference proof is recorded
  (Underlay Reference PR5, merge commit `6af27837`)
- [x] Contact Patch is clean and exactly aligned with `origin/main`
  (`8d5b6f4c`, verified 2026-08-27)
- [x] Book is classified as an external read-only content input or removed from
  bootstrap; it is not workspace ownership
- [x] fatal deployed config/cookie behavior is decided
- [x] public Book rate-limit posture is explicitly accepted

## Settled Owner Policy

- Book remains the declared external sibling, read-only input, and is not a Bun
  workspace or package-catalog member.
- Local/effigy/test are the bounded non-deployed set. Malformed config and
  insecure cookies fail startup in dev, staging, production, and unknown
  environments, matching the reference policy.
- `DATABASE_URL` and the JWT keypair are startup-required. `ENCRYPTION_KEY` is
  additionally required in deployed environments; selected adapter/backend
  credentials remain conditional.
- The cacheable public Book GET family has no app-local throttle. Deployment
  edge/abuse controls own its rate-limit posture; authenticated and mutation
  families retain their app-owned policy.

## Dispatch Evidence

- Target-owned handoff:
  `/Users/tom/Dev/projects/contact-patch/docs/handoffs/20260827-124253-g09-048-runtime-access-rollout.md`
- Pushed Contact Patch `main`: `3d7228c1444cc48ba9c419a7c4f76e9e644dcc29`
- Target docs/Northstar QA passed; no worker PR was open at dispatch.

## Scope

- add complete env and required-secret authority
- repair package/root docs, including `.env`, OpenAPI, and package-local Effigy
  instructions
- retain the valid Book front-family builder and make shared/admin assembly
  shallow and explicit
- protect authenticated passkey registration and fail closed on impermissible
  CSRF disablement
- replace handler-local forwarded-header parsing with peer-aware request context
- apply the already-advertised API-version header consistently on business
  routes and exempt runtime routes
- retain the rich crate profile, `AppState`, lean runtime surface, and existing
  direct-router test seam

## Acceptance

- contract `024` env/secret and docs checks pass
- Book remains outside workspace/dependency ownership unless explicitly adopted
  as a normal package
- shared/admin/front family ownership is evident from source
- cookie-backed authenticated mutations have the reference CSRF posture
- policy-bearing client IP cannot be forged through an untrusted peer
- the client-advertised version header is no longer silently ignored
- no public URL changes without an explicit compatibility plan

## Validation

- `effigy tasks`
- workspace and env/secret conformance
- targeted API auth, proxy, version, and Book route tests
- `effigy test --plan`
- package health/validate and root `effigy health`, `effigy validate`, `effigy qa`
- `git diff --check`

## Stop Conditions

Stop on unresolved Book ownership, production proxy topology, mandatory secret
classification, or public-route compatibility.

## Consumer Upgrade Impact

- Impact class: security/runtime hardening
- Affected consumer: Contact Patch API and client/browser callers
- Required action: preserve the declared version header and CSRF/browser flow
- Compatibility window: no route removal is authorized

## Completion Evidence

- Contact Patch PR
  [#5](https://github.com/contact-patch/contact-patch/pull/5) merged on
  2026-08-27 as `bc26676d6f5ab973c65dce4fc79046c66c210284`.
- Reviewed worker head: `4b37b2b735ac133fbee3d1031ee47a16d25060cd`.
- Canonical review:
  [PR comment](https://github.com/contact-patch/contact-patch/pull/5#issuecomment-5439333144).
- Exact-head review passed env/workspace authority, API and client checks, 50
  client/admin/front tests, test-plan inspection, and `git diff --check`.
- GitHub exposed no hosted checks. The recorded Docker Hub DNS failure was
  environmental; equivalent host checks passed.

## Next Task

Repair the Underlay Reference cross-tab CSRF prerequisite, then contribute this
reviewed merge evidence to `g09.053`.
