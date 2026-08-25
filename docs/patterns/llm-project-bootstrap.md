# LLM Pattern: Project Bootstrap

Quick reference for bootstrapping a new Underlay workspace. Use
[contract 024](../contracts/024-new-app-bootstrap-and-bring-up.md) as the
normative source and [guide 175](../guides/175-llm-bootstrap-guide.md) for the
full walkthrough.

## Prompt Pattern

When a user asks to create a new project, use this shape:

```
I'll help you create a new [project-name] application using the Underlay
workspace contract.

## Steps

1. Create one Git repository with apps/, packages/, docs/, and a root manifest
2. Pin released Underlay and Poodle dependencies
3. Configure root state, secrets, and the API package's migration:* tasks
4. Install once from the root and verify the workspace
5. Start the app through the root Effigy catalog
```

Do not offer a flat package tree, a separate repository per package, or a
committed source-checkout dependency as an alternative.

## Key Files to Reference

| Purpose | Reference |
|---------|-----------|
| Workspace topology and root manifest | `docs/contracts/024-new-app-bootstrap-and-bring-up.md` |
| Directory layout and root AGENTS.md | `docs/guides/020-project-structure.md` |
| Released Underlay integration | `docs/guides/030-underlay-integration.md` |
| Rust backend | `apps/api/Cargo.toml` and `docs/guides/040-rust-backend.md` |
| Database migrations | `apps/api/migrations/` and `docs/contracts/021-database-migration-and-schema-workflow.md` |
| TypeScript client | `packages/client/` and `docs/guides/080-typescript-client.md` |
| Admin and front apps | `apps/admin/` and `apps/front/` |

## Renaming Checklist

Replace the placeholder project name in:

1. root and package manifests
2. Rust crate names and modules
3. TypeScript package names, commands, and types
4. SQL schema names and migration inputs
5. cookie, token, and display names

Review each replacement. Do not rewrite migration history or generated files
with a blind global substitution.

## Verification Commands

Run from the workspace root:

```bash
effigy workspace:js:prepare
effigy tasks
effigy health
effigy test --plan
effigy validate
effigy state plan
effigy state apply local --yes
effigy dev
```

For the app-local Rust workspace:

```bash
cd apps/api
cargo check --workspace
```

## Common Issues

| Error | Solution |
|-------|----------|
| Missing Underlay crates | Check the released Git tag and `apps/api/Cargo.lock` |
| Schema errors | Apply the root state stack, then use the routed API-package `migration:*` task |
| Secret errors | Initialize the local Effigy secret store and check the required-secret manifest |
| Import errors | Use explicit Underlay exports such as `client/*` and `runtime/*` |

## Full Guide

See [175-llm-bootstrap-guide.md](../guides/175-llm-bootstrap-guide.md) for
complete step-by-step instructions.
