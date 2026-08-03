# g09.012 - Conformance Guard: No ENVIRONMENT At Build Time

Status: ready
Owner: repo maintainers

## Purpose

The bundle env schema applies to all task runs, including builds. Nothing
reads `ENVIRONMENT` at build time today (front/admin public config comes
from the config stack), but nothing prevents a future build from baking
`effigy` into a production artifact. Cheap guard.

## Evidence

- `docs/logs/2026-08/03-104132-config-convergence.md` ("Watch item")
- Audit item 12

## Planned Changes

- [ ] `underlay/scripts/check-consumer-conformance.sh`: new
  `build-env-read` check — grep consumer front/admin build configs and
  build scripts (vite config, build tasks, Dockerfiles) for `ENVIRONMENT`
  reads; flag any outside an explicit allowlist.
- [ ] Run green across all six consumers; document the allowlist mechanism.

## Consumer Upgrade Impact

Impact class: `additive` (new conformance check; consumers stay green
unless they already read `ENVIRONMENT` at build time, which would be a
real finding).

## Validation

- [ ] `effigy qa:security` green in all six repos (18 checks)

## Stop Conditions

None expected.

## Next Task

Generation closeout or `g10` scoping (maintainer direction).
