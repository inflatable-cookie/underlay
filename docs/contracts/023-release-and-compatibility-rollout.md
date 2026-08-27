# Contract: Release and Compatibility Rollout

Status: active
Owner: repo maintainers
Depends on: `001-working-rules.md`, `022-testing-posture-and-shared-harnesses.md`, `027-api-canonical-path-cutovers-and-compatibility-retirement.md`, `111-consumer-template-adoption-and-exception-policy.md`

## Purpose

Define how shared Underlay changes should roll through the consumer fleet.

This contract covers:

- when a shared change needs an explicit rollout plan
- compatibility alias and deprecation-window posture at the fleet level
- cross-repo rollout order
- release-note and upgrade-note expectations
- proof required before a compatibility surface can be retired

It does not redefine API path cutover mechanics. That stays in `027`.

## Sources of Truth

Shared release and upgrade guidance:

- [`docs/guides/190-upgrade-compatibility.md`](../guides/190-upgrade-compatibility.md)
- [`docs/guides/200-project-sync.md`](../guides/200-project-sync.md)
- [`docs/guides/code/190-upgrade-compatibility/feature-upgrade-note-template.md`](../guides/code/190-upgrade-compatibility/feature-upgrade-note-template.md)
- [`docs/guides/code/190-upgrade-compatibility/release-log-upgrade-block-template.md`](../guides/code/190-upgrade-compatibility/release-log-upgrade-block-template.md)

Prior rollout and retirement evidence:

- [`docs/roadmaps/g01/031-consumer-upgrade-and-change-communication.md`](../roadmaps/g01/031-consumer-upgrade-and-change-communication.md)
- [`docs/roadmaps/g05/009-rust-runtime-contract-audit-and-next-contract-set.md`](../roadmaps/g05/009-rust-runtime-contract-audit-and-next-contract-set.md)
- [`027-api-canonical-path-cutovers-and-compatibility-retirement.md`](./027-api-canonical-path-cutovers-and-compatibility-retirement.md)
- [`111-consumer-template-adoption-and-exception-policy.md`](./111-consumer-template-adoption-and-exception-policy.md)

Consumer fleet evidence:

- `underlay-reference`
- `acowtancy`
- `compli-me`
- `contact-patch`
- `songsprout`
- `loophole/composer`

If these diverge, the contract plus the clearest modern rollout posture win.

## Contract Goal

Underlay should make cross-repo rollout boring.

A normal shared change should not leave teams guessing:

- whether a compatibility alias is needed
- whether the change is additive, deprecating, or breaking
- which repo should move first
- how long an old surface may stay live
- what proof is required before retirement

The goal is one declared fleet-rollout posture instead of case-by-case
judgment.

## Scope Boundary

In scope:

- shared TS, Svelte, Rust, config, migration, and docs changes that affect
  consumer apps
- compatibility windows
- release and upgrade-note expectations
- rollout order across Underlay and the six consumer repos
- retirement proof for deprecated shared surfaces

Out of scope:

- app-internal release process
- package publishing mechanics
- CI implementation details
- one-off emergency fixes that do not change consumer obligations

## Shared Boundary

### Rollout-plan trigger rule

A change needs an explicit rollout plan when it alters:

- public APIs
- public exports or import paths
- configuration keys or required env/config structure
- migrations or database bring-up posture
- recommended integration patterns
- shared page, workflow, or runtime ownership rules

Rules:

- do not treat consumer rollout planning as optional follow-up work
- add `Consumer Upgrade Impact` in the active roadmap batch
- ship the upgrade note or compatibility note in the same batch

### Impact classification rule

Every consumer-affecting shared change must be classified as one of:

- `additive`
- `deprecation`
- `breaking`

Rules:

- use `additive` when adoption is optional and no existing caller breaks
- use `deprecation` when the old surface still works temporarily but has a
  declared replacement and sunset plan
- use `breaking` when consumers must change code or config to stay working

### Compatibility window rule

Compatibility windows are allowed only when they buy real fleet safety.

Allowed reasons:

- server and client cannot cut over atomically
- multiple consumer repos need staged adoption
- config or migration transitions need dual-read, dual-route, or warning-first
  posture

Rules:

- compatibility windows must be explicit, not implied
- write concrete dates when a deprecation window or sunset exists
- do not keep aliases or compatibility exports indefinitely because they are
  convenient
- do not stack multiple overlapping generations of fallback surface

### Cross-repo rollout order rule

Default rollout order:

1. land the shared Underlay change
2. add compatibility posture if needed
3. repoint the clearest reference consumer first
4. repoint the remaining affected consumers
5. update docs and inventories to treat the new surface as primary
6. retire the old surface once consumer proof exists

Rules:

- prefer `underlay-reference` as the first consumer proof when the change fits
  the reference app
- use the most directly affected live app first when the proof is not a good
  fit for `underlay-reference`
- do not retire a surface before the live callers have already moved

### Release-note rule

Every consumer-affecting shared batch must ship release-facing upgrade notes.

Minimum output:

- impact class
- exact consumer actions
- deprecation window or cutoff date when relevant
- validation commands
- links to the changed guides, contracts, roadmap, or logs

Rules:

- use the existing upgrade templates instead of ad hoc rollout prose
- keep the compact release-log block brief and link out when the rollout has
  more than one step
- keep recurring fleet policy in the guide layer, not only in logs

### Validation-before-retirement rule

Do not retire a compatibility surface until the replacement is already proved.

Minimum proof:

- the active callers already use the replacement path, export, config key, or
  template
- the owning repo batch ran the normal validation commands
- the roadmap or inventory records the retirement

Stronger proof when the change is broad:

- one reference consumer proof
- one additional live consumer proof
- explicit upgrade note showing the replacement steps

### Mutation-first and narrow-first rule

Prefer retiring the safest narrow compatibility surface first.

Typical order:

- admin-only writes before mixed reads
- exports/import paths before larger product-flow changes
- thin shell retirements before inner workflow redesigns

Rules:

- keep retirements narrow and legible
- do not mix path, payload, auth, config, and product-flow redesign into one
  opaque batch unless the redesign is truly intentional

## Versioning And Consumer Pin

Underlay is unpublished (`private: true`). The six consumers ride it via sibling
relative-path dependencies — Cargo `path = "../../underlay/..."`, npm
`file:../../underlay`. This is the **default lockstep-development workflow** and
stays the default: while one maintainer controls every repo, path deps give the
tightest feedback loop (a local change is instantly testable across the fleet)
and there is no publish step to manage. This card does **not** move consumers off
path deps.

What it adds is an **optional pin mechanism** so a consumer *can* hold back or
bisect when needed, and so drift is visible:

- **Shared version reflects the generation.** The Cargo workspace and
  `package.json` version track the active generation (`g08` -> `0.8.0`). Bump the
  minor per generation (or per breaking batch within a generation) so the version
  string moves even though nothing is published. Path-dep consumers are
  unaffected — bare `path`/`file` deps carry no version requirement.
- **Tag each six-consumer-proof point.** After a batch passes the six-consumer
  proof, tag underlay (`v0.8.0`, or `v0.8.0-<lane>` for intra-generation
  checkpoints). The tag is the bisectable boundary the path-dep workflow lacks.

### Holding a consumer back (path -> git tag)

To pin one consumer to a proven underlay revision while others track HEAD, switch
just that consumer's dependency from the sibling path to the git tag:

```toml
# Cargo (consumer): was  underlay-core = { path = "../../underlay/rust/crates/underlay-core" }
underlay-core = { git = "https://github.com/inflatable-cookie/underlay.git", tag = "v0.8.0" }
```

```jsonc
// npm (consumer): was  "@inflatable-cookie/underlay": "file:../../underlay"
"@inflatable-cookie/underlay": "github:inflatable-cookie/underlay#v0.8.0"
```

Revert to the `path`/`file` form to rejoin lockstep development. Keep the switch
scoped to the one consumer that needs the hold-back; do not convert the fleet.

## When A Broad Rollout Plan Is Not Required

An explicit fleet rollout plan is usually not required when the change is:

- additive and unused by existing consumers
- docs-only with no consumer obligation change
- internal refactoring with no export, behavior, or integration drift
- a local consumer fix with no shared-surface impact

Rules:

- still classify the change honestly
- do not claim "internal only" when the public surface or recommended pattern
  changed

## What Good Looks Like

Good outcomes:

- roadmap batches name the impact class and rollout posture clearly
- compatibility aliases are time-boxed and documented
- shared changes move through a visible repo order instead of surprise breakage
- retirements happen only after caller proof exists
- release logs and upgrade notes tell consumers exactly what to do

Bad outcomes:

- shared breakage lands with no upgrade note
- aliases stay live with no sunset or inventory
- retirements happen before consumer callers are moved
- broad shared changes are merged with only local-repo proof and no fleet read

## Next Task

Use this contract whenever a shared Underlay change affects more than one
consumer repo or introduces a compatibility window, upgrade obligation, or
retirement plan.
