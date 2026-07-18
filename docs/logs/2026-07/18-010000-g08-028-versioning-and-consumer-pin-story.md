# 2026-07-18 - g08.028 versioning and consumer-pin story

## Context

Everything was `0.0.1`; the six consumers ride underlay HEAD via sibling
relative-path deps (Cargo `path`, npm `file:`) with no version, git rev, or tag.
Any local change is instantly visible to all consumers, with no way to hold one
back and no bisectable boundary. Fine while one maintainer controls every repo,
but fragile.

## Decision

Keep **path deps as the default** lockstep-development workflow — it is correct
and lowest-friction for a solo-controlled fleet (the stop condition's concern).
Do not impose tags or convert the fleet. Add an **optional** pin mechanism so a
consumer can hold back or bisect when needed, and so drift is visible.

## Changes

- Bumped the shared version `0.0.1 -> 0.8.0` (Cargo `workspace.package.version` +
  `package.json`), tracking the active generation (`g08`). All 36 crates inherit
  `version.workspace`. `Cargo.lock` bumped surgically — 36 version lines only,
  reverting the transitive churn `cargo generate-lockfile` would have introduced
  (confirmed all `0.0.1` lock entries were underlay crates, so a scoped sed was
  safe). Path-dep consumers are unaffected: bare `path`/`file` deps carry no
  version requirement.
- Contract `023` gains a "Versioning And Consumer Pin" section: path-dep default
  for lockstep dev, git-tag for hold-back, with the exact Cargo (`{ git, tag }`)
  and npm (`github:...#tag`) switch recipes and the tag convention.
- Tagged `v0.8.0` (annotated) at the generation proof point.

## Validation

- `v0.8.0` pushed and resolvable via `git ls-remote --tags origin`, so a
  consumer `{ git, tag }` dep resolves. The full consumer-side rebuild-through is
  intentionally not run — it would mutate a live consumer repo mid-generation;
  the resolvable tag + documented recipe are the deliverable.
- `effigy validate` clean; `cargo metadata` resolves the workspace at `0.8.0`.

## Consumer Upgrade Notes

Impact class **process**. Adds an optional pin mechanism (version visibility +
git-tag hold-back); does not force any migration. Consumers stay on path deps by
default.

## Next

`g08.029` i18n message-seam decision (planning gate).
