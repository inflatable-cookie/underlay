# g10.005 - Underlay Reference Normalization

Status: ready
Owner: Underlay Reference maintainers
Spec: `docs/specs/monorepo-consumer-workspace-rollout.md`

## Purpose

Make the bootstrap fixture physically match the monorepo contract.

## Scope

- Move `acme-api`, `acme-admin`, and `acme-front` into `apps/`.
- Move `acme-client` and `acme-ui` into `packages/`.
- Move `acme-docs` to root `docs/`.
- Replace four child locks with one root lock and four internal `file:` edges
  with `workspace:*`.
- Update Effigy catalogs, bundle dirs, aliases, tests, docs, Rhai paths, and
  instruction surfaces.
- Preserve the released Underlay `v0.9.4` and Poodle `0.2.2` application
  dependency boundary; keep local co-development machine-local.

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

## Readiness Evidence

Underlay Reference `main` at
`3354803ebc484d4d611878a8e60e356ab92e206e` has no open PR. Its recent
tagged-dependency adoption already removed committed sibling Underlay source
dependencies and passed the four JavaScript package validations, API build,
docs QA, and fresh frozen package installs.

The remaining topology baseline is exact: the root manifest is private but has
no pinned `packageManager` or workspaces; four child Bun locks remain; Admin
and Front carry four total internal `file:` edges; and the five owned package
roots plus `acme-docs` still use the retired top-level layout. The conformance
checker reports seven expected violations: four child locks and missing root
package-manager, root-lock, and workspaces fields.

The target's own `g01.007` strict lane remains planning authority for retained
surface semantics. This card may update its paths and front-door references as
part of the docs move, but must not execute or redefine that lane.

## Next Task

After merge, prepare independent handoffs for `g10.006`–`g10.009`.
