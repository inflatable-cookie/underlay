# g09.047 - Underlay Reference Runtime And Access Proof

Status: planned
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
- [ ] mandatory secret classes are named by the app owner per environment
- [ ] allowed CSRF-disable environments are explicit and fail closed elsewhere
- [ ] malformed deployed config and insecure deployed-cookie behavior have an
  explicit fatal/warn policy

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

Settle the three remaining app-owner security decisions, then prepare and push
the g09.047 worker handoff. After reviewed merge and exact-main verification,
promote independent roadmaps `g09.048`-`g09.052` whose decision gates are
satisfied.
