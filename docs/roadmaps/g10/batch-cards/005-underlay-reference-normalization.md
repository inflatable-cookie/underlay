# g10.005 - Underlay Reference Normalization

Status: planned
Blocked by: `g10.004`
Owner: Underlay Reference maintainers
Spec: `docs/specs/monorepo-consumer-workspace-rollout.md`

## Purpose

Make the bootstrap fixture physically match the monorepo contract.

## Scope

- Move `acme-api`, `acme-admin`, and `acme-front` into `apps/`.
- Move `acme-client` and `acme-ui` into `packages/`.
- Move `acme-docs` to root `docs/`.
- Replace four child locks with one root lock and eight `file:` edges with
  workspace or released dependencies as appropriate.
- Update Effigy catalogs, bundle dirs, aliases, tests, docs, Rhai paths, and
  instruction surfaces.
- Retire committed Underlay source dependencies; keep local co-development
  machine-local.

## Acceptance

- The fixture passes the workspace-shape check and one frozen root install.
- All root/package health and check surfaces pass.
- No compatibility symlinks or old-path fallbacks remain.

## Validation

- `effigy health`
- `effigy test --plan` and targeted package checks
- workspace-shape conformance check
- `git diff --check`

## Stop Conditions

Stop if released Underlay cannot compile the fixture or a package role is
ambiguous.

## Next Task

After merge, prepare independent handoffs for `g10.006`–`g10.009`.
