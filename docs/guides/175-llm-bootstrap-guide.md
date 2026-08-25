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

## Step 1: Create or Bootstrap the Repository

For an existing starter repository, bootstrap the single root:

```bash
effigy bootstrap git@github.com:your-org/your-project.git
```

For a new repository:

```bash
mkdir my-project && cd my-project
git init
mkdir -p apps packages config docs
```

There is one Git root. Do not initialize repositories inside `apps/` or
`packages/`.

## Step 2: Create the Root JavaScript Manifest

Create `package.json` at the repository root. List only JavaScript packages
that own a manifest; the Rust-only `apps/api` directory is not a Bun workspace
member.

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

Keep one root `bun.lock`. Generate and verify it with the root Effigy install
task:

```bash
effigy workspace:js:prepare
```

Do not create child lockfiles or run package-by-package installs.

## Step 3: Declare Released Dependencies

In `apps/api/Cargo.toml`, pin Underlay crates to one release tag:

```toml
[workspace.dependencies]
underlay-core = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.4" }
underlay-http = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.4" }
underlay-auth = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.4" }
```

In each consuming app package such as `apps/admin` or `apps/front`, use the
same released tag and point at internal packages through `workspace:*`:

```json
{
  "dependencies": {
    "@inflatable-cookie/underlay": "git+ssh://git@github.com/inflatable-cookie/underlay.git#v0.9.4",
    "@myorg/client": "workspace:*"
  }
}
```

Use released Poodle package versions. Internal JavaScript edges use
`workspace:*`. Do not commit source paths, sibling dependency references, or
other local replacements.

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

## Step 5: Configure Secrets and Runtime Inputs

Create the root config and secret manifests required by the app:

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

## Step 6: Apply Local State and Schema

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

## Step 7: Install and Verify the Workspace

Install once from the repository root, then run the repo-owned baseline:

```bash
effigy workspace:js:prepare
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

## Step 8: Start the Application

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
