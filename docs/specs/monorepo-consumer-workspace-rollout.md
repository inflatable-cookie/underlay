# Monorepo Consumer Workspace Rollout

Status: active
Owner: repo maintainers
Roadmap: `g10`
Authority mode: strict

## Outcome

Make the Acowtancy workspace shape the default and only supported topology for
normal Underlay consumers. Polyrepo layouts are retired.

The rollout is complete when Underlay's active contract and guide surfaces teach
one shape, the shape is mechanically checkable, and all six consumers conform.

## Canonical Shape

One Git repository owns the product workspace:

```text
.
├── apps/
│   ├── api/
│   ├── admin/
│   └── front/
├── packages/
│   ├── client/
│   └── ui/
├── docs/
├── package.json
├── bun.lock
└── effigy.toml
```

Names may remain product-specific. Roles and ownership may not become implicit.

The root JavaScript manifest uses this shape:

```json
{
  "name": "@org/project",
  "private": true,
  "packageManager": "bun@1.3.14",
  "workspaces": [
    "apps/admin",
    "apps/front",
    "packages/client",
    "packages/ui"
  ]
}
```

Rules:

- list only JavaScript packages that own a manifest;
- use explicit workspace paths;
- keep one root `bun.lock` and no child lockfiles;
- run one frozen root install through Effigy;
- use `workspace:*` for internal JavaScript dependencies;
- keep `packageManager` and `workspaces` at the root;
- keep Rust workspaces application-local when that is their natural boundary;
- consume Underlay through a released Git tag and Poodle through released
  packages;
- use sibling Underlay/Poodle mounts for QA and tooling only, never as the
  committed application dependency shape;
- keep orchestration in Effigy rather than mirroring it into root package
  scripts.

## Proven Reference

Acowtancy is the current live proof:

- one Git root;
- `apps/*`, `packages/*`, and root `docs/`;
- one explicit root Bun workspace manifest and lockfile;
- internal `workspace:*` edges;
- one `bun install --frozen-lockfile` Effigy setup task;
- released Underlay and Poodle application dependencies.

`g10.004` corrected its active README and AGENTS evidence without changing
manifests, locks, or application dependencies.

Underlay Reference remains the bootstrap fixture. `g10.005` makes its physical
shape match the contract before the remaining consumers migrate.

## Fleet Baseline

All five non-Acowtancy consumers are already one Git repository. None needs a
history merge. Each needs directory normalization, a real root Bun workspace,
one lockfile, and internal workspace edges.

| Consumer | Root manifest | Child locks | Internal dependency posture |
| --- | --- | ---: | --- |
| Underlay Reference | private placeholder | 4 | `file:` paths |
| Contact Patch | private placeholder | 4 | `file:` paths |
| Compli Me | private placeholder | 4 | `file:` paths plus empty API manifest |
| Songsprout | private placeholder | 4 | `file:` paths |
| Composer | private placeholder | 3 | `file:` paths and repeated child installs |

## Runway

1. `g10.001` — normative contract and front-door authority.
2. `g10.002` — migration contract and active guide normalization.
3. `g10.003` — consumer workspace-shape conformance check.
4. `g10.004` — Acowtancy evidence correction.
5. `g10.005` — Underlay Reference normalization.
6. `g10.006` — Contact Patch normalization.
7. `g10.007` — Compli Me normalization.
8. `g10.008` — Songsprout normalization.
9. `g10.009` — Composer normalization.
10. `g10.010` — six-consumer proof and distribution closeout.

`g10.001`–`g10.005` are serial. After `g10.005`, `g10.006`–`g10.009` may run
in parallel because they own separate repositories and have no shared mutable
files. `g10.010` waits for all consumer PRs.

## Migration Rules

For each consumer:

1. preserve package Git history with `git mv`;
2. move runtime applications into `apps/*`;
3. move internal JavaScript libraries into `packages/*`;
4. move the documentation authority to root `docs/` when it is still a named
   package directory;
5. declare explicit JavaScript workspaces in the root manifest;
6. replace child locks with one generated root lock;
7. replace internal `file:` dependencies with `workspace:*`;
8. update Effigy catalogs, bundle directories, source aliases, tests, docs,
   and config paths in the same PR;
9. leave app-local Cargo workspace ownership intact;
10. do not add compatibility symlinks or path fallbacks for the retired layout.

## Acceptance

- Contract `024` states that polyrepos are unsupported.
- Active bootstrap and integration docs use `apps/*` and `packages/*` only.
- The exact root `package.json` shape is documented once normatively and linked
  from narrative guides.
- A repo-owned check rejects nested Git repositories, child Bun locks, missing
  root workspace fields, internal `file:` dependencies, and absent
  `workspace:*` edges.
- Each consumer has one root manifest, one root lock, no child locks, and green
  repo-owned health/check surfaces.
- Underlay Build bootstrap guidance is redistributed after the canonical docs
  land.
- Historical logs and closed roadmaps remain unchanged.

## Stop Conditions

Pause the active worker and return to the orchestrator if:

- a package role cannot be classified as application or reusable package;
- a move requires splitting Git history or repository ownership;
- Bun cannot resolve the workspace with one frozen root lock;
- committed source dependencies are required for runtime correctness;
- an Effigy catalog cannot represent the new paths without a contract change;
- validation exposes unrelated application behavior that would widen the card.

## Next Task

Launch the published handoffs for ready cards `g10.006`–`g10.009`. Keep each
consumer in its own worktree, branch, handoff, and PR; relay each PR for review.
