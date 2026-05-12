# Underlay Guides (Deprecated)

**This directory is deprecated.** Content has been migrated to `docs/usage/`.

See:
- [docs/usage/000-overview.md](../usage/000-overview.md) — New user-facing docs front door
- [docs/usage/backend/](../usage/backend/) — Backend guides (Rust, DB, API)
- [docs/usage/frontend/](../usage/frontend/) — Frontend guides (SvelteKit)
- [docs/usage/runtime/](../usage/runtime/) — Runtime and client helpers
- [docs/usage/templates/](../usage/templates/) — Admin template system

This directory will be removed in a future update. Do not add new guides here.

---

Legacy content follows.

Poodle is the canonical home for shared UI primitives and generic composites.
Underlay’s guides now describe the retained package surfaces that still belong
here:

- `@decodelabs/underlay/patterns` for retained workflow/page-shell UI
- `@decodelabs/underlay/runtime/*` for shared app/runtime helpers and controllers
- `@decodelabs/underlay/utils/*` for small standalone helpers
- `@decodelabs/underlay/client/*` for transport and SvelteKit-facing client helpers
- `@decodelabs/underlay/nightfire/*` for structured content editor/runtime

UI guide translation status:

- Poodle now owns the canonical implementation guides for shared UI
- Underlay UI-shaped guides should shrink to retained boundary notes,
  workflow/runtime exceptions, and full-stack integration guidance
- new generic UI implementation recipes should be added in Poodle, organised on
  a per-implementation basis

## Modes (Multi-repo vs Monorepo)

This guide supports two layouts:

- **Multi-repo workspace (default):** multiple git repositories checked out side-by-side in a local folder.
- **Monorepo:** a single git repository containing `apps/*` and `libs/*`.

### Path Mapping Convention

To keep examples readable, many docs use **monorepo-style logical paths** like `apps/api/...` and `libs/client/...`.

- In **multi-repo mode**, interpret these as paths *within the corresponding repo*:
  - `apps/api/...` → `<api-repo>/...`
  - `apps/web/...` → `<web-repo>/...`
  - `apps/admin/...` → `<admin-repo>/...`
  - `libs/client/...` → `<client-repo>/...`
  - `libs/ui/...` → `<ui-repo>/...`
  - `libs/underlay/...` → `<underlay-repo>/...`

## Reading Order

Read these documents in order for a complete understanding:

1. **[000 - Architecture Overview](./000-overview.md)** - High-level architecture, principles, when to use
2. **[010 - Prerequisites](./010-prerequisites.md)** - System requirements, verification
3. **[020 - Project Structure](./020-project-structure.md)** - Directory layout, AGENTS.md
4. **[030 - Underlay Integration](./030-underlay-integration.md)** - Linking the Underlay foundation
5. **[040 - Rust Backend](./040-rust-backend.md)** - Workspace, core crate, patterns
6. **[050 - Database & Migrations](./050-database.md)** - DB crate, sqlx, migrations
7. **[055 - Background Jobs](./055-background-jobs.md)** - Job queues, handlers, scheduling
8. **[060 - Authentication](./060-authentication.md)** - Auth providers, JWT, TOTP, WebAuthn, OAuth
9. **[065 - Session Management](./065-session-management.md)** - Session lifecycle, cookies, refresh
10. **[066 - SPA Deployment & Static Auth](./066-spa-deployment-and-static-auth.md)** - Static deployment, hybrid tokens, auth-aware pages
11. **[067 - Authorization](./067-authorization.md)** - Role-based access control
12. **[068 - Security](./068-security.md)** - Rate limiting, CSP, password policy, lockout
13. **[070 - API Handlers](./070-api-handlers.md)** - HTTP handlers, routing
14. **[071 - JSON Naming Policy](./071-json-naming.md)** - Canonical `snake_case` JSON conventions
15. **[072 - Admin/Front Separation](./072-admin-front-separation.md)** - Audience routing conventions
16. **[073 - API Profiles and Unified Query Contract](./073-api-profiles-and-query-contract.md)** - Canonical resource routes with profile-driven projections

17. **[074 - HTTP Caching and Freshness Contract](./074-http-caching-and-freshness.md)** - Validator-first caching, concurrency preconditions, and bounded microcache policy
18. **[075 - Validation](./075-validation.md)** - Request validation patterns
19. **[076 - Nightfire](./076-nightfire.md)** - Block-based structured content
20. **[077 - Media Library](./077-media-library.md)** - File uploads, blob storage, media management
21. **[080 - TypeScript Client](./080-typescript-client.md)** - HTTP client, commands
22. **[081 - Auth Security Alerting](./081-auth-security-alerting.md)** - Failed-login/lockout alerting with shared thresholds and dedupe
23. **[090 - UI Kit](./090-ui-kit.md)** - Component patterns
24. **[092 - Selection Suggestions](./092-selection-suggestions.md)** - Intelligent suggestions with selection history
25. **[095 - Navigation Context](./095-navigation-context.md)** - Contextual back buttons and form redirects
26. **[097 - Autonomous List Components](./097-autonomous-list-components.md)** - List state architecture, composition guidance, and batch patterns
27. **[098 - Shared Admin Patterns](./098-shared-admin-patterns.md)** - Higher-level admin composition guidance (EmptyState, detail headers, keyboard patterns, etc.)
28. **[100 - Frontend (Web)](./100-frontend-web.md)** - SvelteKit setup, routing
29. **[110 - Admin Frontend](./110-admin.md)** - Admin UI structure
30. **[120 - Configuration](./120-configuration.md)** - Typed config model, env boundaries, migration checklist
31. **[130 - Testing](./130-testing.md)** - Test patterns for all layers
32. **[140 - Local Development](./140-local-development.md)** - Running locally, debugging
33. **[150 - CI/CD](./150-ci-cd.md)** - GitHub Actions template
34. **[160 - Troubleshooting](./160-troubleshooting.md)** - Common issues and solutions
35. **[170 - Checklist](./170-checklist.md)** - Completion verification
36. **[172 - Lean AGENTS.md Files](./172-agents-files.md)** - Keep agent instructions concise and operational
37. **[176 - AI Runtime Routing](./176-ai-runtime-routing.md)** - Provider-agnostic backend LLM runtime boundary
38. **[180 - Admin Workflow Playbook](./180-admin-workflow-playbook.md)** - Start-here implementation flow for admin features
39. **[181 - Temporary API Profile Migration Playbook](./181-temporary-api-profile-migration-playbook.md)** - Cross-app migration checklist (temporary)
40. **[185 - Recipe Map and Testing Matrix](./185-recipe-map-and-testing-matrix.md)** - Recipe-to-code references + minimum tests
41. **[190 - Upgrade Compatibility Matrix](./190-upgrade-compatibility.md)** - Upgrade expectations and breakage checks
42. **[200 - Project Sync](./200-project-sync.md)** - Migration/sync checklist for existing projects
43. **[205 - Legacy Migration Framework (End-to-End)](./205-legacy-migration-framework.md)** - Historical detailed migration-core reference; use the state-layout/Effigy migration policy for active operator posture

## Effigy-First Repo Loop

When a repo in your workspace publishes `effigy.toml`, prefer its Effigy surface before raw tool commands:

```bash
effigy tasks
effigy health
effigy test --plan
```

Then use repo-owned tasks such as `effigy validate` or `effigy dev`.
If you are not already in the repo root, `cd` there first instead of adding a
redundant `--repo` flag for the current tree.
Use raw `cargo`, `bun`, or framework CLIs directly only when the repo has not represented that path in Effigy yet.

## Shared UI Catalog

Underlay now ships a local Storybook catalog for the retained shared UI surface.
Use it when you need to understand shared workflow shells and helpers quickly:

```bash
effigy storybook
effigy storybook:build
```

Use Poodle's own preview/docs for primitives and generic composites that were
migrated out of Underlay during the Poodle adoption wave.

## Code Examples

Code examples referenced in the guides are located in the `code/` subdirectory:

```
code/
├── 040-rust-backend/       # Rust workspace examples
├── 050-database/           # SQL migrations
├── 055-background-jobs/    # Job handler examples
├── 060-authentication/     # Auth provider implementations
├── 068-security/           # Security utilities, CSP, rate limiting
├── 070-api-handlers/       # Handler examples
├── 077-media-library/      # Media upload, blob storage examples
├── 080-typescript-client/  # HTTP client, types
├── 090-ui-kit/             # Deprecated: generic UI recipes now live in Poodle
├── 100-frontend-web/       # Integration-oriented frontend-web snippets
├── 110-admin/              # Integration-oriented admin snippets
├── 120-configuration/      # Environment examples
├── 130-testing/            # Test examples
├── 150-ci-cd/              # CI/CD workflows
└── 205-legacy-migration-framework/ # End-to-end migration framework artifacts
```

## Quick Reference

UI example rule:

- use Poodle guides plus ACME reference apps for generic visible UI
- use Underlay `code/` examples only for retained integration or runtime wiring

### Essential Commands

**Effigy-first:** when a repo publishes `effigy.toml`, prefer its task surface first.

```bash
effigy tasks
effigy health
effigy test --plan
effigy validate
```

**Multi-repo raw fallback:** run direct tool commands only when the repo has not represented that path in Effigy yet.

```bash
# API (backend)
cd myapp-api && cargo test
cd myapp-api/crates/db && sqlx migrate run
cd myapp-api && cargo run -p myapp-api

# Web (frontend)
cd myapp-web && bun install
cd myapp-web && bun dev

# Admin (frontend)
cd myapp-admin && bun install
cd myapp-admin && bun dev

# Client (TypeScript)
cd myapp-client && bun install
cd myapp-client && bun check

# UI kit (optional)
cd myapp-ui && bun install
cd myapp-ui && bun check
```

**Monorepo raw fallback:** run workspace scripts from repo root when the repo does not expose an Effigy surface.

```bash
bun install:all
bun check:all
bun test:all

cd apps/api && cargo test
cd apps/api/crates/db && sqlx migrate run
cd apps/api && cargo run -p myapp-api
```


### Directory Structure

**Multi-repo workspace (default):**

```
myapp-workspace/
├── underlay/            # Foundation (git repo)
├── myapp-api/           # Rust API (git repo)
├── myapp-client/        # TypeScript client (git repo)
├── myapp-ui/            # UI kit (git repo, optional)
├── myapp-web/           # Frontend (git repo)
└── myapp-admin/         # Admin frontend (git repo)
```

**Monorepo:**

```
my-project/
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

## Research

When making architecture or implementation decisions that depend on external comparison or source-backed learning:

- Check the [Research section](../research/) for evidence-based recommendations
- Use `master-index.md` to navigate from questions to relevant artifacts
- Follow the `research-to-implementation-playbook.md` to carry research into delivery

## Getting Help

- Check [160-troubleshooting](./160-troubleshooting.md) for common issues
- Review Underlay documentation in the Underlay repo (`underlay/docs/`), or `libs/underlay/docs/` in monorepo mode
