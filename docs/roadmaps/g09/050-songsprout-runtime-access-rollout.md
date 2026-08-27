# g09.050 - Songsprout Runtime And Access Rollout

Status: planned
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
- [ ] Songsprout is clean and exactly aligned with `origin/main`
- [ ] rate-limit backend failure is explicitly chosen as fail-open or fail-closed
- [ ] Bloom/Greenhouse browser-cookie and framework CSRF posture is confirmed
- [ ] advertised readiness is either implemented and consumed or removed

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

## Next Task

Record reviewed merge evidence for `g09.053`.
