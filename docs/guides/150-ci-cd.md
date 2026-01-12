# 150 - CI/CD

This document covers setting up continuous integration and deployment.

## Modes

- **Multi-repo (default):** each repo typically has its own workflow; keep CI close to the repo it validates.
- **Monorepo:** one workflow can run all checks across `apps/*` and `libs/*`.

The YAML below is written in a monorepo style; in multi-repo, remove the `cd apps/api` step and run `cargo test` from the API repo root.

## GitHub Actions Workflow

Create `.github/workflows/ci.yml`:

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  rust:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: |
          # Monorepo:
          cd apps/api
          cargo test
          cargo clippy --all-targets --all-features -- -D warnings

          # Multi-repo:
          # cargo test
          # cargo clippy --all-targets --all-features -- -D warnings

  typescript:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: pnpm/action-setup@v4
        with:
          version: 9
      - uses: actions/setup-node@v4
        with:
          node-version: 20
      - name: Install and test
        run: |
          # Monorepo:
          pnpm install:all
          pnpm check:all

          # Multi-repo:
          # pnpm install
          # pnpm check
```

See full template in `/code/150-ci-cd/`

## Next Steps

- [160-troubleshooting.md](./160-troubleshooting.md)
