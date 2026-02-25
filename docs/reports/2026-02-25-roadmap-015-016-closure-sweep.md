# Roadmap 015/016 Closure Sweep (2026-02-25)

## Scope

Executed the combined closure command for remaining in-progress roadmaps:

- `015` Unified Error Reporting
- `016` JSON Naming Standardization

## Command

From `underlay`:

```bash
scripts/roadmap-015-016-closure.sh
```

Runtime evidence mode:

```bash
ACOWTANCY_API_BASE_URL=http://0.0.0.0:40001 scripts/roadmap-015-016-closure.sh --run-runtime
```

## Result

- Overall: `PASS`
- Failures: `0`
- Skips: `0`
- Runtime smoke path: `PASS`
- Runtime metric gate (015): `OPEN` (`handler_context` null-rate `85.71%` / `6 of 7` in last 24h)

## What Passed

1. `016` JSON naming guardrails:
- `check-json-naming.sh` on:
  - `underlay/rust`
  - `underlay-reference/acme-api/crates`
  - `acowtancy/farmyard/crates` (with `scripts/json-naming-allowlist.txt`)
  - `compli-me/api/crates`
  - `songsprout/nursery/crates`
- `check-compatibility-sunset.sh`

2. `015` route error pattern guardrails:
- `check-route-error-patterns.sh` on route trees for:
  - underlay-reference
  - acowtancy
  - compli-me
  - songsprout

3. Underlay auth regression baseline:
- `cargo test -p underlay-auth -p underlay-auth-password -p underlay-auth-jwt -p underlay-auth-totp -p underlay-auth-webauthn -p underlay-auth-oauth --all-features`

## Remaining To Fully Close 015/016

- `015`: collect production-like runtime evidence showing sustained reduction in `handler_context` null-rate and diagnosis-speed improvement.
- `016`: complete post-reset critical Acowtancy admin/integration validation and remove final compatibility adapters at cutover.
