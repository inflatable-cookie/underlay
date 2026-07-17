# g08.028 - Versioning And Consumer-Pin Story

Status: planned
Owner: repo maintainers
Started:
Completed:

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

## Planned Changes

- [ ] Tag underlay at each six-consumer-proof point so consumers *can* pin via
  git dep when needed.
- [ ] Bump a shared workspace version per breaking batch (even unpublished) so
  drift is visible.
- [ ] Document the pin/upgrade path in `023` (path-dep for lockstep dev, git-tag
  for hold-back).

## Consumer Upgrade Impact

Impact class: `process`. Adds an optional pin mechanism; does not force
migration.

## Validation

- [ ] a consumer can switch one dep from path to git-tag and build
- [ ] `effigy validate`

## Stop Conditions

Decision card; the solo-dev workflow may prefer to stay on path deps - surface
the trade-off rather than imposing tags.

## Next Task

`g08.029` i18n message-seam decision (planning gate).
