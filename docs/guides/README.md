# Underlay Guides

Comprehensive guides for building full-stack applications with Underlay. This documentation covers architecture, implementation, testing, and deployment.

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
7. **[060 - Authentication](./060-authentication.md)** - Auth providers, JWT, TOTP, WebAuthn, OAuth
8. **[065 - Session Management](./065-session-management.md)** - Session lifecycle, cookies, refresh
9. **[067 - Authorization](./067-authorization.md)** - Role-based access control
10. **[068 - Security](./068-security.md)** - Rate limiting, CSP, password policy, lockout
11. **[070 - API Handlers](./070-api-handlers.md)** - HTTP handlers, routing
12. **[072 - Admin/Front Separation](./072-admin-front-separation.md)** - Audience routing conventions
13. **[075 - Validation](./075-validation.md)** - Request validation patterns
14. **[076 - Nightfire](./076-nightfire.md)** - Block-based structured content
15. **[080 - TypeScript Client](./080-typescript-client.md)** - HTTP client, commands
16. **[090 - UI Kit](./090-ui-kit.md)** - Component patterns
17. **[095 - Navigation Context](./095-navigation-context.md)** - Contextual back buttons and form redirects
18. **[100 - Frontend (Web)](./100-frontend-web.md)** - SvelteKit setup, routing
19. **[110 - Admin Frontend](./110-admin.md)** - Admin UI structure
20. **[120 - Configuration](./120-configuration.md)** - Env files, validation
21. **[130 - Testing](./130-testing.md)** - Test patterns for all layers
22. **[140 - Local Development](./140-local-development.md)** - Running locally, debugging
23. **[150 - CI/CD](./150-ci-cd.md)** - GitHub Actions template
24. **[160 - Troubleshooting](./160-troubleshooting.md)** - Common issues and solutions
25. **[170 - Checklist](./170-checklist.md)** - Completion verification

## Code Examples

Code examples referenced in the guides are located in the `code/` subdirectory:

```
code/
├── 040-rust-backend/       # Rust workspace examples
├── 050-database/           # SQL migrations
├── 060-authentication/     # Auth provider implementations
├── 068-security/           # Security utilities, CSP, rate limiting
├── 070-api-handlers/       # Handler examples
├── 080-typescript-client/  # HTTP client, types
├── 090-ui-kit/             # Svelte components
├── 100-frontend-web/       # SvelteKit pages (web)
├── 110-admin/              # SvelteKit pages (admin)
├── 120-configuration/      # Environment examples
├── 130-testing/            # Test examples
└── 150-ci-cd/              # CI/CD workflows
```

## Quick Reference

### Essential Commands

**Multi-repo (default):** run commands from each repo root.

```bash
# API (backend)
cd myapp-api && cargo test
cd myapp-api/crates/db && sqlx migrate run
cd myapp-api && cargo run -p myapp-api

# Web (frontend)
cd myapp-web && pnpm install
cd myapp-web && pnpm dev

# Admin (frontend)
cd myapp-admin && pnpm install
cd myapp-admin && pnpm dev

# Client (TypeScript)
cd myapp-client && pnpm install
cd myapp-client && pnpm check

# UI kit (optional)
cd myapp-ui && pnpm install
cd myapp-ui && pnpm check
```

**Monorepo:** run workspace scripts from repo root.

```bash
pnpm install:all
pnpm check:all
pnpm test:all

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
└── trellis/docs/
```

## Getting Help

- Check [160-troubleshooting](./160-troubleshooting.md) for common issues
- Review Underlay documentation in the Underlay repo (`underlay/docs/`), or `libs/underlay/docs/` in monorepo mode
