# Contract: New App Bootstrap and Bring-Up

Status: active
Owner: repo maintainers
Depends on: `020-http-transport-and-server-boundary.md`, `025-rust-app-runtime-assembly-and-router-topology.md`, `110-admin-template-system.md`, `120-tooling-testing-and-contract-artifacts.md`

## Purpose

Define the single supported workspace topology and bring-up posture for a normal
Underlay app.

This contract covers:

- the normative single-repository workspace shape
- the normative root JavaScript manifest
- package and crate families
- docs authority placement
- Effigy-first bootstrap and bring-up posture
- dependency and local-mount expectations
- minimum day-one config and env surfaces

It does not define detailed route grammar, template APIs, or migration policy.
Those build on top of this layer.

## Sources of Truth

Normative rollout spec:

- [`docs/specs/monorepo-consumer-workspace-rollout.md`](../specs/monorepo-consumer-workspace-rollout.md)

Live workspace evidence:

- `acowtancy` — the proven implementation of this contract: one Git root,
  `apps/cream`, `apps/dairy`, `apps/farmyard`, `packages/cattle-grid`,
  `packages/froyo`, root `docs/`, one root Bun manifest and lockfile, internal
  `workspace:*` edges, and released Underlay/Poodle dependencies.
- `underlay-reference` — the bootstrap fixture. It has not converged on this
  contract yet; `g10.005` normalizes it. Do not copy its current physical
  layout.
- `contact-patch`, `compli-me`, `songsprout`, `loophole/composer` — consumers
  scheduled for normalization in `g10.006`–`g10.009`. Their present layouts are
  pre-contract evidence, not templates.

Supporting shared contracts:

- [`025-rust-app-runtime-assembly-and-router-topology.md`](./025-rust-app-runtime-assembly-and-router-topology.md)
- [`110-admin-template-system.md`](./110-admin-template-system.md)
- [`120-tooling-testing-and-contract-artifacts.md`](./120-tooling-testing-and-contract-artifacts.md)

If a consumer workspace diverges from this contract, the contract and the strict
spec win. A consumer that has not migrated yet is drift, not an alternative.

## Contract Goal

Underlay should make a new app workspace boring to start, with exactly one shape
to copy.

A normal team should not have to rediscover:

- how many repositories a product needs
- which packages belong in the workspace and where they sit
- where docs authority lives
- how Underlay and Poodle enter the dependency graph
- which env and config files must exist on day one

## Scope Boundary

In scope:

- workspace root topology and root manifest shape
- default package families
- root and package README/Effigy posture
- local bootstrap and bring-up flow
- app-local env-manifest and secret-bootstrap expectations
- root-level dev orchestration

Out of scope:

- app domain modeling
- detailed API route shape
- release policy
- migration policy beyond bring-up hooks

## Shared Boundary

### Workspace topology rule

A normal Underlay product is **one Git repository**. Polyrepo layouts are
unsupported.

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

Rules:

- one Git root owns the whole product workspace;
- runtime applications live under `apps/*`;
- reusable internal libraries live under `packages/*`;
- docs authority is the root `docs/` directory;
- the root owns one `README.md`, one `effigy.toml`, one `package.json`, and one
  `bun.lock`;
- no nested Git repositories, Git submodules, or child bootstraps inside the
  workspace;
- no symlinked or vendored source copies of Underlay or Poodle inside the
  workspace;
- names may be product-specific, but roles must not become implicit.

Normal package family:

- `apps/<api>` — Rust backend
- `apps/<admin>` — admin SvelteKit app
- `apps/<front>` — optional product-facing SvelteKit app
- `packages/<client>` — shared TypeScript API client
- `packages/<ui>` — optional shared UI package
- `docs/` — docs authority

Allowed naming variation: `apps/farmyard`, `apps/dairy`, `apps/cream`,
`packages/cattle-grid`, `packages/froyo`. The ownership split must still map
back to the family above, and the root README must state which directory plays
which role.

### Root JavaScript manifest rule

The root manifest is the only JavaScript workspace declaration:

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

- list only JavaScript packages that own a manifest — a Rust-only app directory
  is not a workspace member;
- use explicit workspace paths, not glob patterns;
- keep `packageManager` and `workspaces` at the root;
- keep exactly one `bun.lock` at the root and no child lockfiles;
- run one frozen root install through Effigy, not per-package installs.

### Rust ownership rule

Rust workspaces stay application-local.

Rules:

- the Rust `Cargo.toml` workspace lives inside the owning `apps/<api>` directory
  when that is its natural boundary;
- do not hoist a Cargo workspace to the repository root to mirror the JavaScript
  workspace;
- Rust crates inside an app use normal in-app path members.

### Dependency rule

Underlay and Poodle are consumed as released dependencies.

Rules:

- depend on Underlay through a pinned release tag on both language surfaces —
  `"@inflatable-cookie/underlay": "git+ssh://git@github.com/inflatable-cookie/underlay.git#vX.Y.Z"`
  for JavaScript, and `{ git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "vX.Y.Z" }`
  for Cargo;
- depend on Poodle through released package versions;
- use `workspace:*` for internal JavaScript dependencies between `apps/*` and
  `packages/*`;
- do not use `file:` dependencies for Underlay, Poodle, or internal packages;
- sibling Underlay/Poodle checkouts are a QA and tooling convenience only —
  they may back conformance scripts, dev mounts, and machine-local Cargo
  `[patch]` links written by `effigy deps link`, but they must never be the
  committed application dependency shape, and the generated `.cargo/config.toml`
  must stay untracked;
- pin the release tag everywhere it is declared so `bun.lock` and `Cargo.lock`
  stay reproducible.

### Docs authority rule

Docs authority is the root `docs/` directory.

Rules:

- do not leave docs authority implicit;
- do not split planning authority across a sibling docs repository;
- package READMEs may point at root `docs/` instead of duplicating it.

### Effigy-first root posture

The root workspace uses Effigy as the default command surface.

Expected root loop:

- `effigy tasks`
- `effigy health`
- `effigy test --plan`

Expected common root commands:

- `effigy dev`
- `effigy validate`
- `effigy qa`
- `effigy db:migrate`
- `effigy db:reset`

Rules:

- root tasks own cross-package orchestration;
- keep orchestration in Effigy rather than mirroring it into root package
  scripts;
- the frozen root install is an Effigy task, for example a
  `workspace:js:prepare` task running `bun install --frozen-lockfile`;
- DB tasks may resolve through catalog routing instead of being reimplemented at
  the root.

### Package-scoped Effigy posture

Package-scoped work runs through the root catalog, not through child repo
selectors.

Rules:

- address a package's tasks by their catalog-qualified selector, for example
  `effigy <package>/dev` or `effigy <package>/check`;
- when running from inside a package directory, run `effigy <task>` plainly;
- never add `--repo .` while already inside the target tree;
- package scripts may remain convenience wrappers, but are not the shared
  posture.

### Bundle and workspace-config rule

When a workspace uses the shared Effigy bundle model, the root `effigy.toml`
declares:

- `bundle.base`
- host/project metadata
- `bundle.dirs`
- local system mounts

Rules:

- `bundle.dirs` maps the real docs/api/client/ui/front/admin ownership split onto
  `apps/*`, `packages/*`, and `docs/` paths;
- there are no child repos to declare — a workspace that needs a child bootstrap
  entry has not converged on this contract;
- declared dev mounts for Underlay/Poodle are tooling mounts and do not change
  the committed dependency shape.

### Bootstrap rule

A normal Underlay workspace supports first-time setup through:

- `effigy bootstrap <repo-url>`
- optional `--start`

Rules:

- bootstrap clones one repository and prepares the whole workspace;
- bootstrap runs the frozen root dependency install;
- bootstrap leaves the workspace ready for immediate health or dev commands;
- `--start` is the explicit opt-in for launching the live dev stack.

### Environment and config rule

A normal workspace exposes at least:

- `config/env-manifest.txt` for runtime app packages that still read env
- `config/required-secrets.txt` for startup-critical secret/runtime keys
- API config layering notes where the API supports typed config files

Expected API config posture when applicable:

1. committed defaults
2. local override file
3. env injection highest precedence

Rules:

- runtime packages must not rely on undocumented env keys;
- runtime packages must not depend on committed `.env` files;
- admin/front packages document the minimum public env surface;
- API packages document config precedence when they support layered config.

### Bring-up rule

A new workspace has one boring bring-up path:

1. bootstrap the workspace
2. run `effigy health`
3. run `effigy test --plan`
4. run `effigy dev` or `effigy dev <surface>`
5. use `db:migrate` or `db:reset` from the root when DB ownership is routed

Rules:

- the root README makes this path obvious;
- package READMEs refine local loops without fighting the root story;
- local URLs, gateway hosts, and container/dev notes belong in the root
  workspace docs when they matter.

## What Good Looks Like

Good outcomes:

- one repository, one root manifest, one lockfile, one docs authority;
- `apps/*` and `packages/*` make every package role obvious;
- Underlay and Poodle arrive as released dependencies with a reproducible lock;
- Effigy is the obvious root and package-scoped loop;
- bootstrap and bring-up do not depend on tribal memory.

Bad outcomes:

- nested Git repositories, submodules, or symlinked source dependencies;
- child `bun.lock` files or per-package installs;
- internal packages wired with `file:` paths;
- a `libs/*` directory standing in for `packages/*`;
- docs authority spread across repositories or left implied;
- `config/env-manifest.txt` missing for runtime apps that still read env.

## Questions This Contract Should Settle

- How many repositories does a normal Underlay product use? One.
- Where do applications and reusable packages live? `apps/*` and `packages/*`.
- What is the exact root JavaScript manifest shape?
- How do Underlay and Poodle enter the dependency graph?
- What is the canonical first-run bring-up flow?

## Next Task

Use this contract as the base for new-app scaffolding, checklist artifacts, and
the `g10` consumer normalization cards. Consumers that still use a pre-contract
layout are tracked in
[`docs/specs/monorepo-consumer-workspace-rollout.md`](../specs/monorepo-consumer-workspace-rollout.md).
