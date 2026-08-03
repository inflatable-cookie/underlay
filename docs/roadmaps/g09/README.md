# g09 - Config Convergence Follow-Through

Status: active
Owner: repo maintainers
Started: 2026-08-03

## Current Generation

`g09` acts on the 2026-08-03 self-audit of the config convergence (the
`effigy` dev environment, canonical env/CORS helpers, config overlays,
shared dev credentials). The convergence landed and is verified; the audit
found the follow-through work: small real gaps, dead code, remaining
duplication, and documented variants that should either converge or be
accepted deliberately.

Source audit: `docs/logs/2026-08/03-104132-config-convergence.md` and the
thread closeout review.

## Goals

- close the small real gaps (silent prod CORS, invisible legacy env vars,
  operator machines overriding shared config)
- remove dead and vestigial machinery (`with_environment_from_env`,
  cloned `CORS_ORIGINS` parses, inverted/legacy gates)
- write the config model's front-door guide
- converge or deliberately park the documented variants (songsprout seam,
  farmyard seed credentials, shell-tab env)

## Queue

1. [`g09.001`](001-prod-empty-origins-warning.md) — warn at boot when prod/staging CORS has no explicit origins
2. [`g09.002`](002-legacy-env-var-deprecation-signal.md) — deprecation signal for legacy env vars in `Environment::resolve`
3. [`g09.003`](003-operator-local-toml-strip-note.md) — operator note: strip stale `local.toml` overrides
4. [`g09.004`](004-retire-with-environment-from-env.md) — retire `with_environment_from_env`
5. [`g09.005`](005-admin-cors-layer-from-env.md) — `admin_cors_layer_from_env` + collapse `CORS_ORIGINS` clones
6. [`g09.006`](006-nursery-env-precedence-flip.md) — nursery `ENVIRONMENT_NAME` precedence flip
7. [`g09.007`](007-farmyard-dev-gate-decision.md) — farmyard `Dev` gate decision
8. [`g09.008`](008-config-model-guide.md) — config model front-door guide
9. [`g09.009`](009-songsprout-config-seam.md) — songsprout config seam alignment
10. [`g09.010`](010-farmyard-seed-bundle-credentials.md) — farmyard seed-bundle shared dev credentials
11. [`g09.011`](011-shell-tab-schema-env.md) — effigy shell-tab schema env propagation
12. [`g09.012`](012-build-time-environment-guard.md) — conformance guard: no `ENVIRONMENT` at build time

## Rules

- One card at a time; keep diffs minimal and pattern-consistent.
- Cards that change consumer-visible behavior include a `Consumer Upgrade
  Impact` section.
- Validation per card; underlay changes run `effigy validate`; consumer
  changes run `cargo check --workspace --all-features --all-targets` and
  `effigy qa:security` where applicable.

## Next Task

`g09.001` — prod-empty-origins boot warning.
