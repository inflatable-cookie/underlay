# 020 - Project Structure

This document covers the initial project setup, including directory layout, root configuration files, and the essential `AGENTS.md` that guides LLM interactions.

## Workspace Shape

Underlay consumers use one layout: a **single Git repository** with `apps/*`,
`packages/*`, and a root `docs/`. Polyrepo workspaces are unsupported — no
nested Git repositories, no submodules, no symlinked Underlay checkout, and no
committed `file:` source dependencies.

[Contract 024](../contracts/024-new-app-bootstrap-and-bring-up.md) owns the
normative topology, the exact root `package.json` shape, and the dependency
rules. This guide shows how to build it.

```
my-project/
├── .github/
├── AGENTS.md
├── README.md
├── apps/
│   ├── api/          # Rust backend
│   ├── admin/        # Admin SvelteKit app
│   └── front/        # Product-facing SvelteKit app (optional)
├── packages/
│   ├── client/       # Shared TypeScript API client
│   └── ui/           # Shared UI package (optional)
├── config/
├── docs/
├── package.json
├── bun.lock
└── effigy.toml
```

**Key characteristics:**

- one Git repository, one history, one CI surface
- one root `package.json` declaring explicit workspaces
- one root `bun.lock`; no child lockfiles and no per-package installs
- internal dependencies use `workspace:*`
- Underlay and Poodle arrive as released dependencies, never as in-tree source
- docs authority is the root `docs/` directory

**Naming convention:** directory names may be product-specific, but roles must
stay recognizable and the root README must say which directory plays which role.
`acowtancy` uses `apps/farmyard` (api), `apps/cream` (front), `apps/dairy`
(admin), `packages/cattle-grid` (client), and `packages/froyo` (ui).

### Path mapping convention

Guides use generic role names. Map these onto your project's directories:

| Generic name | Purpose | Location |
|--------------|---------|----------|
| `api` | Rust API backend | `apps/api/` |
| `front` | User-facing SvelteKit frontend | `apps/front/` |
| `admin` | Admin SvelteKit frontend | `apps/admin/` |
| `client` | TypeScript API client | `packages/client/` |
| `ui` | Shared Svelte UI components | `packages/ui/` |
| `docs` | System documentation | `docs/` |

**Example**: if a guide mentions "the api's main.rs", open
`apps/api/crates/api/src/main.rs`.

---

## Step-by-Step Setup

Replace generic names with your project's chosen names.

### 1. Create the repository

```bash
mkdir -p my-project && cd my-project
git init
mkdir -p apps packages docs config
```

There is exactly one `git init` in this procedure. If you find yourself running
a second one inside `apps/` or `packages/`, stop — that is the retired layout.

### 2. Create the root manifest

`package.json` at the repository root is the only JavaScript workspace
declaration:

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

List only JavaScript packages that own a manifest. `apps/api` is Rust-only, so
it is not a workspace member. Use explicit paths rather than globs.

### 3. Declare dependencies

Internal packages use `workspace:*`. Underlay comes from a released Git tag and
Poodle from released package versions:

```json
{
  "name": "@myorg/admin",
  "dependencies": {
    "@myorg/client": "workspace:*",
    "@myorg/ui": "workspace:*",
    "@inflatable-cookie/underlay": "git+ssh://git@github.com/inflatable-cookie/underlay.git#v0.9.4",
    "@inflatable-cookie/poodle-core": "0.2.2",
    "@inflatable-cookie/poodle-svelte": "0.2.2"
  }
}
```

Do not use `file:` for Underlay, Poodle, or internal packages. A sibling
Underlay checkout is a QA and tooling convenience only — it never becomes the
committed dependency shape.

### 4. Generate the single lockfile

```bash
bun install
```

Commit the resulting root `bun.lock`. Every later install runs frozen from the
root, normally through an Effigy task:

```bash
effigy workspace:js:prepare   # bun install --frozen-lockfile
```

### 5. Create the root AGENTS.md

**This file is critical** for LLM interactions. Create one `AGENTS.md` at the
repository root. Package-level `AGENTS.md` files are optional refinements, not
a replacement.

```markdown
# Repository Guidelines

This repository is the [product description] workspace.

## Workspace Map

- `apps/api/` – Rust API backend.
- `apps/front/` – user-facing SvelteKit frontend.
- `apps/admin/` – admin SvelteKit frontend.
- `packages/client/` – shared TypeScript API client.
- `packages/ui/` – shared Svelte UI kit (optional).
- `docs/` – system, domain, and process documentation authority.

Underlay and Poodle are released dependencies, not directories in this repo.

## Build, Test, and Development Commands

Prefer the repo's Effigy surface:

- `effigy tasks`
- `effigy health`
- `effigy test --plan`

Install once from the root: `bun install --frozen-lockfile`. Never run a
per-package install and never commit a child lockfile.

When changing Rust code, prefer running:

- `cargo fmt --all`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test` for the narrowest relevant set of crates.

When changing TypeScript/Svelte code, prefer running:

- `bun lint`
- `bun check`
- `bun test`

Run the narrowest relevant commands before opening a PR.

## Coding Style & Naming Conventions

- TypeScript/JavaScript: 2-space indentation;
  components `PascalCase.svelte`; helpers `kebab-case.ts` with `camelCase` identifiers.
- Rust: use `rustfmt` defaults; modules and files `snake_case`,
  types and enums `PascalCase`.
- Docs: Markdown with `kebab-case` filenames; keep sections short and skimmable.

## Testing Guidelines

- Prefer small, fast unit tests close to the code, plus a few integration tests for cross-module flows.
- Mirror file names in test files (e.g., `Foo.svelte` → `Foo.test.ts`,
  Rust module `foo` → `foo_tests` or `tests/foo.rs`).
- Focus coverage on domain and API behavior; UI snapshot tests should be minimal and stable.

## Commit, PR, and Security Guidelines

- Write clear, imperative commit messages.
- For non-trivial changes, add documentation.
- Keep secrets and credentials out of the repo; use the declared config and
  secret surfaces rather than committed `.env` files.

## Assistant Interaction Preferences

When working against an established plan, treat short replies like "Yes", "Yes, do it",
"Continue", "Carry on with that", or similar as explicit approval to proceed with
the next concrete item and actually implement it, not just restate the plan.

**"Continue" protocol:** Treat `Continue` as a codeword that accepts the assistant's
most recently proposed "next step".

This protocol applies across sessions for this repository.
```

Keep it lean — see [172-agents-files](./172-agents-files.md).

### 6. Create the root .gitignore

One `.gitignore` at the repository root:

```gitignore
# === OS ===
.DS_Store
Thumbs.db

# === Rust ===
target/

# === Node.js ===
node_modules/
.pnp
.pnp.js
dist/
build/

# === SvelteKit ===
.svelte-kit/

# === Environment ===
.env
.env.local
.env.*.local

# === IDE ===
.idea/
.vscode/
*.swp
*.swo

# === Logs ===
*.log
npm-debug.log*

# === Testing ===
coverage/
.nyc_output/

# === Database ===
*.sqlite
*.db

# === Secrets ===
*.pem
*.key
secrets/
```

Commit the root `bun.lock` and the API's `Cargo.lock`; both are part of the
reproducible workspace.

### 7. CI Configuration

Create one `.github/workflows/ci.yml` for the workspace:

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: oven-sh/setup-bun@v2
      - run: bun install --frozen-lockfile
      # Add workspace-specific steps...
```

---

## Directory Structure Templates

### `apps/api` (Rust)

```
apps/api/
├── Cargo.toml            # App-local Cargo workspace
├── Cargo.lock
└── crates/
    ├── api/src/          # HTTP handlers, router
    ├── core/src/         # Domain types, IDs
    ├── auth/src/         # Auth providers
    ├── db/
    │   ├── src/
    │   └── migrations/
    └── infra/src/        # Config, logging
```

The Cargo workspace stays app-local. Do not hoist it to the repository root to
mirror the Bun workspace.

### `packages/client` (TypeScript)

```
packages/client/
├── package.json          # workspace member; no lockfile
├── tsconfig.json
└── src/
    ├── index.ts
    ├── utils/
    │   ├── http-client.ts
    │   └── client-factory.ts  # configureClient + getClient
    ├── types/
    │   ├── common-types.ts
    │   └── auth-types.ts
    └── commands/
        ├── core-commands.ts
        └── auth-commands.ts
```

### `apps/front` / `apps/admin` (SvelteKit)

```
apps/front/
├── package.json          # workspace member; no lockfile
├── svelte.config.js
├── vite.config.ts
├── tsconfig.json
└── src/
    ├── app.html
    ├── app.d.ts
    ├── hooks.server.ts       # Calls configureClient() at module load
    ├── routes/
    │   ├── +layout.svelte
    │   └── +page.svelte
    └── lib/
        ├── components/
        └── utils/
```

**Note:** Frontends import `getClient()` directly from `packages/client` via its
`workspace:*` dependency. Do not create local `$lib/api/client.ts` wrappers that
duplicate the shared package's functionality.

### `docs/`

```
docs/
├── vision/
├── architecture/
├── guides/
├── patterns/
├── contracts/
├── processes/
├── roadmaps/
└── logs/
```

This is the workspace's single docs authority. It is a directory, not a package
and not a separate repository.

## Code Organization and Anti-God-File Policy

Use this policy from day one. Do not wait for files to become painful to navigate.

### File Size Thresholds

| File Type | Warning Threshold | Hard Limit | Action |
|---|---|---|---|
| Rust (`.rs`) | 500 lines | 900 lines | Split modules before merge (see `041-rust-module-splitting.md`) |
| TypeScript (`.ts`) | 300 lines | 500 lines | Split by domain responsibility and call site |
| Svelte (`.svelte`) | 250 lines | 400 lines | Move logic to helpers/stores and split UI into child components |
| SQL migration (`.sql`) | 250 lines | 400 lines | Split migration into focused steps/files when possible |

If a file is under the hard limit but difficult to read, still split it. Cohesion matters more than line count.

### Organization Rules (All Packages)

1. One file should have one primary job.
2. Group by feature/domain first, then by technical layer.
3. Keep route handlers thin and push business logic into service/query modules.
4. Keep UI pages orchestration-focused and push rendering into local feature components.
5. Keep command files scoped to one domain entity or workflow.
6. Co-locate tests with the unit being tested, or mirror structure in `tests/`.

### Recommended Structure by Layer

#### Rust API (`apps/api`)

Prefer this split for each domain:

```
crates/api/src/routes/admin/<domain>/
├── mod.rs                  # Router wiring only
├── list.rs                 # List endpoint handlers
├── get.rs                  # Single fetch handlers
├── mutations/
│   ├── create.rs
│   ├── update.rs
│   ├── delete.rs
│   └── reorder.rs          # When needed
└── validation.rs           # Live validation endpoints
```

Query and mutation logic belongs in `crates/db/src/<domain>/...`, not in route files.

#### TypeScript Client (`packages/client`)

Split commands by feature domain:

```
src/
├── commands/
│   └── <domain>/
│       ├── queries.ts
│       ├── mutations.ts
│       ├── validation.ts
│       └── reorder.ts
├── types/
│   └── <domain>-types.ts
└── utils/
    └── http-client.ts
```

Avoid monolithic `*-commands.ts` files that accumulate unrelated domains.

#### SvelteKit Apps (`apps/admin`, `apps/front`)

For each feature route, split page shell from feature implementation:

```
src/routes/(app)/<domain>/<feature>/
├── +page.ts
├── +page.svelte            # Orchestration only
├── _components/
│   ├── <Feature>List.svelte
│   ├── <Feature>Form.svelte
│   └── <Feature>Toolbar.svelte
├── _state/
│   └── <feature>-store.ts
└── _api/
    └── <feature>-client.ts # Optional view-specific adapters
```

Move business logic out of `+page.svelte` once it starts mixing fetch, transform, and UI concerns.

### Split Triggers (Refactor Immediately)

Split a file when any of these are true:

- More than one distinct workflow lives in the same file (for example: list + edit + reorder + trash).
- Tests are more than 30% of file size and reduce readability.
- A Svelte page has multiple modal/dialog flows and action handlers.
- A route/controller contains both HTTP wiring and domain rules.
- Reviewing the file requires frequent jumping across distant sections.

### Pull Request Gate

Before merging:

- [ ] No changed file exceeds hard limits above.
- [ ] New features were added as new files/modules, not appended to an existing god file.
- [ ] Route/page files remain orchestration-focused.
- [ ] Tests were added/updated in the same feature structure.
- [ ] If a temporary exception is needed, a follow-up split task is documented before merge.

---

## Next Steps

With the project structure in place, proceed to [030-underlay-integration](./030-underlay-integration.md) to link the Underlay foundation.
