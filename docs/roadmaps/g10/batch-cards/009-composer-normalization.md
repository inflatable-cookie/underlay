# g10.009 - Composer Normalization

Status: planned
Blocked by: `g10.005`
Owner: Composer maintainers
Spec: `docs/specs/monorepo-consumer-workspace-rollout.md`

## Scope

- Move `composer-api`, `composer-admin`, and `composer-front` into `apps/`.
- Move `composer-api-client` into `packages/`.
- Move `composer-docs` to root `docs/`.
- Consolidate three child locks and replace internal `file:` edges.
- Replace repeated child `bun install --no-cache` commands with one locked root
  setup; remove suppressed install failures.
- Remove old-path fallbacks, sibling Underlay validation, and unused `/underlay`
  symlink hacks.
- Update Effigy catalogs, bundle dirs, aliases, tests, docs paths, and
  instruction surfaces.

## Acceptance And Validation

- One frozen root install; workspace-shape check green.
- Child dev/build/check tasks are pure package tasks.
- `effigy health`, planned targeted tests/checks, and `git diff --check` green.

## Stop Conditions

Stop if removing an Underlay mount changes runtime compilation or if Effigy
cannot route package setup from the root.

## Next Task

Return a reviewable PR to the orchestrator; do not merge.
