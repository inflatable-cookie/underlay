# Release

Underlay releases as an immutable Git tag, `vX.Y.Z`, with the Rust workspace
and the JavaScript package on the same synchronized version. There is no
registry publication. Consumers pin the tag on every Underlay declaration, as
[contract 023](023-release-and-compatibility-rollout.md) specifies.

Releasing is an operator action: an agent prepares it only when the operator
asks, and never pushes a tag on its own.

## Versioning

- Pre-1.0: a breaking change takes the minor version, and anything else takes
  the patch.
- The version lives in `Cargo.toml` under `[workspace.package]`, and is synced
  to `Cargo.lock` and `package.json`.
- Roadmap or plan numbering never decides a version.

## Steps

1. Land every change for the release on `main`, with QA green.
2. Add the `CHANGELOG.md` entry. A consumer-affecting release needs the
   contract 023 upgrade block: impact class, exact consumer actions, any
   deprecation window, validation commands, and a rollback tag.
3. Check readiness: `effigy release status --check-gates`. The gates are
   version sync, `effigy validate`, `effigy rust:check` and
   `effigy rust:clippy`.
4. Prepare: `effigy release prepare --plan` to review, then
   `effigy release prepare`. This bumps the version and syncs the lock and
   package files.
5. **Check `Cargo.lock` before executing.** Accept only workspace-crate version
   bumps. `v0.9.11` failed because the bump also moved a third-party crate
   (`rsa`) to a version not on crates.io, and `v0.9.12` had to be cut to fix it.
6. Execute: `effigy release execute`. This commits `release: vX.Y.Z` and
   creates the tag.
7. Push the commit and the tag.

## Verify

- The `rust.yml` workflow passes on the release commit.
- A consumer can resolve the new tag on both language surfaces.
- For a consumer-affecting release, one reference consumer upgrades from the
  tag and runs its own validation.

## Roll back

Tags are immutable. Consumers roll back by pinning the previous known-good tag.
Fix forward with a new patch release, and say in its changelog entry which tag
to skip.
