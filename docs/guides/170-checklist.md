# 170 - Completion Checklist

Use this checklist to verify your project is properly set up.

## Project Structure

- [ ] Root `AGENTS.md` created
- [ ] `apps/` directory with `web/`, `admin/`, `api/`
- [ ] `libs/` directory with `ui/`, `client/`
- [ ] `trellis/` or `docs/` directory for documentation

## Backend (API)

- [ ] Rust workspace created with `Cargo.toml`
- [ ] `core` crate with ID types
- [ ] `auth` crate with providers
- [ ] `db` crate with pool and migrations
- [ ] `api` crate with handlers and router
- [ ] Database migrations created and run
- [ ] Auth provider works (dev mode for local)
- [ ] API endpoints return expected responses

## API Client

- [ ] `package.json` with correct exports
- [ ] `tsconfig.json` with strict mode
- [ ] HTTP client with auth support
- [ ] TypeScript types for domain objects
- [ ] Command functions for API calls
- [ ] Auth token storage helpers
- [ ] Tests pass (`bun test`)

## UI Kit

- [ ] `package.json` configured
- [ ] Shared components created
- [ ] Design tokens defined
- [ ] Tests pass

## Frontends (Web/Admin)

- [ ] SvelteKit project initialized
- [ ] Routes created for main pages
- [ ] API client integrated
- [ ] Auth flow implemented
- [ ] Dev server runs (`bun dev`)

## Configuration

- [ ] `.env` files created for all apps
- [ ] Environment variables documented
- [ ] Configuration validation at startup

## Testing

- [ ] Rust tests pass (`cargo test`)
- [ ] TypeScript tests pass (`bun test`)
- [ ] Frontend tests pass

## Local Development

- [ ] Database runs and accepts connections
- [ ] Migrations applied successfully
- [ ] Backend runs (`cargo run`)
- [ ] Frontends run (`bun dev`)
- [ ] All services accessible

## Documentation

- [ ] README.md created with setup instructions
- [ ] Architecture documented
- [ ] API documented

## Code Organization

- [ ] No files exceed hard limits from `020-project-structure.md`
- [ ] Large features split by domain and workflow (not appended into existing god files)
- [ ] Route/page files remain orchestration-focused
- [ ] Business logic is extracted into db/service/helper modules
- [ ] Tests are split/co-located to match feature structure

## Contract and Quality Gates

- [ ] Cross-repo contract checklist completed for backend/client/frontend changes
- [ ] Naming conventions by layer are consistent (`200-project-sync.md`)
- [ ] State management follows local vs URL vs store rules (`200-project-sync.md`)
- [ ] Error handling follows taxonomy and UI behavior map (`185-recipe-map-and-testing-matrix.md`)
- [ ] Admin list performance guardrails are applied where relevant
- [ ] Risk-based testing depth matches change type (`185-recipe-map-and-testing-matrix.md`)
- [ ] Pattern deviations are documented with ADR-lite notes

## Git

- [ ] Git repository initialized
- [ ] `.gitignore` configured
- [ ] Initial commit made
- [ ] CI/CD workflow created

## Final Verification

```bash
# Run all checks (multi-repo)
cd myapp-api && cargo test
cd myapp-client && bun test
cd myapp-web && bun test

# Run all checks (monorepo)
cd apps/api && cargo test
cd libs/client && bun test
cd apps/web && bun test
```

If all checks pass, your project is ready for development!
