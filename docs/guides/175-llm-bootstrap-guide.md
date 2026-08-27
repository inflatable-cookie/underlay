# LLM Bootstrap Guide: From Zero to Working App

This is the current guided bootstrap path for a new Underlay consumer. Contract
[024](../contracts/024-new-app-bootstrap-and-bring-up.md) owns the guarantees;
[020](./020-project-structure.md) owns the detailed workspace layout and root
manifest.

## Prerequisites

- Git
- Effigy
- Bun `1.3.14`
- Rust and Cargo
- access to the workspace's declared secrets and local container services

An Underlay source checkout is not required. Underlay and Poodle enter the
workspace as released dependencies. Sibling checkouts may be mounted for QA
tooling, but they are not committed application dependencies.

## Workspace Shape

Create one Git repository with this role map:

```text
my-project/
├── apps/
│   ├── api/          # Rust backend and app-local Cargo workspace
│   ├── admin/        # Admin SvelteKit app
│   └── front/        # Product-facing SvelteKit app, when needed
├── packages/
│   ├── client/       # Shared TypeScript API client
│   └── ui/           # Shared UI package, when needed
├── config/
├── docs/
├── package.json
├── bun.lock
└── effigy.toml
```

Do not copy the pre-convergence physical layout of `underlay-reference`.
Use it for component and domain examples only; the workspace contract and the
strict rollout spec define the topology.

## Step 1: Bootstrap a Complete Repository

Start from a real project scaffold, not an empty Git repository. The scaffold
must already own the surfaces that the later commands invoke:

- one Git root with `apps/api`, `apps/admin`, `apps/front`, `packages/client`,
  and `packages/ui` directories for this guide's full role map;
- `apps/admin/package.json`, `apps/front/package.json`,
  `packages/client/package.json`, and `packages/ui/package.json`;
- the root `package.json`, `effigy.toml`, and initial `bun.lock`;
- a root Effigy catalog (including `workspace:js:prepare`, `dev`, `health`,
  `test --plan`, `validate`, and the local state stack), plus an API-package
  catalog exposing the routed `migration:*` tasks;
- `config/env-manifest.txt`, `config/required-secrets.txt`, and the committed
  state/config files required by the local stack.

Use the canonical bootstrap command against that scaffold:

```bash
effigy bootstrap git@github.com:your-org/my-project.git
cd my-project
```

`effigy bootstrap` performs the initial frozen install, so the scaffold must
already contain a matching root lockfile and `workspace:js:prepare` task. Verify
the complete shape before changing manifests or running state commands:

```bash
test -f package.json
test -f bun.lock
test -f effigy.toml
test -f apps/api/effigy.toml
test -f apps/admin/package.json
test -f apps/front/package.json
test -f packages/client/package.json
test -f packages/ui/package.json
test -f config/env-manifest.txt
test -f config/required-secrets.txt
rg -n 'workspace:js:prepare' effigy.toml infra state
rg -n 'migration:' apps/api/effigy.toml
```

If any check fails, return to the scaffold/bootstrap step. Do not replace it
with `mkdir` plus a partial root manifest: an empty repository does not yet
have an executable Effigy catalog, state stack, package graph, or lockfile.
There is one Git root; never initialize repositories inside `apps/` or
`packages/`.

## Step 2: Verify and Customize the Root JavaScript Manifest

The scaffold already contains `package.json` at the repository root. Keep its
workspace declaration in this shape: list only JavaScript packages that own a
manifest; the Rust-only `apps/api` directory is not a Bun workspace member.

```json
{
  "name": "@myorg/my-project",
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

The four workspace entries above must resolve to the package manifests checked
in Step 1. If an optional role is omitted, remove its workspace entry and all
references to it before regenerating the lockfile. Do not install yet; first
declare the released and internal dependencies below.

## Step 3: Declare Released Dependencies

In `apps/api/Cargo.toml`, pin Underlay crates to one release tag:

```toml
[workspace.dependencies]
underlay-core = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.5" }
underlay-http = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.5" }
underlay-auth = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.5" }
```

In each consuming app package such as `apps/admin` or `apps/front`, use the
same released tag and point at internal packages through `workspace:*`:

```json
{
  "dependencies": {
    "@inflatable-cookie/underlay": "git+ssh://git@github.com/inflatable-cookie/underlay.git#v0.9.5",
    "@myorg/client": "workspace:*"
  }
}
```

Use released Poodle package versions. Internal JavaScript edges use
`workspace:*`. Do not commit source paths, sibling dependency references, or
other local replacements.

Do not create child lockfiles or run package-by-package installs.

## Step 4: Rename the Project

Replace the placeholder name in the root and package manifests, Rust crate
names, TypeScript package names, schema names, cookie/token identifiers, and
display text. Keep the role paths stable:

- API routes and Rust crates: `apps/api/`
- TypeScript commands and types: `packages/client/`
- Admin routes: `apps/admin/`
- Product-facing routes: `apps/front/`

Search before editing. Review every replacement; do not rename generated files,
migration history, or unrelated prose mechanically.

## Step 5: Generate and Verify the Root Lockfile

After the manifests, dependencies, and project names are correct, generate the
single root lockfile once, then prove that the frozen Effigy task can reproduce
it:

```bash
bun install
test -f bun.lock
effigy workspace:js:prepare
```

Run both commands from the repository root. The first command is the only
non-frozen lockfile generation step; all later installs use the Effigy selector.

## Step 6: Configure Secrets and Runtime Inputs

Complete the scaffold's root config and secret manifests for the app:

- `config/env-manifest.txt` for runtime environment keys
- `config/required-secrets.txt` for startup-critical secrets
- committed config defaults and an ignored local override where supported

Initialize local secret storage through Effigy:

```bash
effigy secrets init
```

Runtime packages must not depend on committed `.env` files. Environment
injection is the highest-precedence override over committed defaults and local
config.

## Step 7: Apply Local State and Schema

Use the root state stack for database and seed state:

```bash
effigy state plan
effigy state apply local --yes
```

Schema execution stays API-owned. Run the routed API-package `migration:*`
front door when schema work is needed; for a package that declares it, the
normal local reset/replay example is:

```bash
effigy migration:reset
```

Do not add root or package `db:migrate`, `db:reset`, or `db:drop` aliases.

## Step 8: Verify the Workspace

The root install and frozen-lockfile proof are complete. Run the repo-owned
baseline:

```bash
effigy tasks
effigy health
effigy test --plan
effigy validate
```

For the app-local Rust workspace, run the narrow check from its owning
directory:

```bash
cd apps/api
cargo check --workspace
cargo test --workspace
```

Return to the repository root before running root Effigy selectors.

## Step 9: Start the Application

Start the whole workspace through the root catalog:

```bash
cd /path/to/my-project
effigy dev
```

To start one surface, use its catalog-qualified package task:

```bash
effigy <admin-package>/dev
effigy <front-package>/dev
```

The root README should document local URLs, gateway hosts, and any container
notes that are specific to the workspace.

## Customization Checklist

- [ ] Root `package.json` is private and pins the Bun package manager.
- [ ] Root workspaces list every JavaScript package explicitly.
- [ ] `apps/api` owns the Cargo workspace; no root Cargo workspace is needed.
- [ ] `apps/admin`, `apps/front`, `packages/client`, and `packages/ui`
      have roles documented in the root README.
- [ ] One root `bun.lock` exists and no child lockfiles exist.
- [ ] Internal JavaScript dependencies use `workspace:*`.
- [ ] Underlay and Poodle use released dependencies.
- [ ] `config/env-manifest.txt` and `config/required-secrets.txt` exist
      where runtime packages read environment or secret values.
- [ ] Root state and API-package migration tasks are documented separately.
- [ ] `effigy health`, `effigy test --plan`, and `effigy validate` pass.

## Troubleshooting

### Cargo cannot find an Underlay crate

Check that every Underlay crate uses the intended released Git tag and that
`apps/api/Cargo.lock` was regenerated. Do not repair this with a source path.

### Schema or relation errors appear during bring-up

Run `effigy state plan`, apply the local state stack, then use the API
package's routed `migration:*` task. Confirm that durable migrations live
under `apps/api/migrations/` and dev overlays are separate.

### Secret or JWT errors appear at startup

Confirm the required keys are listed in `config/required-secrets.txt` and
that the local Effigy secret store is initialized. Keep generated values out of
Git.

### TypeScript imports fail after an Underlay upgrade

Run `effigy workspace:js:prepare` from the root and use explicit exports such
as `@inflatable-cookie/underlay/client/*`,
`@inflatable-cookie/underlay/runtime/*`, or
`@inflatable-cookie/underlay/patterns`.

## Full Reference

For the detailed layout, read [020-project-structure](./020-project-structure.md)
and [030-underlay-integration](./030-underlay-integration.md). For the
normative bootstrap and bring-up rules, read
[contract 024](../contracts/024-new-app-bootstrap-and-bring-up.md).
