# 2026-04-11 09:15 - g02.007 Poodle package consolidation lane open

## Context

Poodle’s Svelte package boundary changed again after the broad
consumer-normalization line closed.

The old split package names:

- `@poodle/svelte-primitives`
- `@poodle/svelte-composites`

were consolidated behind:

- `@poodle/svelte`

## Decision

Opened `g02.007` as a new narrow migration lane instead of reopening the old
consumer-family rollout line.

## Initial Evidence

Live fallout is currently concentrated in:

- `underlay`
- `underlay-reference`
- `contact-patch`
- `loophole/composer`

Current residue scan says these families are already clean for the split
package names:

- `acowtancy`
- `compli-me`
- `songsprout`

## Next Task

Execute `g02.007` Batch 7.1 by migrating the live source imports, package
manifests, and local alias wiring in Underlay, `underlay-reference`,
`contact-patch`, and `loophole/composer`, then validate the touched repos
before widening into guide alignment.
