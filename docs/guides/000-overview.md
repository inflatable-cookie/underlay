# 000 - Architecture Overview

This guide provides step-by-step instructions for initializing a new project
that follows the Underlay workspace architecture: one Git repository containing
`apps/*` and `packages/*`. This architecture is designed for full-stack
applications requiring:

- **Rust API backend** with domain-driven design
- **TypeScript API client** with typed commands
- **SvelteKit frontends** (artist-facing + admin)
- **Shared UI kit** built on Poodle primitives and generic composites
- **Underlay integration** for workflow shells, client/runtime helpers, and
  specialized shared systems

## When to Use This Guide

Use this guide when creating a **new product** that:

1. Requires a **robust, type-safe API** with Rust
2. Needs **multiple frontends** (user-facing + admin)
3. Benefits from **shared design system** and client libraries
4. Wants to leverage **Underlay's cross-cutting primitives**
5. Requires **production-grade auth, observability, and database patterns**

## Architecture Diagram

One Git repository owns the whole product workspace.
[Contract 024](../contracts/024-new-app-bootstrap-and-bring-up.md) is the
normative source for this topology.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    Project Workspace (one git repository)                │
├─────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────┐  ┌─────────────────────┐  ┌──────────────────┐ │
│  │   apps/front/       │  │  apps/admin/        │  │  apps/api/       │ │
│  │   (Web UI)          │  │   (Admin UI)        │  │   (Rust API)     │ │
│  │   SvelteKit         │  │   SvelteKit         │  │   Axum + Domain  │ │
│  └──────────┬──────────┘  └──────────┬──────────┘  └────────┬─────────┘ │
│             │                        │                       │          │
│             └────────────────────────┼───────────────────────┘          │
│                                      │                                  │
│  ┌───────────────────────────────────┼──────────────────────────────┐  │
│  │  packages/ui/                     │  packages/client/             │  │
│  │  Shared UI kit                    │  Shared TypeScript API client │  │
│  │  Components, patterns, tokens     │  HTTP, commands, types        │  │
│  │  (internal `workspace:*` edges)   │  (internal `workspace:*` edges)│ │
│  └───────────────────────────────────┴──────────────────────────────┘  │
│                                      │                                  │
│  ┌───────────────────────────────────┴──────────────────────────────┐  │
│  │  Underlay + Poodle (released dependencies, not in-tree)           │  │
│  │  Cross-cutting primitives: IDs, errors, envelopes, auth, observability │
│  └───────────────────────────────────────────────────────────────────┘  │
│                                      │                                  │
│  ┌───────────────────────────────────┴──────────────────────────────┐  │
│  │  External Services                                                   │  │
│  │  PostgreSQL, Redis (optional), Prometheus metrics, tracing          │  │
│  └───────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────┘
```

## Key Principles

### 1. Apps Own Domain Logic

Underlay provides **primitives**; your app defines its **nouns and rules**.

- Underlay: `Uuid`, `AppError`, `AuthProvider`, `CorsConfig`
- Your App: `Artist`, `Program`, `Task`, `Release`

### 2. Stable Boundaries

Shared code uses **small, well-typed interfaces** that apps compose.

```rust
// Underlay provides the interface
trait AuthProvider {
    async fn authenticate_bearer(&self, token: &str) -> AuthResult<Principal>;
}

// Your app implements it
impl AuthProvider for MyAppAuthProvider {
    async fn authenticate_bearer(&self, token: &str) -> AuthResult<Principal> {
        // Your implementation
    }
}
```

### 3. UI Boundary Is Explicit

Use the package that actually owns the contract:

- Poodle for primitives and generic composites
- Underlay `patterns` for retained workflow and page-shell UI
- Underlay `runtime` for shared browser/app orchestration and controllers
- Underlay `utils` for small standalone helpers
- Underlay `client` for transport and SvelteKit-facing integration helpers
- Underlay `nightfire` for structured content editing/runtime

### 4. No Forced Stack Choices

Underlay provides **defaults** but they are **optional**.

| Layer | Default | Optional Alternatives |
|-------|---------|----------------------|
| API Framework | Axum | Actix, Rocket |
| Database | SQLx | Diesel, SeaORM |
| ORM | sqlx::query_as | SeaORM entities |
| HTTP Client | reqwest | ureq, attohttpc |
| Frontend | SvelteKit | Next.js, Remix |
| UI Framework | Svelte | React, Vue |
| Styling | CSS + Underlay tokens | Tailwind, Bootstrap |

### 5. Consistent Conventions

Following naming and structure patterns from reference apps ensures:

- **Predictable navigation** for new developers
- **Easier cross-project contributions**
- **Consistent tooling** (linters, formatters, tests)

## Layer Responsibilities

### Apps Layer (`apps/`)

| Directory | Responsibility |
|-----------|---------------|
| `apps/api/` | Rust API implementation, domain models, HTTP handlers |
| `apps/front/` | SvelteKit frontend (main UI) |
| `apps/admin/` | Admin/author SvelteKit frontend |

Only JavaScript apps that own a manifest are root workspace members. A
Rust-only app directory is not.

### Packages Layer (`packages/`)

| Directory | Responsibility |
|-----------|---------------|
| `packages/client/` | TypeScript API client, HTTP abstraction, typed commands |
| `packages/ui/` | Shared UI components, design tokens, UI patterns |

Apps depend on these with `workspace:*`. Underlay and Poodle are **not** in-tree
packages — they arrive as released dependencies.

### Documentation Layer (`docs/`)

| Directory | Responsibility |
|-----------|---------------|
| `docs/` | Vision, architecture, domain docs, roadmaps, and logs — one root authority per workspace |

## How to Use This Guide

### For LLMs

1. Read documents **in order** (000 → 170)
2. Copy code examples from the `code/` subdirectories
3. Follow the checklist in `170-checklist.md`
4. Treat the docs as scaffolding; fill in app-specific domain details as you build

### For Human Developers

1. Skim 000-overview for architecture context
2. Follow 010-prerequisites to set up your machine
3. Use 020-project-structure for initial setup
4. Reference specific sections as needed (e.g., 060-authentication for auth setup)
5. Use 170-checklist to verify completeness

## Document Map

| Document | Title | Purpose |
|----------|-------|---------|
| 000 | Architecture Overview | High-level architecture, principles |
| 010 | Prerequisites | System requirements, verification |
| 020 | Project Structure | Directory layout, AGENTS.md |
| 030 | Underlay Integration | Linking the Underlay foundation |
| 040 | Rust Backend | Workspace, core crate, patterns |
| 050 | Database & Migrations | DB crate, sqlx, migrations |
| 060 | Authentication | Auth providers, JWT, TOTP, WebAuthn, OAuth |
| 065 | Session Management | Login/logout flows, cookie management, refresh |
| 067 | Authorization | RBAC, role extraction, protected routes |
| 070 | API Handlers | HTTP handlers, routing, middleware |
| 075 | Validation | Backend/frontend validation, error display |
| 077 | Media Library | File uploads, blob storage, media management |
| 080 | TypeScript Client | HTTP client, commands |
| 090 | UI Kit | Component patterns, design tokens |
| 100 | Frontend (Web) | SvelteKit setup, routing |
| 110 | Admin Frontend | Admin UI structure |
| 120 | Configuration | Env files, validation |
| 130 | Testing | Test patterns for all layers |
| 140 | Local Development | Running locally, debugging, guardrails |
| 150 | CI/CD | GitHub Actions template |
| 160 | Troubleshooting | Common issues and solutions |
| 170 | Checklist | Completion verification |

## Reference Implementation

`acowtancy` is the live proof of this architecture: one Git root, `apps/*`,
`packages/*`, root `docs/`, one root Bun manifest and lockfile, internal
`workspace:*` edges, and released Underlay/Poodle dependencies.

`underlay-reference` remains the bootstrap fixture and carries the Acme example
stack (Rust backend, TypeScript client, admin and public SvelteKit frontends,
shared UI package). Its physical layout has not converged on this contract yet;
`g10.005` normalizes it. Read it for component-level patterns, not for workspace
topology.

See [175-llm-bootstrap-guide.md](./175-llm-bootstrap-guide.md) for step-by-step instructions on bootstrapping a new project from the reference.

## Next Steps

Proceed to [010-prerequisites](./010-prerequisites.md) to verify your development environment.
