# g09.052 - Acowtancy Runtime And Access Rollout

Status: complete
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
- [x] Acowtancy is clean and exactly aligned with `origin/main`
  (`7c592aab`, verified 2026-08-27)
- [x] environment-specific mandatory secrets are reconciled with
  `infra/secrets.toml` and container injection
- [x] Cream/Dairy cookie refresh/logout CSRF rollout is understood

## Settled Owner Policy

- `DATABASE_URL`, `AUTH_JWT_PRIVATE_KEY`, and `AUTH_JWT_PUBLIC_KEY` are required
  for Farmyard API startup. `ENCRYPTION_KEY` is additionally required outside
  local/effigy/test.
- OAuth, Google, SMTP, AI, blob, Vimeo, PDF, Stripe, and other integration
  credentials are conditional on the corresponding configured provider or
  runtime. `infra/secrets.toml` targets describe injection reach; they do not
  make every optional integration mandatory for every process.
- Keep the existing fail-closed database, deployed-cookie, CORS, encryption,
  email-adapter, AI-provider, and blob guards.
- Cream and Dairy call Farmyard directly through Cattle Grid. When refresh or
  logout falls back to the browser cookie, Farmyard requires the reference CSRF
  proof; SameSite alone is not the API security boundary. Body-token/native and
  bearer-only flows remain outside CSRF.

## Dispatch Evidence

- Target-owned handoff:
  `/Users/tom/Dev/projects/acowtancy/docs/handoffs/20260827-124253-g09-052-runtime-access-rollout.md`
- Pushed Acowtancy `main`: `d63e13d74036c2bc091a1887aa42e7fcbef6c282`
- Target docs/Northstar QA passed; no worker PR was open at dispatch. Execution
  remains owned by the separate Acowtancy thread.

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

## Completion Evidence

- Acowtancy PR
  [#62](https://github.com/acowtancy/market/pull/62) merged on 2026-08-27
  as `85c868e132407f86df0525086af90d5abf0fb7fc`.
- Reviewed worker head: `cf154ceefd6960163ab9ff3e942d06b80a64c091`.
- Canonical review:
  [PR comment](https://github.com/acowtancy/market/pull/62#issuecomment-5439636883).
- Exact-head review passed env/secret authority, Farmyard config, middleware,
  CSRF, version, OpenAPI, router, workspace checks, Cattle Grid type/tests,
  docs QA, Northstar QA, and `git diff --check`.
- The focused two-tab regression proves a second CSRF-token fetch does not
  invalidate the first tab. Existing public paths remain unchanged; the only
  added path is `GET /v1/auth/csrf-token`.
- Container-owned full health/QA could not prove the worktree because Docker
  Hub DNS failed and the running container mounted `main`. The narrower
  exact-head checks are recorded in the target execution log.
- Acowtancy `main` and `origin/main` both resolve to the merge commit.

## Next Task

Review and merge `g09.048`-`g09.051`. Keep `g09.053` blocked until those four
lanes and the Underlay Reference cross-tab CSRF follow-up are complete.
