# LLM Pattern: Project Bootstrap

Quick reference for LLMs bootstrapping new Underlay projects.

## Prompt Pattern

When a user asks to create a new project, use this pattern:

```
I'll help you create a new [project-name] application using the Underlay reference implementation.

## Steps:

1. Copy from the `underlay-reference` repository (`acme-*` projects)
2. Rename `acme` → `[project-name]` throughout
3. Set up database and run migrations
4. Generate JWT keys
5. Install dependencies and verify builds

Let me start by copying the files...
```

## Key Files to Reference

From the `underlay-reference` repository:

| Purpose | Reference File |
|---------|----------------|
| Rust workspace | `acme-api/Cargo.toml` |
| API entry point | `acme-api/crates/api/src/main.rs` |
| Auth service | `acme-api/crates/auth/src/local.rs` |
| Database schema | `acme-api/migrations/` |
| TypeScript client | `acme-client/src/index.ts` |
| Admin layout | `acme-admin/src/routes/(app)/+layout.svelte` |
| Auth hooks | `acme-admin/src/hooks.server.ts` |

## Renaming Checklist

Essential substitutions (replace `acme` with project name):

1. **Cargo.toml files** - crate names
2. **package.json files** - package names
3. **Rust source** - module names, struct names
4. **TypeScript source** - function names, type names
5. **SQL migrations** - schema names
6. **Cookie names** - token identifiers
7. **Display names** - UI text, email templates

## Verification Commands

```bash
# Rust builds
cd api && cargo build

# TypeScript type checks
cd api-client && bun run build
cd admin && bun check
cd front && bun check

# API responds
curl http://localhost:3000/api/health
```

## Common Issues

| Error | Solution |
|-------|----------|
| Missing underlay crates | Check `underlay` symlink |
| Database errors | Run migrations |
| JWT errors | Regenerate keys |
| Import errors | Rebuild api-client |

## Full Guide

See [175-llm-bootstrap-guide.md](../guides/175-llm-bootstrap-guide.md) for complete step-by-step instructions.
