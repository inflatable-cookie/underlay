# Underlay Guides

This is Underlay's **narrative how-to layer** — the active front door for
building against the foundation (backend, database, auth, API handlers,
TypeScript client, media, admin components). Contracts (`docs/contracts/`) own
the normative guarantees; these guides explain and demonstrate them. The admin
**template usage reference** lives in [`docs/usage/`](../usage/000-overview.md).

## Scope

Poodle is the canonical home for shared UI primitives and generic composites.
Underlay’s guides describe the retained package surfaces that still belong
here:

- `@inflatable-cookie/underlay/patterns` for retained workflow/page-shell UI
- `@inflatable-cookie/underlay/runtime/*` for shared app/runtime helpers and controllers
- `@inflatable-cookie/underlay/utils/*` for small standalone helpers
- `@inflatable-cookie/underlay/client/*` for transport and SvelteKit-facing client helpers
- `@inflatable-cookie/underlay/nightfire/*` for structured content editor/runtime

UI guide translation status:

- Poodle now owns the canonical implementation guides for shared UI
- Underlay UI-shaped guides should shrink to retained boundary notes,
  workflow/runtime exceptions, and full-stack integration guidance
- new generic UI implementation recipes should be added in Poodle, organised on
  a per-implementation basis

## Workspace Shape

Underlay consumers use one workspace shape: a **single Git repository** with
`apps/*`, `packages/*`, and a root `docs/`. Polyrepo layouts are unsupported.

[Contract 024](../contracts/024-new-app-bootstrap-and-bring-up.md) owns the
normative topology, the root `package.json` shape, and the dependency rules.
These guides explain and demonstrate it; they do not restate the guarantees.

Path convention used throughout the guides:

- `apps/api/...` — Rust backend
- `apps/front/...` — product-facing SvelteKit app
- `apps/admin/...` — admin SvelteKit app
- `packages/client/...` — shared TypeScript API client
- `packages/ui/...` — shared UI package

Directory names may be product-specific. Roles may not become implicit.

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
37. **[175 - LLM Bootstrap Guide](./175-llm-bootstrap-guide.md)** - Guided single-workspace bootstrap for LLMs
38. **[176 - AI Runtime Routing](./176-ai-runtime-routing.md)** - Provider-agnostic backend LLM runtime boundary
39. **[180 - Admin Workflow Playbook](./180-admin-workflow-playbook.md)** - Start-here implementation flow for admin features
40. **[181 - Temporary API Profile Migration Playbook](./181-temporary-api-profile-migration-playbook.md)** - Cross-app migration checklist (temporary)
41. **[185 - Recipe Map and Testing Matrix](./185-recipe-map-and-testing-matrix.md)** - Recipe-to-code references + minimum tests
42. **[190 - Upgrade Compatibility Matrix](./190-upgrade-compatibility.md)** - Upgrade expectations and breakage checks
43. **[200 - Project Sync](./200-project-sync.md)** - Migration/sync checklist for existing projects
44. **[205 - Legacy Migration Framework (End-to-End)](./205-legacy-migration-framework.md)** - Historical detailed migration-core reference; use the state-layout/Effigy migration policy for active operator posture

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

**Raw fallback:** run direct tool commands from the workspace root only when the repo has not represented that path in Effigy yet.

```bash
# One frozen install for the whole workspace
bun install --frozen-lockfile

# Rust backend
cd apps/api && cargo test
cd apps/api/crates/db && sqlx migrate run

# SvelteKit apps and shared packages
cd apps/front && bun dev
cd apps/admin && bun dev
cd packages/client && bun check
cd packages/ui && bun check
```

Never run per-package installs and never create a child lockfile.

### Directory Structure

```
my-project/
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

`acowtancy` is the live proof of this shape. `underlay-reference` is the
bootstrap fixture and converges on it in `g09.025`.

## Research

When making architecture or implementation decisions that depend on external comparison or source-backed learning:

- Check the [Research section](../research/) for evidence-based recommendations
- Use `master-index.md` to navigate from questions to relevant artifacts
- Follow the `research-to-implementation-playbook.md` to carry research into delivery

## Getting Help

- Check [160-troubleshooting](./160-troubleshooting.md) for common issues
- Review Underlay documentation in the Underlay repo (`underlay/docs/`)
