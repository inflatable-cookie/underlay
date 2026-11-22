# 150 - CI/CD

This document covers setting up continuous integration and deployment.

## Workspace CI

The supported shape is one Git repository with one CI workflow. The workflow
validates the root Effigy surface, the app-local Rust workspace, and the root
Bun workspace. There is no alternate multi-repository setup to maintain.

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
      - uses: dtolnay/rust-toolchain@master
      - name: Run tests
        run: |
          cd apps/api
          cargo test
          cargo clippy --all-targets --all-features -- -D warnings

  typescript:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: oven-sh/setup-bun@v2
      - uses: inflatable-cookie/setup-effigy@987fd556617ea2c3e0ab5cef6b47b250817f50c8 # v1.0.0
        with:
          version: "0.11.0"
      - name: Install workspace dependencies
        run: |
          effigy workspace:js:prepare
          effigy validate
```

`dtolnay/rust-toolchain@master` reads the repository's `rust-toolchain.toml`
automatically (including the `rustfmt` and `clippy` components). See Underlay's
own workflows in [`.github/workflows/rust.yml`](../../.github/workflows/rust.yml)
and [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml) for complete
examples.

## Next Steps

- [160-troubleshooting.md](./160-troubleshooting.md)
