# Historical Tags Lack GitHub Releases

Status: open
Captured: 2026-08-27

## Observation

Remote tags `v0.9.0` through `v0.9.4` exist, but `gh release view v0.9.4` and
`gh release list` show no corresponding GitHub Release. The current Effigy
release protocol says execute creates both the immutable tag and GitHub Release.

## Impact

Consumers can pin the tags, but release notes and the release-facing changelog
links do not have the provider surface the current process promises.

## Disposition

Keep open. Do not rewrite or re-tag historical releases. Require the next patch
release to prove both remote tag and GitHub Release creation, then decide
separately whether historical GitHub Releases should be backfilled from their
existing immutable tags.
