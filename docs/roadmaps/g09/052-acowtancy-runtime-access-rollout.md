# g09.052 - Acowtancy Runtime And Access Rollout

Status: planned
Owner: Acowtancy maintainers
Contracts: `024`, `025`, `026`, `030`, `031`
Found by: `g09.045`
Depends on: `g09.047`

## Purpose

Repair Acowtancy's env/secret authority, Farmyard binary and middleware seams,
cookie mutation CSRF, and already-declared API-version posture without flattening
its valid rich runtime profile.

## Promotion Gate

- [x] `g09.047` is merged and its exact reference proof is recorded
  (Underlay Reference PR5, merge commit `6af27837`)
- [ ] Acowtancy is clean and exactly aligned with `origin/main`
- [ ] environment-specific mandatory secrets are reconciled with
  `infra/secrets.toml` and container injection
- [ ] Cream/Dairy cookie refresh/logout CSRF rollout is understood

## Scope

- add complete env and required-secret authorities; reconcile them with
  `env_keys.rs`, `infra/secrets.toml`, and active config docs
- extract the large OpenAPI registry/output helpers from `main.rs` while keeping
  runtime assembly visible
- reorder middleware so baseline request/tracing/error context surrounds policy
  middleware
- protect cookie-backed refresh/logout under the reference CSRF posture
- stop discarding the configured API version and give the client/OpenAPI-declared
  header a consistent business-route server posture
- repair the stale front-family GET-only comment
- retain rich runtime health, trusted-proxy request context, capability-scoped
  publishing, explicit route families, and existing test support

## Acceptance

- env/secret and workspace conformance pass
- required-secret declarations match real startup/container behavior
- `main.rs` is runtime assembly rather than an OpenAPI declaration registry
- policy failures retain request-id/tracing/error context
- cookie refresh/logout cannot bypass CSRF
- the configured/client-advertised version header is not discarded or silently
  ignored and does not burden runtime endpoints
- existing rich runtime and publishing seams remain intact

## Validation

- `effigy tasks`
- workspace and env/secret conformance
- targeted CSRF, middleware, version, OpenAPI, and router tests
- `effigy test --plan`
- Farmyard and root health/validate/QA through repo-owned Effigy selectors
- `git diff --check`

## Stop Conditions

Stop if required-secret classification or browser CSRF behavior is unresolved.
Do not weaken Farmyard's existing fail-closed runtime guards or peer-aware proxy
posture.

## Consumer Upgrade Impact

- Impact class: security/runtime hardening
- Affected consumer: Farmyard API plus Cream, Dairy, and Cattle Grid callers
- Required action: carry CSRF and declared-version behavior through the client
  surfaces
- Compatibility window: preserve existing business paths

## Next Task

Record reviewed merge evidence for `g09.053`.
