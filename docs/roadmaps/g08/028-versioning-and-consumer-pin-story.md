# g08.028 - Versioning And Consumer-Pin Story

Status: done
Owner: repo maintainers
Started: 2026-07-18
Completed: 2026-07-18

## Purpose

Give consumers a way to pin. Everything is `0.0.1`; the six consumers
(`underlay-reference`, `acowtancy`, `compli-me`, `contact-patch`, `songsprout`,
`loophole/composer`) ride HEAD via sibling relative path deps (Cargo
`path = "../../underlay/..."`, npm `file:../../underlay`) with no lockfile pin,
git rev, or tag. Any local change is instantly visible to all consumers; there
is no way to hold one back and no bisectable boundary. Workable while one person
controls every repo, but fragile. Contract `023` exists but enforcement is
entirely convention.

## Evidence

- `Cargo.toml` / `package.json` (`0.0.1`, `private: true`)
- consumer deps (sibling path/file references)
- `docs/contracts/023-release-and-compatibility-rollout.md` (convention only)

## Governing References

- [023 Release and compatibility rollout](../../contracts/023-release-and-compatibility-rollout.md)

## Decision

Keep sibling **path deps as the default** lockstep-development workflow — it is
correct and lowest-friction while one maintainer controls every repo (the stop
condition's concern). Do not impose tags or convert the fleet. Instead add an
**optional** pin mechanism so a consumer *can* hold back or bisect, and so drift
is visible.

## Changes

- [x] Bumped the shared version `0.0.1 -> 0.8.0` (Cargo `workspace.package` +
  `package.json`) so the version tracks the generation and drift is visible. All
  36 crates inherit `version.workspace`; `Cargo.lock` updated surgically (36
  version lines only, no transitive churn). Path-dep consumers are unaffected
  (bare `path`/`file` deps carry no version requirement).
- [x] Documented the pin/upgrade story in contract `023` (new "Versioning And
  Consumer Pin" section): path-dep default for lockstep dev, git-tag for
  hold-back, with the exact Cargo/npm switch recipes and the tag convention
  (`v0.8.0`, `v0.8.0-<lane>` for intra-generation checkpoints).
- [x] Tagged the six-consumer-proof point `v0.8.0` (annotated) at the generation
  commit so consumers *can* pin via git dep.

## Consumer Upgrade Impact

Impact class: `process`. Adds an optional pin mechanism; does not force
migration.

## Validation

- [x] Pin mechanism resolvable: the `v0.8.0` tag is pushed and visible via
  `git ls-remote --tags origin`, so a consumer's `{ git, tag }` / `github:#tag`
  dep resolves. The exact switch recipe is documented in `023`. (The full
  consumer-side rebuild-through is intentionally not executed here — it would
  mutate a live consumer repo mid-generation; the recipe + resolvable tag are the
  deliverable.)
- [x] `effigy validate` clean; `cargo metadata` resolves at `0.8.0`.

## Stop Conditions

Decision card; the solo-dev workflow may prefer to stay on path deps - surface
the trade-off rather than imposing tags.

## Next Task

`g08.029` i18n message-seam decision (planning gate).
