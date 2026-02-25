# Roadmap 015/016 Closure Runbook (2026-02-25)

This runbook defines the canonical closure path for the two remaining in-progress roadmaps:

- `015` Unified Error Reporting
- `016` JSON Naming Standardization

## Single Command Entry Point

From `underlay`:

```bash
scripts/roadmap-015-016-closure.sh
```

Optional strict mode (fails if any external repo path is missing):

```bash
scripts/roadmap-015-016-closure.sh --strict-external
```

Optional runtime mode for `015` evidence capture in Acowtancy:

```bash
ACOWTANCY_API_BASE_URL=http://0.0.0.0:40001 scripts/roadmap-015-016-closure.sh --run-runtime
```

## What The Script Verifies

1. `016` naming guardrails:
- `check-json-naming.sh` for underlay + underlay-reference + acowtancy + compli-me + songsprout
- `check-compatibility-sunset.sh`

2. `015` route-level error pattern guardrails:
- `check-route-error-patterns.sh` for all consuming API route trees

3. Shared auth stability baseline:
- underlay auth crate regression sweep with all features

4. Optional runtime validation (`--run-runtime`):
- delegates to Acowtancy `scripts/validate-error-reporting.sh`

## Closure Evidence Checklist

- [ ] Closure script passes with no failures.
- [ ] Runtime error-reporting validation evidence captured (null-rate + smoke proof).
- [ ] Compatibility adapters removed or explicitly retained with valid sunset dates.
- [ ] `docs/roadmap/015-unified-error-reporting-roadmap.md` success metrics checked complete.
- [ ] `docs/roadmap/016-json-naming-standardization-roadmap.md` success metrics checked complete.
- [ ] `docs/roadmap/README.md` status counts updated.

## Notes

- OAuth credential-dependent checks are environment-specific and may be run outside this script.
- Use `--strict-external` in CI or release branches to prevent accidental skip-based pass.
