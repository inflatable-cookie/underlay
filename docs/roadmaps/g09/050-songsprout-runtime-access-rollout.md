# g09.050 - Songsprout Runtime And Access Rollout

Status: complete
Owner: Songsprout maintainers
Contracts: `024`, `025`, `026`, `030`, `031`
Found by: `g09.045`
Depends on: `g09.047`

## Purpose

Repair Songsprout's env authority, elevated-role and middleware seams, runtime
docs, and direct-router proof without forcing its artist product vocabulary into
generic front naming.

## Promotion Gate

- [x] `g09.047` is merged and its exact reference proof is recorded
  (Underlay Reference PR5, merge commit `6af27837`)
- [x] Songsprout is clean and exactly aligned with `origin/main`
  (`e1fd46ef`, verified 2026-08-27)
- [x] rate-limit backend failure is explicitly chosen as fail-open or fail-closed
- [x] Bloom/Greenhouse browser-cookie and framework CSRF posture is confirmed
- [x] advertised readiness is either implemented and consumed or removed

## Settled Owner Policy

- Rate-limit backend failure is fail-closed for protected abuse/auth flows.
  Backend failure must not silently grant the request.
- Bloom and Greenhouse retain SvelteKit's origin protection for their server
  actions. Nursery cookie-backed browser refresh/logout also gains the explicit
  reference CSRF seam; SameSite cookies and framework protection are not used
  as substitutes for the API boundary.
- Remove advertised readiness claims because Nursery mounts only `/health` and
  `/metrics` and no deployment consumer uses a readiness endpoint.
- Local/effigy/test are the bounded non-deployed set. Apply the reference
  deployed config/cookie failure policy and classify startup secrets from real
  runtime and selected-adapter requirements.

## Dispatch Evidence

- Target-owned handoff:
  `/Users/tom/Dev/projects/songsprout/docs/handoffs/20260827-124253-g09-050-runtime-access-rollout.md`
- Pushed Songsprout `main`: `87cb72df98e1ae28f3298b85f296283086137c8e`
- Target docs/Northstar QA passed; no worker PR was open at dispatch.

## Scope

- add env and required-secret authorities and remove nonexistent
  `.env.example` guidance
- replace package README `--repo .` instructions with supported selectors
- retain runtime/auth/artist/admin as a valid explicit rich family topology
- replace bespoke forbidden codes and handler-local superadmin checks with the
  canonical extractor boundary
- reorder middleware so request-id, tracing, and error logging observe policy
  middleware
- implement the chosen rate-limit failure posture and cover backend failure
- align architecture docs with actual runtime and remove the stale `/v1/ops`
  and readiness claims unless implemented
- add a bounded direct-router test-state proof

## Acceptance

- env/secret and workspace conformance pass
- artist remains the clear product/front equivalent
- elevated role and rejection policy is extractor-owned and canonical
- rate-limit failure follows a recorded security decision
- middleware policy failures retain request and trace context
- runtime docs name only mounted and supported endpoints
- a representative router test runs without `main()`

## Validation

- `effigy tasks`
- workspace and env/secret conformance
- targeted extractor, middleware, rate-limit, and router tests
- `effigy test --plan`
- package and root health/validate/QA
- `git diff --check`

## Stop Conditions

Stop if rate-limit failure, browser CSRF, readiness consumption, or secret
requiredness remains undecided. Do not silently choose a weaker security posture.

## Consumer Upgrade Impact

- Impact class: runtime/security hardening
- Affected consumer: Songsprout API and browser applications
- Required action: align operational monitoring and auth-abuse behavior with the
  recorded decisions
- Compatibility window: no route cutover is authorized

## Completion Evidence

- Songsprout PR [#5](https://github.com/inflatable-cookie/songsprout/pull/5)
  merged on 2026-08-27 as
  `e05ad04f986054647697f55c696850fda5fa694b`.
- Reviewed worker head: `4f348533ddb1e6505b8891dda01256580f701ac9`.
- Canonical review:
  [PR comment](https://github.com/inflatable-cookie/songsprout/pull/5#issuecomment-5439872643).
- Exact-head review passed the test plan, Nursery check, 11 API tests,
  touched-file rustfmt, and `git diff --check`.
- GitHub exposed no hosted checks. The only compiler warning was a pre-existing
  unused import outside the review correction.

## Next Task

Repair the Underlay Reference cross-tab CSRF prerequisite, then contribute this
reviewed merge evidence to `g09.053`.
