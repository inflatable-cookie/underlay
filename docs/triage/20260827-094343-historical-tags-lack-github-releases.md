# Historical Tags Lack GitHub Releases

Status: open
Captured: 2026-08-27

## Observation

Remote tags `v0.9.0` through `v0.9.4` exist, but `gh release view v0.9.4` and
`gh release list` show no corresponding GitHub Release. Underlay's release
protocol now treats GitHub Release publication as a separate operator step
after `effigy release execute` (tag only); historical tags may still lack
that provider surface.

## Impact

Consumers can pin the tags, but release notes and the release-facing changelog
links do not have the provider surface the current process promises.

## Disposition

Keep open for an operator decision on historical GitHub Release backfill. The
next patch release proved the current publication path. Do not rewrite or
re-tag historical releases; any backfill must use the existing immutable tags.

## Current Evidence

Underlay `v0.9.5` proved the current path on 2026-08-27: annotated tag and
GitHub Release both exist at release commit `8ffafb92`, exact-source and
release-commit CI passed, and Rust plus TypeScript tagged-consumer smokes
resolved the immutable tag. Historical backfill remains a separate operator
decision.
