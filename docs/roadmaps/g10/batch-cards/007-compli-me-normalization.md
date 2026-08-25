# g10.007 - Compli Me Normalization

Status: planned
Blocked by: `g10.005`
Owner: Compli Me maintainers
Spec: `docs/specs/monorepo-consumer-workspace-rollout.md`

## Scope

- Move `api`, `admin`, and `front` into `apps/`.
- Move `api-client` and `ui` into `packages/`; retain root `docs/`.
- Delete the empty API `package.json`; do not make the Rust API a Bun workspace.
- Consolidate four child locks and replace internal `file:` edges.
- Update Effigy catalogs, bundle dirs, aliases, tests, config/QA paths, docs,
  and instruction surfaces.
- Validate whether Bun `1.3.14` can use the released Underlay tag; retain an
  exact SHA only with explicit evidence and a documented exception.

## Acceptance And Validation

- One frozen root install; workspace-shape check green.
- `effigy health`, planned targeted tests/checks, and `git diff --check` green.
- Any retained SHA exception is explicit and reproducible.

## Stop Conditions

Stop if tag resolution reproduces the prior Bun cache defect or requires an
unplanned release change.

## Next Task

Return a reviewable PR to the orchestrator; do not merge.
