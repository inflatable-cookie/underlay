# 170 - Completion Checklist

Use this checklist to verify your project is properly set up.

## Project Structure

- [ ] Root `AGENTS.md` created
- [ ] `apps/` directory with `bloom/`, `greenhouse/`, `nursery/`
- [ ] `libs/` directory with `petal/`, `stem/`
- [ ] `trellis/` or `docs/` directory for documentation

## Backend (Nursery)

- [ ] Rust workspace created with `Cargo.toml`
- [ ] `core` crate with ID types
- [ ] `auth` crate with providers
- [ ] `db` crate with pool and migrations
- [ ] `api` crate with handlers and router
- [ ] Database migrations created and run
- [ ] Auth provider works (dev mode for local)
- [ ] API endpoints return expected responses

## API Client (Stem)

- [ ] `package.json` with correct exports
- [ ] `tsconfig.json` with strict mode
- [ ] HTTP client with auth support
- [ ] TypeScript types for domain objects
- [ ] Command functions for API calls
- [ ] Auth token storage helpers
- [ ] Tests pass (`pnpm test`)

## UI Kit (Petal)

- [ ] `package.json` configured
- [ ] Shared components created
- [ ] Design tokens defined
- [ ] Tests pass

## Frontends (Bloom/Greenhouse)

- [ ] SvelteKit project initialized
- [ ] Routes created for main pages
- [ ] API client integrated
- [ ] Auth flow implemented
- [ ] Dev server runs (`pnpm dev`)

## Configuration

- [ ] `.env` files created for all apps
- [ ] Environment variables documented
- [ ] Configuration validation at startup

## Testing

- [ ] Rust tests pass (`cargo test`)
- [ ] TypeScript tests pass (`pnpm test`)
- [ ] Frontend tests pass

## Local Development

- [ ] Database runs and accepts connections
- [ ] Migrations applied successfully
- [ ] Backend runs (`cargo run`)
- [ ] Frontends run (`pnpm dev`)
- [ ] All services accessible

## Documentation

- [ ] README.md created with setup instructions
- [ ] Architecture documented
- [ ] API documented

## Git

- [ ] Git repository initialized
- [ ] `.gitignore` configured
- [ ] Initial commit made
- [ ] CI/CD workflow created

## Final Verification

```bash
# Run all checks
cd apps/nursery && cargo test
cd libs/stem && pnpm test
cd apps/bloom && pnpm test
```

If all checks pass, your project is ready for development!
