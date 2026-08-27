# g09.049 - Compli Me Runtime And Access Rollout

Status: ready
Owner: Compli Me maintainers
Contracts: `024`, `025`, `026`, `030`, `031`
Found by: `g09.045`
Depends on: `g09.047`

## Purpose

Repair Compli Me's env authority, route-family assembly, elevated-role seam,
cookie mutation protection, and direct-router proof.

## Promotion Gate

- [x] `g09.047` is merged and its exact reference proof is recorded
  (Underlay Reference PR5, merge commit `6af27837`)
- [x] Compli Me is clean and exactly aligned with `origin/main`
  (`12fa0d17`, verified 2026-08-27)
- [x] `/v1/compli/*` is classified as shared multi-client or product/front
- [x] metrics exposure is explicitly enabled with a route or removed as unused
- [x] environment-specific mandatory secrets are named

## Settled Owner Policy

- `/v1/compli/*` remains the shared authenticated claim/verification family,
  matching the canonical endpoint-family matrix. Its product vocabulary does
  not require a front-family path cutover.
- Remove the unexposed metrics registry/state wiring. Do not invent a runtime
  `/metrics` surface without an operator consumer.
- Local/effigy/test are the bounded non-deployed set. Malformed config and
  insecure cookies fail startup in dev, staging, production, and unknown
  environments.
- `DATABASE_URL` and the JWT keypair are startup-required. `ENCRYPTION_KEY` is
  required in deployed environments; email, blob, and other adapter credentials
  are required only when their corresponding implementation is selected.
- Cookie-backed refresh/logout adopts the reference CSRF posture.

## Scope

- add env and required-secret authorities and remove references to a nonexistent
  `.env.example`
- replace package README `--repo .` instructions with root-catalog/package-local
  selectors
- split the flat root route chain into shallow runtime/shared/front-if-chosen/
  admin family builders without changing paths by implication
- move repeated superadmin policy into a dedicated extractor and canonical
  `auth.forbidden` rejection
- protect cookie-backed refresh/logout mutations with the reference CSRF posture
- document the existing API-version observability contract
- add an app-owned test-state/direct-router seam and representative integration
  proof
- retain the valid rich crate profile, baseline middleware order, config
  validation, and shutdown posture

## Acceptance

- env/secret and workspace conformance pass
- every route has explicit family ownership
- elevated-role checks are extractor-owned rather than handler-local
- browser cookie mutation cannot bypass CSRF
- version behavior and metrics posture are active, documented facts
- the router is testable without invoking `main()`

## Validation

- `effigy tasks`
- workspace and env/secret conformance
- targeted extractor, CSRF, router, and metrics/version tests
- `effigy test --plan`
- package and root health/validate/QA
- `git diff --check`

## Stop Conditions

Stop if route classification, metrics exposure, or secret requiredness is still
an operator/product decision. Do not invent a front family or operational
surface from source layout alone.

## Consumer Upgrade Impact

- Impact class: compatible topology/test hardening plus browser security change
- Affected consumer: Compli Me API and browser session callers
- Required action: supply CSRF proof for cookie refresh/logout
- Compatibility window: no public path cutover is authorized

## Next Task

Record reviewed merge evidence for `g09.053`.
