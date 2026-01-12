# 020 - Project Structure

This document covers the initial project setup, including directory layout, root configuration files, and the essential `AGENTS.md` that guides LLM interactions.

## Modes

This quickstart supports two layouts:

- **Multi-repo workspace (default):** separate git repos checked out side-by-side.
- **Monorepo:** a single git repo with `apps/*` and `libs/*`.

### Multi-repo workspace layout (default)

A local folder containing multiple git repositories:

```
myapp-workspace/
├── underlay/            # Foundation (git repo)
├── myapp-api/           # Rust API backend (git repo)
├── myapp-client/        # TypeScript API client (git repo)
├── myapp-ui/            # UI kit (git repo, optional)
├── myapp-web/           # SvelteKit frontend (git repo)
└── myapp-admin/         # SvelteKit admin (git repo)
```

Notes:

- There is no required “root repo” in this mode.
- Each repo can have its own `AGENTS.md` and CI.

### Monorepo layout

A single repository containing all apps and libs:

```
my-project/
├── .github/
├── AGENTS.md
├── apps/
│   ├── web/
│   ├── admin/
│   └── api/
├── libs/
│   ├── ui/
│   ├── client/
│   └── underlay/
└── trellis/
```

### Path mapping convention

To keep examples readable, some docs use monorepo-style logical paths like `apps/api/...` and `libs/client/...`.

#### Quick Reference Table

| Logical Path (in docs) | Monorepo | Multi-repo |
|------------------------|----------|------------|
| `apps/api/...` | `<project-root>/apps/api/...` | `<api-repo>/...` |
| `apps/web/...` | `<project-root>/apps/web/...` | `<web-repo>/...` |
| `apps/admin/...` | `<project-root>/apps/admin/...` | `<admin-repo>/...` |
| `libs/client/...` | `<project-root>/libs/client/...` | `<client-repo>/...` |
| `libs/ui/...` | `<project-root>/libs/ui/...` | `<ui-repo>/...` |
| `libs/underlay/...` | `<project-root>/libs/underlay/...` | `<underlay-repo>/...` |

**Example**: If a guide mentions `apps/api/src/main.rs`:
- Monorepo: Open `my-project/apps/api/src/main.rs`
- Multi-repo: Open `myapp-api/src/main.rs`

- In **multi-repo mode**, interpret these as paths *within the corresponding repo*:
  - `apps/api/...` → `<api-repo>/...`
  - `apps/web/...` → `<web-repo>/...`
  - `apps/admin/...` → `<admin-repo>/...`
  - `libs/client/...` → `<client-repo>/...`
  - `libs/ui/...` → `<ui-repo>/...`
  - `libs/underlay/...` → `<underlay-repo>/...`

## Step-by-Step Setup (Monorepo)

If you are using the default **multi-repo** layout, treat this section as guidance to apply per-repo (or create a thin meta-repo just for workspace scripts/CI if you want).

### 1. Initialize Git Repository

```bash
# Create project directory
mkdir -p my-project
cd my-project

# Initialize Git
git init

# Create initial commit
echo "# My Project" > README.md
git add .
git commit -m "Initial project structure"
```

### 2. Create .gitignore

Create `.gitignore` with the following content:

```gitignore
# === OS ===
.DS_Store
Thumbs.db

# === Rust ===
target/
Cargo.lock

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
yarn-debug.log*
yarn-error.log*

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

### 3. Create Root AGENTS.md

**This file is critical** for LLM interactions. Create `AGENTS.md` in the project root:

```markdown
# Repository Guidelines

This project may be set up either as:

- a **multi-repo workspace** (default), or
- a **monorepo** (`apps/*` and `libs/*`).

In a monorepo, the main projects live at:

- `apps/web/` – SvelteKit web frontend.
- `apps/admin/` – SvelteKit admin frontend.
- `apps/api/` – Rust API backend.
- `libs/ui/` – shared UI kit and design system.
- `libs/client/` – shared TypeScript API client.
- `trellis/` – system, domain, and process documentation.

In a multi-repo workspace, these are separate repos checked out side-by-side; treat the `apps/*` and `libs/*` paths in docs as logical names and map them to the appropriate repo root.

> Root-scope rule for agents:
> Prefer keeping new code inside `apps/` and `libs/`.
> Root-level files are allowed when they are standard repo plumbing (e.g. `README.md`, `.gitignore`, `pnpm-workspace.yaml`, root `package.json`, and `.github/workflows/*`).

## Project Structure & Module Organization

- App frontends: `apps/bloom/` and `apps/greenhouse/` (routes, Svelte components, assets).
  Co-locate UI, styles, and tests by feature.
- Backend: `apps/nursery/` (Rust crates, domain modules, HTTP handlers, integrations).
- Shared libraries: `libs/petal/` (UI components, design tokens) and `libs/stem/` (HTTP client, commands, typed models).
- Documentation: `trellis/docs` (architecture, domain, processes, decisions).

## Build, Test, and Development Commands

- Bloom dev server:
  - Monorepo: `cd apps/bloom && pnpm install && pnpm dev`
  - Multi-repo: `cd myapp-bloom && pnpm install && pnpm dev`
- Greenhouse dev server:
  - Monorepo: `cd apps/greenhouse && pnpm install && pnpm dev`
  - Multi-repo: `cd myapp-greenhouse && pnpm install && pnpm dev`
- Nursery backend:
  - Monorepo: `cd apps/nursery && cargo test` and `cargo run -p myapp-api`
  - Multi-repo: `cd myapp-api && cargo test` and `cargo run -p myapp-api`
- Libraries:
  - Monorepo: `cd libs/stem && pnpm test`, `cd libs/petal && pnpm test`
  - Multi-repo: `cd myapp-stem && pnpm test`, `cd myapp-petal && pnpm test`

When changing Rust code in Nursery, prefer running:

- `cargo fmt --all`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test` for the narrowest relevant set of crates (or the whole workspace when appropriate).

When changing TypeScript/Svelte code in Bloom, Greenhouse, Petal, or Stem, prefer running:

- `pnpm lint`
- `pnpm check`
- `pnpm test`

Run the narrowest relevant commands before opening a PR.

## Coding Style & Naming Conventions

- TypeScript/JavaScript (bloom/greenhouse/petal/stem): 2-space indentation;
  components `PascalCase.svelte`; helpers `kebab-case.ts` with `camelCase` identifiers.
- Rust (nursery): use `rustfmt` defaults; modules and files `snake_case`,
  types and enums `PascalCase`.
- Docs: Markdown with `kebab-case` filenames; keep sections short and skimmable.

## Testing Guidelines

- Prefer small, fast unit tests close to the code, plus a few integration tests for cross-module flows.
- Mirror file names in test files (e.g., `Foo.svelte` → `Foo.test.ts`,
  Rust module `foo` → `foo_tests` or `tests/foo.rs`).
- Focus coverage on domain and API behavior; UI snapshot tests should be minimal and stable.

## Commit, PR, and Security Guidelines

- Write clear, imperative commit messages (e.g., `Add stem playlist commands`,
  `Refine nursery auth flow`).
- For non-trivial changes, add documentation in `trellis/docs/`.
- Keep secrets and credentials out of the repo; use `.env` files.

## Assistant Interaction Preferences

When working against an established plan, treat short replies like "Yes", "Yes, do it",
"Continue", "Carry on with that", or similar as explicit approval to proceed with
the next concrete item and actually implement it, not just restate the plan.

**"Continue" protocol:** Treat `Continue` as a codeword that accepts the assistant's
most recently proposed "next step".

Semantics:

- The assistant should end each task-oriented message by proposing a **single next concrete step**.
- If the user's message is exactly `Continue` (optionally surrounded by whitespace)
  and contains no other instructions, interpret it as: "go ahead with the last next-step
  suggestion you just described", and execute it (code/tests/docs) rather than restating or renegotiating the plan.

This protocol applies across sessions for this repository.
```

### 4. (Monorepo only) Create pnpm-workspace.yaml

Create `pnpm-workspace.yaml` in the project root:

```yaml
packages:
  - 'apps/*'
  - 'libs/*'
```

### 5. (Monorepo only) Create Root package.json

Create `package.json` for workspace-level operations:

```json
{
  "name": "my-project",
  "private": true,
  "version": "0.0.1",
  "description": "My project following this architecture",
  "scripts": {
    "install:all": "pnpm install",
    "check:all": "pnpm -r --if-present check",
    "test:all": "pnpm -r --if-present test"
  },
  "engines": {
    "node": ">=20.0.0",
    "pnpm": ">=9.0.0"
  }
}
```

### 6. CI (Monorepo vs Multi-repo)

- **Multi-repo (default):** each repo usually has its own `.github/workflows/*`.
- **Monorepo:** you can run Rust + TS checks from one workflow.

Below is a simple monorepo example.

### 6. (Monorepo example) Create Initial GitHub Actions

Create `.github/workflows/ci.yml`:

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  rust:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Cache Rust dependencies
        uses: actions/cache@v4
        with:
          path: ~/.cargo
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      - name: Run Rust tests
        run: |
          cd apps/nursery
          cargo test
          cargo clippy --all-targets --all-features -- -D warnings

  typescript:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: pnpm/action-setup@v4
        with:
          version: 9
      - uses: actions/setup-node@v4
        with:
          node-version: 20
          cache: 'pnpm'
      - name: Install dependencies
        run: pnpm install:all
      - name: Run TypeScript checks
        run: pnpm check:all
```

### 7. Create README.md

Create `README.md`:

```markdown
# My Project

A full-stack application following this architecture.

## Architecture

This project uses a monorepo structure with:

- **apps/nursery/** - Rust API backend (Axum + Domain-Driven Design)
- **apps/bloom/** - Artist-facing SvelteKit frontend
- **apps/greenhouse/** - Admin SvelteKit frontend
- **libs/petal/** - Shared Svelte UI kit
- **libs/stem/** - Shared TypeScript API client

## Getting Started

### Prerequisites

- Rust 1.75+
- Node.js 20+
- pnpm 9+
- PostgreSQL 14+

### Setup

```bash
# Multi-repo (default): run per repo
cd myapp-web && pnpm install
cd myapp-admin && pnpm install
cd myapp-client && pnpm install

cd myapp-api/crates/db
sqlx database create
sqlx migrate run

cd myapp-api
cargo run -p myapp-api  # Backend (port 3000)

cd myapp-web && pnpm dev  # Web
cd myapp-admin && pnpm dev  # Admin

# Monorepo: run from repo root
pnpm install:all
cd apps/api/crates/db && sqlx database create && sqlx migrate run
cd apps/api && cargo run -p myapp-api
cd apps/web && pnpm dev
cd apps/admin && pnpm dev
```

## Documentation

- [Architecture Overview](./trellis/docs/architecture/)
- [Domain Modeling](./trellis/docs/domain/)
- [Process Documentation](./trellis/docs/processes/)
- [Quickstart Guide](./docs/guides/)
```

## Directory Creation Script

Run the following to create the full directory structure:

```bash
#!/bin/bash
set -e

PROJECT_ROOT="$(pwd)"

# Create apps directories
mkdir -p apps/web/src/{routes,lib,components,assets}
mkdir -p apps/admin/src/{routes,lib,components,assets}
mkdir -p apps/api/crates/{api,core,auth,db,infra}/src
mkdir -p apps/api/crates/db/migrations

# Create libs directories
mkdir -p libs/ui/src/{components,patterns,styles,hooks}
mkdir -p libs/client/src/{http,commands,types,utils}

# Create docs directories
mkdir -p trellis/docs/{architecture,domain,processes,decisions}
mkdir -p docs/guides/code

# Create GitHub workflows
mkdir -p .github/workflows

echo "Directory structure created at $PROJECT_ROOT"
```

## Next Steps

With the project structure in place, proceed to [030-underlay-integration](./030-underlay-integration.md) to link the Underlay foundation.
