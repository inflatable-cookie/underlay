# Quickstart Guide Index

This guide provides step-by-step instructions for initializing a new project following the Songsprout/Acowtancy architecture.

## Reading Order

Read these documents in order for a complete understanding:

1. **[000 - Architecture Overview](./000-overview.md)** - High-level architecture, principles, when to use
2. **[010 - Prerequisites](./010-prerequisites.md)** - System requirements, verification
3. **[020 - Project Structure](./020-project-structure.md)** - Directory layout, AGENTS.md
4. **[030 - Underlay Integration](./030-underlay-integration.md)** - Linking the Underlay foundation
5. **[040 - Rust Backend (Nursery)](./040-rust-backend.md)** - Workspace, core crate, patterns
6. **[050 - Database & Migrations](./050-database.md)** - DB crate, sqlx, migrations
7. **[060 - Authentication](./060-authentication.md)** - Auth providers, JWT, dev mode
8. **[070 - API Handlers](./070-api-handlers.md)** - HTTP handlers, routing, middleware
9. **[080 - TypeScript Client (Stem)](./080-typescript-client.md)** - HTTP client, commands
10. **[090 - UI Kit (Petal)](./090-ui-kit.md)** - Component patterns
11. **[100 - Frontend (Bloom)](./100-frontend-bloom.md)** - SvelteKit setup, routing
12. **[110 - Admin Frontend (Greenhouse)](./110-admin-greenhouse.md)** - Admin UI structure
13. **[120 - Configuration](./120-configuration.md)** - Env files, validation
14. **[130 - Testing](./130-testing.md)** - Test patterns for all layers
15. **[140 - Local Development](./140-local-development.md)** - Running locally, debugging
16. **[150 - CI/CD](./150-ci-cd.md)** - GitHub Actions template
17. **[160 - Troubleshooting](./160-troubleshooting.md)** - Common issues and solutions
18. **[170 - Checklist](./170-checklist.md)** - Completion verification

## Code Examples

Code examples referenced in the guides are located in the `code/` subdirectory:

```
code/
├── 040-rust-backend/       # Rust workspace examples
├── 050-database/           # SQL migrations
├── 060-authentication/     # Auth provider implementations
├── 070-api-handlers/       # Handler examples
├── 080-typescript-client/  # HTTP client, types
├── 090-ui-kit/             # Svelte components
├── 100-frontend-bloom/     # SvelteKit pages
├── 110-admin-greenhouse/   # Admin pages
├── 120-configuration/      # Environment examples
├── 130-testing/            # Test examples
└── 150-ci-cd/              # CI/CD workflows
```

## Quick Reference

### Essential Commands

```bash
# Install Rust deps
cd apps/nursery && cargo test

# Install Node deps
pnpm install:all

# Run migrations
cd apps/nursery/crates/db && sqlx migrate run

# Start backend
cargo run -p myapp-api

# Start frontends
cd apps/bloom && pnpm dev
cd apps/greenhouse && pnpm dev

# Type checking
pnpm check:all
```

### Directory Structure

```
my-project/
├── apps/
│   ├── bloom/          # Artist UI
│   ├── greenhouse/     # Admin UI
│   └── nursery/        # Rust API
├── libs/
│   ├── petal/          # UI kit
│   ├── stem/           # API client
│   └── underlay/       # Foundation
└── trellis/docs/       # Documentation
```

## Reference Projects

- **Songsprout** - Artist productivity tool
- **Acowtancy** - Accounting tool

Both demonstrate the patterns in this guide.

## Getting Help

- Check [160-troubleshooting](./160-troubleshooting.md) for common issues
- Reference Songsprout/Acowtancy source code
- Review Underlay documentation in `libs/underlay/docs/`
