# Release

Underlay releases as an immutable Git tag, `vX.Y.Z`, with the Rust workspace
and the JavaScript package on the same synchronized version. There is no
registry publication; the GitHub Release is the release-notes surface.
Consumers pin the tag on every Underlay declaration, as
[contract 023](023-release-and-compatibility-rollout.md) specifies.

Release preparation and publication each require operator authorization.

## Versioning

- Pre-1.0: a breaking change takes the minor version, and anything else takes
  the patch.
- The version lives in `Cargo.toml` under `[workspace.package]`. Three files
  must carry the same version: `Cargo.toml`, `package.json`, and `info.version`
  in `contracts/openapi/underlay.openapi.yaml`.
  `ts/scripts/check-release-version-sync.ts` enforces that match, and it is the
  `version-sync` gate.
- Roadmap or plan numbering never decides a version.

## Prepare

Use Effigy 0.15 or newer. Its full release run publishes the tag. Underlay
uses the read-only plan and standalone gates so the complete local commit can
be reviewed before publication.

1. Land the release changes and their `[Unreleased]` changelog entries on
   `main` with QA green. Start on a clean local `main` matching `origin/main`.
   For a consumer-affecting release, include the contract 023 upgrade block
   with the impact class, exact consumer actions, any deprecation window,
   validation commands, and rollback tag.
2. Review `effigy release --plan` and confirm the proposed version against the
   versioning rules above.
3. Set all three version fields. Refresh `Cargo.lock` with
   `oxide update --workspace --quiet`. Insert `## [X.Y.Z] - YYYY-MM-DD` below
   `[Unreleased]` so the notes move under it with their formatting intact.
4. Inspect the five-file diff. Only workspace-crate versions should change in
   `Cargo.lock`. In `v0.9.11`, an unintended third-party `rsa` version made the
   release unusable and required a new patch tag.
5. Run `effigy release gates` on the prepared files. Every gate declared in
   `[release.gates]` in [effigy.toml](../../../effigy.toml) must pass, including
   the version-sync check across the manifests and OpenAPI contract.
6. Commit the reviewed files as `release: vX.Y.Z`.

## Publish

After the operator authorizes publication, confirm local `main` is the release
commit, its parent is still the remote `main` tip, and the tag is unused locally
and remotely. Create the annotated tag with
`git tag -a vX.Y.Z -m "Release vX.Y.Z"`. Push the commit and tag together with
`git push --atomic origin main refs/tags/vX.Y.Z`.

Publish the corresponding GitHub Release using the exact version section
from the tagged changelog. Pass the section on standard input to
`gh release create vX.Y.Z --verify-tag --title "Underlay vX.Y.Z" --notes-file -`.

## Verify

- The `rust.yml` and `ci.yml` workflows pass on the release commit. `rust.yml`
  runs the Rust build, test, clippy, format, and MSRV jobs; `ci.yml` runs the
  `ts-and-docs` and `cargo-deny` jobs.
- A consumer can resolve the new tag on both language surfaces.
- For a consumer-affecting release, one reference consumer upgrades from the
  tag and runs its own validation.

## Roll back

Tags are immutable. Consumers roll back by pinning the previous known-good tag.
Fix forward with a new patch release, and say in its changelog entry which tag
to skip.
