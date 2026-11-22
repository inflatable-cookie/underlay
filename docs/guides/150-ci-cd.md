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
      - name: Read rust-toolchain.toml
        id: toolchain
        run: |
          python3 - <<'PY' >> "$GITHUB_OUTPUT"
          import tomllib
          with open("rust-toolchain.toml", "rb") as f:
              tc = tomllib.load(f)["toolchain"]
          print(f"channel={tc['channel']}")
          print(f"components={','.join(tc.get('components', []))}")
          PY
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ steps.toolchain.outputs.channel }}
          components: ${{ steps.toolchain.outputs.components }}
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

Reading `rust-toolchain.toml` and passing its `channel` and `components`
explicitly to `dtolnay/rust-toolchain@master` keeps CI aligned with the
repository's pinned toolchain. See Underlay's own workflows in
[`.github/workflows/rust.yml`](../../.github/workflows/rust.yml) and
[`.github/workflows/ci.yml`](../../.github/workflows/ci.yml) for complete
examples.

## Next Steps

- [160-troubleshooting.md](./160-troubleshooting.md)
