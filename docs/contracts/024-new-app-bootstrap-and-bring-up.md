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

Live workspace evidence:

- `acowtancy` — the proven implementation of this contract: one Git root,
  `apps/cream`, `apps/dairy`, `apps/farmyard`, `packages/cattle-grid`,
  `packages/froyo`, root `docs/`, one root Bun manifest and lockfile, internal
  `workspace:*` edges, and released Underlay/Poodle dependencies.
- `underlay-reference` — the conformant bootstrap fixture after `g09.025`.
- `contact-patch`, `compli-me`, `songsprout`, `loophole/composer` — consumers
  normalized and independently checked through `g09.026`–`g09.030`.

Supporting shared contracts:

- [`025-rust-app-runtime-assembly-and-router-topology.md`](./025-rust-app-runtime-assembly-and-router-topology.md)
- [`110-admin-template-system.md`](./110-admin-template-system.md)
- [`120-tooling-testing-and-contract-artifacts.md`](./120-tooling-testing-and-contract-artifacts.md)

If a consumer workspace diverges from this contract, this contract wins. The
completed rollout spec is retained as historical evidence under `docs/specs/archive/`.

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
- `effigy state plan`
- `effigy state apply local --yes`

Rules:

- root tasks own cross-package orchestration;
- keep orchestration in Effigy rather than mirroring it into root package
  scripts;
- the frozen root install is an Effigy task, for example a
  `workspace:js:prepare` task running `bun install --frozen-lockfile`;
- local database and seed state is applied through the Effigy state stack
  (`effigy state plan`, `effigy state apply local --yes`), not through
  root-owned DB aliases;
- the API package owns the migration runner and exposes a `migration:*` front
  door; call it from the workspace root through child-catalog routing;
- do not reintroduce root `db:migrate` / `db:reset` aliases. They were retired
  from the live proof's task surface, and a root alias that shadows
  package-owned `migration:*` routing is drift.

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
- there are no child product repositories to declare — a workspace that needs a
  child bootstrap entry for an application or library has not converged on this
  contract;
- declared dev mounts for Underlay/Poodle are tooling mounts and do not change
  the committed dependency shape.

### External inputs and tooling mounts

Sibling checkouts and explicit read-only content inputs are not workspace
children.

Allowed non-workspace inputs:

- a sibling Underlay or Poodle checkout used for QA, conformance scripts, dev
  mounts, or untracked Cargo `[patch]` links written by `effigy deps link`
- an explicit read-only content or data input, such as a sibling corpus mounted
  for import, generation, or review

Rules:

- these inputs must not become JavaScript workspace members;
- they must not introduce nested Git metadata inside the product workspace;
- they must not be committed `file:` dependencies on Underlay or Poodle;
- they must not replace the released Underlay/Poodle dependency shape;
- they are not polyrepo support, nested product repos, or a second docs
  authority;
- name them in bootstrap docs or Effigy mounts when operators need them;
- do not list them in root `workspaces`.

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
- API packages document config precedence when they support layered config;
- the static env-authority checker proves those two files exist, parse, and
  relate; it does not read secret values or invent which keys are mandatory;
- live value presence stays with `scripts/check-env-manifest.sh` and must not
  become a CI requirement for material secrets.

### Bring-up rule

A new workspace has one boring bring-up path:

1. bootstrap the workspace
2. apply local state with `effigy state plan` then
   `effigy state apply local --yes`
3. run `effigy health`
4. run `effigy test --plan`
5. run `effigy dev` or `effigy dev <surface>`
6. use the API package's `migration:*` front door through catalog routing when
   schema work is needed

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
- internal packages or Underlay/Poodle wired with committed `file:` paths;
- a declared JavaScript workspace outside `apps/*` or `packages/*`;
- a `libs/*` directory standing in for `packages/*`;
- treating a tooling mount or read-only content input as a workspace child;
- docs authority spread across repositories or left implied;
- `config/env-manifest.txt` missing for runtime apps that still read env.

## Questions This Contract Should Settle

- How many repositories does a normal Underlay product use? One.
- Where do applications and reusable packages live? `apps/*` and `packages/*`.
- What is the exact root JavaScript manifest shape?
- How do Underlay and Poodle enter the dependency graph?
- What is the canonical first-run bring-up flow?

## Assessment State

Assessed across Underlay and all six consumer roots by `g09.045` on 2026-08-26.

Verdict: `conforming` after the `g09.046`–`g09.056` repair wave and exact fleet
closeout `g09.054`.

Current closeout state:

- all six roots conform to one Git root, `apps/*` / `packages/*`, explicit root
  workspaces, one root lock, released dependencies, root docs authority, and
  Effigy-first state/migration ownership
- every root carries complete `config/env-manifest.txt` and
  `config/required-secrets.txt` authority
- workspace-shape now rejects unsupported workspace prefixes and committed
  `file:` Underlay/Poodle edges; env/secret authority is a separate static
  check
- external read-only inputs and sibling tooling mounts are classified as
  non-workspace inputs

Shared wording and checker coverage were repaired in `g09.046`. Fleet adoption
and proof completed through `g09.054`; all six exact roots pass workspace and
env authority. See the
[`g09.045` assessment](../logs/2026-08/26-225903-g09-045-bootstrap-runtime-access-assessment.md).
The final exact-head matrix is in the
[`g09.054` closeout](../logs/2026-08/27-174415-g09-054-bootstrap-runtime-access-fleet-closeout.md).

## Next Task

`g09.057` is complete. Execute the authorised target-owned `g09.058` and
`g09.059` route-retirement lanes.
