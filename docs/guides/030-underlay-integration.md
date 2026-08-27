# 030 - Underlay Integration

This document explains how to integrate the Underlay foundation library into your new project. Underlay provides cross-cutting primitives that your app composes rather than reimplementing.

## What is Underlay?

Underlay is a reusable foundation that provides:

| Crate | Purpose |
|-------|---------|
| `underlay-core` | IDs, error types, response envelopes |
| `underlay-http` | Axum helpers, CORS, standard error responses |
| `underlay-auth` | Auth boundary, provider abstraction, Axum extractors |
| `underlay-auth-jwt` | JWT issuance/verification (Ed25519 / EdDSA via `jsonwebtoken`) |
| `underlay-db` | SQLx helpers (optional) |
| `underlay-observability` | Tracing bootstrap, request ID |
| `underlay-metrics` | Prometheus registry helpers (optional) |
| `underlay-events` | Outbox/event patterns (optional) |

## How Underlay Enters the Workspace

Underlay is a **released dependency**, not a directory inside your repository.
Both language surfaces resolve the same way: a **pinned release tag** on the
Underlay Git repository.

Do not symlink Underlay into the workspace, add it as a Git submodule, vendor
its source, or point a `file:` dependency at a sibling checkout.
[Contract 024](../contracts/024-new-app-bootstrap-and-bring-up.md) owns this
rule.

A sibling `../underlay` checkout is still useful for QA scripts and dev mounts —
`acowtancy` runs its conformance checks that way. That is a tooling convenience
and must never become the committed dependency shape.

## Rust Integration

Declare the crates once in the app-local Cargo workspace at
`apps/api/Cargo.toml`, pinned to a release tag:

```toml
# apps/api/Cargo.toml

[workspace]
members = [
  "crates/core",
  "crates/api",
  "crates/auth",
  "crates/db",
  "crates/infra",
]

[workspace.dependencies]
underlay-core = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.4" }
underlay-http = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.4", features = ["openapi"] }
underlay-auth = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.4" }
underlay-auth-jwt = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.4" }
underlay-observability = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.4" }
underlay-metrics = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.4" }
```

The tag makes the workspace build without a sibling checkout. Keep the Cargo
workspace inside `apps/api`; do not hoist it to the repository root and do not
add `path` dependencies that reach outside the repository.

For live Underlay co-development, run `effigy deps link cargo ../underlay`,
which writes a machine-local `[patch]` into an untracked `.cargo/config.toml`.
Never commit that file — the link is a local tooling affordance, not the
committed dependency shape.

## TypeScript Integration

Underlay's TS package name is `@inflatable-cookie/underlay`, but source imports
should target explicit subpaths such as:

- `@inflatable-cookie/underlay/client/*`
- `@inflatable-cookie/underlay/runtime/*`
- `@inflatable-cookie/underlay/patterns`
- `@inflatable-cookie/underlay/nightfire/*`
- `@inflatable-cookie/underlay/utils/*`

Depend on a released tag from each consuming package's manifest:

```json
{
  "dependencies": {
    "@inflatable-cookie/underlay": "git+ssh://git@github.com/inflatable-cookie/underlay.git#v0.9.4"
  }
}
```

Pin the tag so the root `bun.lock` stays reproducible. Resolve the dependency
with one frozen install from the repository root:

```bash
bun install --frozen-lockfile
```

Internal packages in the same workspace use `workspace:*` instead:

```json
{
  "dependencies": {
    "@myorg/client": "workspace:*",
    "@myorg/ui": "workspace:*"
  }
}
```

### Workspace-shape conformance

Keep workspace topology drift separate from security conformance. Add a
consumer-owned Effigy task that invokes the published bin entry, then compose
it into `health` or `validate`:

```toml
[tasks."qa:workspace-shape"]
run = "underlay-workspace-shape ."
```

When Underlay is installed as a released dependency, Bun resolves the bin from
the installed package:

```toml
[tasks."qa:workspace-shape"]
run = "bunx underlay-workspace-shape ."
```

A sibling `../underlay` checkout can use the same bin through Bun's local
install during co-development:

```toml
[tasks."qa:workspace-shape"]
run = "bun ../underlay/ts/bin/underlay-workspace-shape.ts ."
```

The checker enforces one Git root, root `private: true`, a pinned
`packageManager`, explicit workspace paths contained by the Git root, complete
`apps/*` / `packages/*` workspace membership, no declared workspaces outside
those prefixes, one root `bun.lock`, no child lockfiles, no internal `file:`
edges, no committed `file:` Underlay/Poodle dependencies, and `workspace:*` for
internal JavaScript dependencies. Security policy remains in
`underlay/scripts/check-consumer-conformance.sh` via a separate task such as
`qa:security`.

### Env-authority conformance

Keep env/secret inventory separate from workspace topology. Add a consumer-owned
Effigy task that invokes the published bin entry, then compose it into `health`
or `validate`:

```toml
[tasks."qa:env-authority"]
run = "underlay-env-authority ."
```

When Underlay is installed as a released dependency:

```toml
[tasks."qa:env-authority"]
run = "bunx underlay-env-authority ."
```

A sibling `../underlay` checkout can use:

```toml
[tasks."qa:env-authority"]
run = "bun ../underlay/ts/bin/underlay-env-authority.ts ."
```

The checker proves `config/env-manifest.txt` and `config/required-secrets.txt`
exist when the workspace has a runtime env reader, that both files parse as key
lists, and that required keys are declared in the manifest. It does not read
`.env` files or guess which product secrets are mandatory. Live value presence
stays with `underlay/scripts/check-env-manifest.sh` and is not a CI secret
requirement.

### Upgrading Underlay

1. bump the pinned tag in every manifest that declares it;
2. run `bun install` from the repository root;
3. bump the Cargo `tag` values in `apps/api/Cargo.toml`;
4. commit the single updated `bun.lock` and `Cargo.lock`;
5. check [190-upgrade-compatibility](./190-upgrade-compatibility.md) for
   breakage expectations.

## Verifying Integration

### 1) Verify Rust Integration

```rust
use underlay_core::Uuid;

#[derive(Debug)]
pub struct MyId(pub Uuid);

impl MyId {
    pub fn new() -> Self {
        Self(Uuid::new_v7())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_generation_works() {
        let id = MyId::new();
        // Underlay Uuid is a UUIDv7 wrapper.
        assert_ne!(id.0.to_string(), "");
    }
}
```

### 2) Verify TypeScript Integration

Underlay’s error envelope shape is:

```ts
import type { ErrorEnvelope } from "@inflatable-cookie/underlay/client/types";

const example: ErrorEnvelope = {
  error: {
    code: "test.ok",
    message: "Test successful",
  },
};
```

## Common Integration Issues

### Issue: "cannot find underlay-core"

Cause: the crate is missing from `[workspace.dependencies]` in
`apps/api/Cargo.toml`, or the member crate did not opt in with
`underlay-core.workspace = true`.

### Issue: a package resolves a different Underlay version than its siblings

Cause: manifests pin different Underlay tags, or a stale child lockfile is
shadowing the root `bun.lock`. Align the tags, delete any child lockfile, and
reinstall from the repository root.

### Issue: "Package path . is not exported from @inflatable-cookie/underlay"

Cause: source code is importing the retired root barrel `@inflatable-cookie/underlay`
instead of an explicit package subpath.

Fix: move the import onto the real package surface:

- `@inflatable-cookie/underlay/client/*`
- `@inflatable-cookie/underlay/runtime/*`
- `@inflatable-cookie/underlay/patterns`
- `@inflatable-cookie/underlay/nightfire/*`
- `@inflatable-cookie/underlay/utils/*`

## Next Steps

Proceed to [040-rust-backend](./040-rust-backend.md).
