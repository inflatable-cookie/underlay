# g06.189 Artifact - Devtools Run Bundle Ref Typing

Status: complete
Owner: repo maintainers
Completed: 2026-06-06

## Purpose

Close the remaining raw-string replay ref edge in `underlay-devtools`.

Migration bundle publish and pull may still use tag refs, but replay must be
digest-pinned. The previous `BundleRunOptions` shape stored `bundle_ref` as a
public `String` and reparsed it inside `migration_run`.

## Result

`BundleRunOptions` now stores `MigrationBundleRef` directly.

Retained constructors:

- `BundleRunOptions::new`
- `BundleRunOptions::from_bundle_ref`
- `BundleRunOptions::parse_bundle_ref`

The CLI parses `--bundle` at the command edge and only calls `migration_run`
with typed run options.

The audit table comment was also corrected to remove stale compatibility text
about raw audit APIs.

## Consumer Upgrade Impact

Impact class: `breaking-tooling-api`.

App runtime crates should not depend on `underlay-devtools`. The six-consumer
scan found no consumer use of `underlay_devtools::BundleRunOptions`.

Only direct devtools library callers that constructed `BundleRunOptions` with a
struct literal need to switch to `BundleRunOptions::parse_bundle_ref` or build a
`MigrationBundleRef` first.

Publish and pull option shapes are unchanged.

## Validation

- `cargo test -p underlay-devtools migration_bundle --all-features`
- six-consumer source scan for `BundleRunOptions`

## Next Task

Continue the `122` candidate-type audit with `underlay-media` storage string
helpers, or re-enter planning if more post-closeout `g06` repairs would create
roadmap churn.
