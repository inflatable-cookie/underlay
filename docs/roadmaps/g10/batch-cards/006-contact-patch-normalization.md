# g10.006 - Contact Patch Normalization

Status: planned
Blocked by: `g10.005`
Owner: Contact Patch maintainers
Spec: `docs/specs/monorepo-consumer-workspace-rollout.md`

## Scope

- Move `cp-api`, `cp-admin`, and `cp-front` into `apps/`.
- Move `cp-client` and `cp-ui` into `packages/`.
- Move `cp-docs` to root `docs/`.
- Consolidate four child locks into one root lock.
- Replace internal `file:` edges with `workspace:*`.
- Update Effigy catalogs, bundle dirs, aliases, tests, config paths, docs, and
  instruction surfaces.

## Acceptance And Validation

- One frozen root install; workspace-shape check green.
- `effigy health`, planned targeted tests/checks, and `git diff --check` green.
- No old-path fallback or compatibility symlink.

## Stop Conditions

Stop on package-role ambiguity, release dependency failure, or scope-expanding
application behavior.

## Next Task

Return a reviewable PR to the orchestrator; do not merge.
