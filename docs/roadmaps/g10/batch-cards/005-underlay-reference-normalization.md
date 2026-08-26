# g10.005 - Underlay Reference Normalization

Status: complete
Completed: 2026-08-26
Owner: Underlay Reference maintainers
Spec: `docs/specs/archive/monorepo-consumer-workspace-rollout.md`

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

## Review And Merge Evidence

PR [#3](https://github.com/inflatable-cookie/underlay-reference/pull/3)
merged to Underlay Reference `main` as
`40924bc93fc9bf29a0a5d686cd1870f728ca48ce` on 2026-08-26. The reviewed
implementation head was `e337bd9e626076e9f8238aede3c49de11c7786d7`.

The fixture now uses the canonical `apps/*`, `packages/*`, and root `docs/`
shape, one root Bun manifest and lock, and internal `workspace:*` edges. Shared
bundle PR
[`underlay-effigy-bundle#1`](https://github.com/inflatable-cookie/underlay-effigy-bundle/pull/1)
merged first as `e680157eebbdb4a14e98b53bd3f9ec38b9a936b7`; the fixture consumes its
catalog-alias and root-workspace inputs with no local lifecycle override.

Frozen root install, health, docs QA, package checks, API build, retained
rollout checks, template/security conformance, workspace-shape conformance,
and diff hygiene passed. Full `effigy validate` retained only the recorded
Front Vitest path-routing baseline; Underlay and the other test targets passed.

## Next Task

Closed after review, operator-authorized merge, and merged-state verification.
Publish independent handoffs for ready cards `g10.006`–`g10.009`.
