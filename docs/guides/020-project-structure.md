# 020 - Project Structure

This document covers the initial project setup, including directory layout, root configuration files, and the essential `AGENTS.md` that guides LLM interactions.

## Modes

This quickstart supports two layouts:

- **Multi-repo workspace (recommended):** separate git repos for each component, browsed from a common parent directory.
- **Monorepo:** a single git repo with `apps/*` and `libs/*` subdirectories.

### Multi-repo workspace layout (recommended)

A local folder containing multiple independent git repositories:

```
my-workspace/
├── underlay/            # Framework (git repo, often symlinked)
├── api-backend/         # Rust API backend (git repo)
├── api-client/          # TypeScript API client (git repo)
├── frontend-web/        # SvelteKit user frontend (git repo)
├── admin-web/           # SvelteKit admin frontend (git repo)
├── ui-kit/              # Shared UI kit (git repo, optional)
└── documentation/       # System documentation (git repo)
```

**Key characteristics:**
- Each folder is an independent git repository
- Repos are browsed together from a common parent directory for convenience
- No "root repo" or workspace package.json is required
- Each repo has its own `AGENTS.md`, CI, and dependencies
- Underlay is typically symlinked into the workspace

**Naming convention:** Choose thematic names that reflect your project's domain. For example:
- A music platform might use: `bloom/`, `greenhouse/`, `nursery/`, `stem/`, `trellis/`
- An education platform might use: `cream/`, `dairy/`, `farmyard/`, `cattle-grid/`, `ledger/`

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
└── docs/
```

This layout is suitable when you prefer a single git history or have many small apps/libs.

### Path mapping convention

To keep examples readable, docs use generic component names. Map these to your project's folder structure:

#### Quick Reference Table

| Generic Name | Purpose | Multi-repo Example |
|--------------|---------|-------------------|
| `frontend-web` | User-facing SvelteKit frontend | `bloom/`, `cream/`, `myapp-web/` |
| `admin-web` | Admin SvelteKit frontend | `greenhouse/`, `dairy/`, `myapp-admin/` |
| `api-backend` | Rust API backend | `nursery/`, `farmyard/`, `myapp-api/` |
| `api-client` | TypeScript API client | `stem/`, `cattle-grid/`, `myapp-client/` |
| `ui-kit` | Shared Svelte UI components | `petal/`, `myapp-ui/` |
| `documentation` | System documentation | `trellis/`, `ledger/`, `docs/` |

**Example**: If a guide mentions "the api-backend's main.rs":
- Multi-repo: Open `<your-api-backend-folder>/crates/api/src/main.rs`
- Monorepo: Open `apps/api/src/main.rs`

**Key principle**: Use your project's naming convention. The generic component names in docs map to whatever folder names make sense for your domain.

---

## Step-by-Step Setup (Multi-repo Workspace)

This section shows setup for a multi-repo workspace. Replace generic names with your project's chosen names.

### 1. Create Workspace Directory

```bash
# Create workspace directory
mkdir -p my-workspace
cd my-workspace

# Clone or create each component repo
# (Each folder below is its own git repository)
```

### 2. Create Component Repositories

For each component, create a separate git repository:

```bash
# Create api-backend repo
mkdir api-backend && cd api-backend && git init
echo "# API Backend" > README.md
git add . && git commit -m "Initial commit"
cd ..

# Create frontend-web repo
mkdir frontend-web && cd frontend-web && git init
echo "# Frontend Web" > README.md
git add . && git commit -m "Initial commit"
cd ..

# Repeat for other components...
```

### 3. Symlink Underlay

Link Underlay into your workspace (assuming Underlay is at a known location):

```bash
ln -s /path/to/underlay ./underlay
```

### 4. Create AGENTS.md in Each Repo

**This file is critical** for LLM interactions. Create `AGENTS.md` in each component's root.

Below is a template. Replace generic names with your project's equivalents:

```markdown
# Repository Guidelines

This repository contains the [component description].

## Related Repositories

This component is part of a multi-repo workspace:

- `frontend-web/` – user-facing SvelteKit frontend.
- `admin-web/` – admin SvelteKit frontend.
- `api-backend/` – Rust API backend.
- `api-client/` – shared TypeScript API client.
- `ui-kit/` – shared Svelte UI kit (optional).
- `documentation/` – system, domain, and process documentation.

## Build, Test, and Development Commands

[Add component-specific commands here]

When changing Rust code, prefer running:

- `cargo fmt --all`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test` for the narrowest relevant set of crates.

When changing TypeScript/Svelte code, prefer running:

- `pnpm lint`
- `pnpm check`
- `pnpm test`

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
- Keep secrets and credentials out of the repo; use `.env` files.

## Assistant Interaction Preferences

When working against an established plan, treat short replies like "Yes", "Yes, do it",
"Continue", "Carry on with that", or similar as explicit approval to proceed with
the next concrete item and actually implement it, not just restate the plan.

**"Continue" protocol:** Treat `Continue` as a codeword that accepts the assistant's
most recently proposed "next step".

This protocol applies across sessions for this repository.
```

### 5. Create .gitignore in Each Repo

Create `.gitignore` with content appropriate for the component:

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

### 6. Package Setup (TypeScript Components)

For TypeScript components (frontend-web, admin-web, api-client, ui-kit), create `package.json`:

```json
{
  "name": "@myorg/api-client",
  "version": "0.0.1",
  "private": true,
  "type": "module",
  "scripts": {
    "check": "tsc -p tsconfig.json --noEmit",
    "test": "vitest"
  },
  "dependencies": {
    "@decodelabs/underlay": "file:../underlay"
  },
  "devDependencies": {
    "@types/node": "^22.0.0",
    "typescript": "^5.0.0",
    "vitest": "^2.0.0"
  }
}
```

### 7. CI Configuration

Create `.github/workflows/ci.yml` in each repo:

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
      # Add component-specific steps...
```

---

## Directory Structure Templates

### api-backend (Rust)

```
api-backend/
├── AGENTS.md
├── Cargo.toml
├── .gitignore
└── crates/
    ├── api/src/          # HTTP handlers, router
    ├── core/src/         # Domain types, IDs
    ├── auth/src/         # Auth providers
    ├── db/
    │   ├── src/
    │   └── migrations/
    └── infra/src/        # Config, logging
```

### api-client (TypeScript)

```
api-client/
├── AGENTS.md
├── package.json
├── tsconfig.json
├── .gitignore
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

### frontend-web / admin-web (SvelteKit)

```
frontend-web/
├── AGENTS.md
├── package.json
├── svelte.config.js
├── vite.config.ts
├── tsconfig.json
├── .gitignore
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

**Note:** Frontends import `getClient()` directly from the shared api-client library. Do not create local `$lib/api/client.ts` wrappers that duplicate the shared library's functionality.

### documentation

```
documentation/
├── AGENTS.md
├── .gitignore
└── docs/
    ├── architecture/
    ├── domain/
    ├── processes/
    └── decisions/
```

---

## Next Steps

With the project structure in place, proceed to [030-underlay-integration](./030-underlay-integration.md) to link the Underlay foundation.
