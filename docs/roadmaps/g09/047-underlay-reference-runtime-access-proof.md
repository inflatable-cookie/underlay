# g09.047 - Underlay Reference Runtime And Access Proof

Status: ready
Owner: Underlay Reference maintainers
Contracts: `024`, `025`, `026`, `030`, `031`
Found by: `g09.045`
Depends on: `g09.046`

## Purpose

Make Underlay Reference the proof anchor for env/secret authority, explicit
route families, cookie-backed mutation CSRF, and peer-aware client-IP policy.

## Promotion Gate

- [x] `g09.046` is complete (PR9, merge commit `7d8c0bae`)
- [x] Underlay `v0.9.5` publishes the env-authority tool (release commit
  `8ffafb92`; GitHub Release published 2026-08-27)
- [x] target checkout is clean and exactly aligned with `origin/main`
  (`854e5ad2`, verified 2026-08-27)
- [x] app owner chose environment-aware startup authority: `DATABASE_URL` and
  the JWT keypair are required; `ENCRYPTION_KEY` is additionally mandatory
  outside local/effigy/test; selected adapter/backend credentials are
  conditional
- [x] app owner allows CSRF disablement only in local/effigy/test; dev,
  staging, production, and unknown names fail closed
- [x] app owner requires fatal startup outside local/effigy/test for malformed
  config or `COOKIE_SECURE=false`; the non-deployed set may warn

## Settled Owner Policy

- Environment classes: local/effigy/test are the bounded non-deployed set;
  dev/staging/production are deployed, and unknown names retain Underlay's
  fail-closed production behavior.
- Required startup authority: `DATABASE_URL`, `AUTH_JWT_PRIVATE_KEY`, and
  `AUTH_JWT_PUBLIC_KEY` are required for the API. `ENCRYPTION_KEY` is required
  in deployed environments and may be absent with an explicit warning only in
  local/effigy/test. Redis, SMTP, SES/AWS, and storage credentials become
  required only when the corresponding backend or adapter is selected.
- CSRF: cookie-backed browser mutation protection may be disabled only in
  local/effigy/test. Every deployed or unknown environment rejects an attempted
  disablement.
- Startup failures: malformed layered config and insecure auth-cookie posture
  are fatal outside local/effigy/test. Local/effigy/test may warn where the
  bounded developer posture still permits startup.

## Scope

- add complete tracked env and required-secret authorities; remove the partial
  `.env.example` contract and contradictory `.env` instructions
- add the missing test-plan and OpenAPI exposure notes to the root story
- expose explicit runtime, shared, product/front, and admin router families
  without changing public URLs merely to match file names
- protect authenticated passkey registration and other cookie-backed browser
  mutations under the settled CSRF policy
- route auth/rate-limit/audit IP inputs through centralized peer-aware request
  context; remove handler-local forwarded-header trust
- align version middleware with the settled business/runtime boundary
- retain the rich crate profile, `AppState`, shutdown, and direct-router test
  support that already conform
- prove the new env/conformance surfaces and one security-relevant route slice

## Acceptance

- no runtime env key is undocumented and startup-critical keys are classified
- no tracked `.env` or `.env.example` is part of the runtime contract
- router source layout makes runtime/shared/product/admin ownership obvious
- authenticated passkey mutation cannot bypass required CSRF protection
- disabling CSRF cannot silently weaken a deployed cookie-auth runtime
- client IP used for abuse or auth policy is derived from a peer-aware context
- runtime endpoints do not require a business API-version header
- direct-router tests prove the changed security and family seams

## Validation

- `effigy tasks`
- env/secret and workspace conformance checks
- targeted API router/auth tests
- `effigy acme-api/health`
- `effigy test --plan`
- `effigy health`
- `effigy validate`
- `effigy qa`
- `git diff --check`

## Stop Conditions

Stop if the work needs an unapproved production secret classification, proxy
topology, public URL cutover, or weaker CSRF policy. Return that decision to the
Underlay Reference owner.

## Consumer Upgrade Impact

- Impact class: security hardening plus app-local docs/topology repair
- Affected consumer: Underlay Reference
- Required action: update any local secret bootstrap and browser mutation flow
  affected by the final policy
- Compatibility window: no URL cutover is authorized by this roadmap

## Next Task

Run the worker handoff published on Underlay Reference `main` at `e4235876`.
After reviewed merge and exact-main verification, promote independent roadmaps
`g09.048`-`g09.052` whose decision gates are satisfied.
