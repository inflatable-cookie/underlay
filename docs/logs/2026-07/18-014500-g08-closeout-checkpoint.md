# 2026-07-18 - g08 closeout checkpoint

## Generation summary

`g08` (Audit Remediation And Edge Hardening) responded to the July 2026 deep
audit across five lanes. **31 of 32 cards are done**; `g08.019` is blocked on
external infrastructure (below).

- **Lane A - security and edge hardening (`g08.001`-`g08.010`):** session token
  exposure, editor preview sanitization, open-redirect guard, upload
  content-type/size enforcement, trusted-proxy IP resolution, internal
  error-header leak, CORS mirror-origin gating, distributed rate-limit backend,
  HTTP-client SSRF/timeout defaults, auth hardening batch.
- **Lane B - correctness and test gate (`g08.011`-`g08.014`):** form-feedback
  clobber, Google login dead handler, media validation bypass + upload
  cancellation, red unit suite fix + test gate.
- **Lane C - Rust structural seams (`g08.015`-`g08.020`):** error taxonomy, media
  domain-type relocation, pagination collision (new `underlay-query` crate via
  `g08.017b`), `underlay-auth-state-postgres` rename, workspace dependency + lint
  hygiene. `g08.019` blocked.
- **Lane D - TypeScript surface (`g08.021`-`g08.024`):** SSR-global state guard,
  export-map diet, EntityList real generic + dedup'd render body + single
  fetch-dedup key + debounced search + refetching affordance, `noImplicitAny`
  enabled with `EntityListItemContext` exported and dep hygiene.
- **Lane E - docs/versioning/i18n posture (`g08.025`-`g08.030`):** front-door doc
  repair, committed-artifact cleanup, contract-sync decision (kept envelope YAML +
  drift-check test), versioning/consumer-pin story (`0.8.0`, `v0.8.0` tagged),
  i18n resolved English-only, archival docs weight reduction (layer model fixed +
  archival designation).

## Blocked: g08.019 (postgres adapter integration tests)

The only non-complete card. `TestDb` is testcontainers-backed and needs Docker;
this environment has no Docker, no standalone Postgres, and no `DATABASE_URL`, so
the adapter contract tests cannot run. Writing unrunnable tests would ship
unverified SQL assumptions, so the card is parked. **Unblock:** provision a CI job
(or local Docker/Postgres) that runs migrations + the adapter tests against real
Postgres. This is the sole remaining g08 obligation before `g09`.

## Fleet and versioning state

- Six consumers kept in sync and typecheck-verified throughout (acowtancy/dairy,
  underlay-reference/acme-admin, compli-me/admin, loophole/composer-admin,
  contact-patch/cp-admin, songsprout/greenhouse+bloom+stem). A pre-existing
  songsprout/stem media-mapper break was fixed in passing during g08.023.
- Version bumped `0.0.1 -> 0.8.0`; `v0.8.0` annotated tag pushed at the
  six-consumer proof point. Path deps remain the default lockstep workflow; the
  git-tag hold-back path is documented in contract `023`.

## Validation (closeout)

- `cargo check --workspace`: clean.
- `effigy validate`: clean - svelte-check 0 errors (2472 files), guardrails,
  component hygiene, poodle prop-name check.
- 742 unit + 33 component tests pass (unit grew 739 -> 742 with the
  envelope-contract-drift guard).

## Next

Provision Postgres/CI to unblock `g08.019`, then scope `g09`. `g08` otherwise
closed across all five lanes.
