# 020 - Project Structure

This document covers the initial project setup, including directory layout, root configuration files, and the essential `AGENTS.md` that guides LLM interactions.

## Directory Layout

```
my-project/
├── .github/
│   └── workflows/
│       └── ci.yml                 # CI/CD configuration
├── .gitignore                     # Git ignore rules
├── README.md                      # Project overview
├── AGENTS.md                      # LLM interaction guide (CRITICAL)
│
├── apps/                          # Application layer
│   ├── bloom/                     # Artist-facing SvelteKit frontend
│   │   ├── src/
│   │   │   ├── routes/            # SvelteKit routes
│   │   │   ├── lib/               # App-specific code
│   │   │   └── app.html           # HTML shell
│   │   ├── package.json
│   │   ├── svelte.config.js
│   │   ├── vite.config.ts
│   │   └── tsconfig.json
│   │
│   ├── greenhouse/                # Admin/author SvelteKit frontend
│   │   ├── src/
│   │   │   ├── routes/
│   │   │   └── lib/
│   │   └── (similar structure to bloom)
│   │
│   └── nursery/                   # Rust API backend
│       ├── Cargo.toml             # Workspace manifest
│       ├── crates/
│       │   ├── api/               # HTTP server, handlers
│       │   ├── core/              # Core domain types
│       │   ├── auth/              # Authentication boundary
│       │   ├── db/                # Database utilities
│       │   └── infra/             # Infrastructure (config, tracing)
│       └── migrations/            # Database migrations
│
├── libs/                          # Shared libraries
│   ├── petal/                     # Shared Svelte UI kit
│   │   ├── src/
│   │   │   ├── components/        # Reusable Svelte components
│   │   │   ├── patterns/          # UI patterns (forms, lists)
│   │   │   └── styles/            # Design tokens, CSS
│   │   └── package.json
│   │
│   ├── stem/                      # Shared TypeScript API client
│   │   ├── src/
│   │   │   ├── http.ts            # HTTP client base
│   │   │   ├── commands/          # API command functions
│   │   │   └── types/             # Shared TypeScript types
│   │   └── package.json
│   │
│   └── underlay/                  # Underlay foundation (sibling or submodule)
│       ├── rust/
│       ├── ts/
│       └── docs/
│
└── trellis/                       # Documentation
    └── docs/
        ├── architecture/          # Architecture decisions
        ├── domain/                # Domain modeling docs
        ├── processes/             # Process documentation
        └── guides/                # How-to guides
```

## Step-by-Step Setup

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

This monorepo contains several related projects:

- `apps/bloom/` – artist-facing SvelteKit frontend.
- `apps/greenhouse/` – admin/author SvelteKit frontend.
- `apps/nursery/` – Rust API backend.
- `libs/petal/` – shared Svelte UI kit and design system.
- `libs/stem/` – shared TypeScript API client for Nursery.
- `trellis/` – system, domain, and process documentation.

> Root-scope rule for agents:
> Do **not** create or modify files directly in the repository root **except** this `AGENTS.md`.
> All new code, docs, and configuration must live inside the appropriate subdirectory.

## Project Structure & Module Organization

- App frontends: `apps/bloom/` and `apps/greenhouse/` (routes, Svelte components, assets).
  Co-locate UI, styles, and tests by feature.
- Backend: `apps/nursery/` (Rust crates, domain modules, HTTP handlers, integrations).
- Shared libraries: `libs/petal/` (UI components, design tokens) and `libs/stem/` (HTTP client, commands, typed models).
- Documentation: `trellis/docs` (architecture, domain, processes, decisions).

## Build, Test, and Development Commands

- Bloom dev server: `cd apps/bloom && pnpm install && pnpm dev`.
- Greenhouse dev server: `cd apps/greenhouse && pnpm install && pnpm dev`.
- Nursery backend: `cd apps/nursery && cargo test` (tests) and `cargo run` (local API).
- Libraries: `cd libs/stem && pnpm test`, `cd libs/petal && pnpm test`.

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

### 4. Create pnpm-workspace.yaml

Create `pnpm-workspace.yaml` in the project root:

```yaml
packages:
  - 'apps/*'
  - 'libs/*'
```

### 5. Create Root package.json

Create `package.json` for workspace-level operations:

```json
{
  "name": "my-project",
  "private": true,
  "version": "0.0.1",
  "description": "My project following the Songsprout/Acowtancy architecture",
  "scripts": {
    "install:all": "pnpm install && cd apps/bloom && pnpm install && cd ../greenhouse && pnpm install && cd ../../libs/petal && pnpm install && cd ../stem && pnpm install",
    "check:all": "cd apps/bloom && pnpm check && cd ../../apps/greenhouse && pnpm check && cd ../../libs/petal && pnpm check && cd ../stem && pnpm check",
    "test:all": "cd apps/bloom && pnpm test && cd ../../apps/greenhouse && pnpm test && cd ../../libs/petal && pnpm test && cd ../stem && pnpm test"
  },
  "engines": {
    "node": ">=20.0.0",
    "pnpm": ">=9.0.0"
  }
}
```

### 6. Create Initial GitHub Actions

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

A full-stack application following the Songsprout/Acowtancy architecture.

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
# Install dependencies
pnpm install:all

# Set up database
cd apps/nursery/crates/db
sqlx database create
sqlx migrate run

# Run development servers
cargo run -p nursery-api  # Backend (port 3000)
cd apps/bloom && pnpm dev  # Frontend (port 5173)
cd apps/greenhouse && pnpm dev  # Admin (port 5174)
```

## Documentation

- [Architecture Overview](./trellis/docs/architecture/)
- [Domain Modeling](./trellis/docs/domain/)
- [Process Documentation](./trellis/docs/processes/)
- [Quickstart Guide](./docs/guides/quickstart/)
```

## Directory Creation Script

Run the following to create the full directory structure:

```bash
#!/bin/bash
set -e

PROJECT_ROOT="$(pwd)"

# Create apps directories
mkdir -p apps/bloom/src/{routes,lib,components,assets}
mkdir -p apps/greenhouse/src/{routes,lib,components,assets}
mkdir -p apps/nursery/crates/{api,core,auth,db,infra}/src
mkdir -p apps/nursery/migrations

# Create libs directories
mkdir -p libs/petal/src/{components,patterns,styles,hooks}
mkdir -p libs/stem/src/{http,commands,types,utils}

# Create docs directories
mkdir -p trellis/docs/{architecture,domain,processes,decisions}
mkdir -p docs/guides/quickstart/code

# Create GitHub workflows
mkdir -p .github/workflows

echo "Directory structure created at $PROJECT_ROOT"
```

## Next Step

With the project structure in place, proceed to [030-underlay-integration](./030-underlay-integration.md) to link the Underlay foundation.
