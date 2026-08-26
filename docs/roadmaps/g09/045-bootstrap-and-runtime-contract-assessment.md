# g09.045 - Bootstrap And Runtime Contract Assessment

Status: planned
Owner: repo maintainers
Contracts: `024-new-app-bootstrap-and-bring-up.md`,
`025-rust-app-runtime-assembly-and-router-topology.md`,
`026-route-families-and-access-model.md`
Depends on: `g09.044`

## Purpose

Assess the proven monorepo family against the bootstrap, Rust runtime assembly,
and route/access contracts after the migration/testing repair wave closes.

## Promotion Gate

- `g09.037`–`g09.044` are complete
- the `g09.044` whole-app DB-harness decision is recorded
- all six consumer roots are current and their package roles remain explicit
- no open migration/testing repair changes the expected runtime baseline

## Scope

- inspect Underlay's bootstrap, runtime assembly, router topology, middleware,
  health/OpenAPI/metrics, and route-family authority
- inspect all six consumer roots and their affected `apps/*` packages
- compare implementation with every applicable rule in contracts `024`–`026`
- distinguish valid lean/rich runtime profiles from actual drift
- record one evidence matrix with ownership and confidence per finding
- compile later repair roadmaps only from confirmed findings
- keep the assessment read-only across consumer repositories

## Acceptance

- every `024`–`026` rule has an Underlay and fleet verdict
- every consumer root and affected API/admin/front package is represented
- workspace bootstrap, Rust runtime assembly, and access-model findings remain
  distinct rather than becoming one generic normalization list
- every finding is a contract match, documentation repair, bounded
  implementation candidate, consumer rollout candidate, or operator decision
- no repair is marked ready from assessment evidence that still needs a product
  or security-policy decision

## Validation

- Underlay `effigy health`
- Underlay `effigy qa:docs`
- Underlay `effigy qa:northstar`
- read-only six-root `effigy tasks` and targeted `effigy test --plan`
- `git diff --check`

## Stop Conditions

Stop and return to planning if the fleet exposes an unresolved lean-versus-rich
runtime policy, access-control decision, or app-local composition seam that
cannot be owned generically by Underlay.

## Consumer Upgrade Impact

- Impact class: assessment only
- Affected consumers: six-consumer family
- Required action: none until a finding becomes a later roadmap

## Next Task

After completion, compile only confirmed `024`–`026` repairs. Then choose the
next contract group from the remaining assessment order; do not promote the
collection or drift-prevention horizons by implication.
