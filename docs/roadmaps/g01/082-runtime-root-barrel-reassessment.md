---
title: Runtime Root Barrel Reassessment
owner: Codex
status: complete
updated: 2026-03-30
---

# Runtime Root Barrel Reassessment

## Goal

Decide whether the root `@decodelabs/underlay/runtime` barrel should stay broad
or be trimmed now that the narrower runtime subpaths are explicit and
documented.

## Outcome

The root runtime barrel stays as an explicit retained convenience surface.

It should not be treated as the primary teaching surface for new narrow-domain
contracts, but it still earns retained ownership because the active app and
client caller family uses it broadly and coherently.

## Judgment

Trimming the root barrel now would be churn, not cleanup.

The live usage signal is still broad across:

- `acme-admin`
- `cp-admin`
- `acme-front`
- `acme-client`
- `cp-client`
- retained guides/examples

The root barrel is still the practical compatibility surface for:

- navigation helpers
- auth/browser/runtime helpers
- data/media controllers
- shared formatter and utility exports

The narrower subpaths are still the right place to shape future boundaries:

- `runtime/auth`
- `runtime/browser`
- `runtime/forms`
- `runtime/navigation`
- `runtime/feedback`
- `runtime/i18n`
- `runtime/data`
- `runtime/relations`
- `runtime/media`
- `runtime/ai`

So the right split is:

- keep the root barrel stable for existing consumers and convenience imports
- use subpaths when documenting or introducing new focused contracts
- keep tightening the subpaths first before considering a root-barrel reduction

## Changes

- documented the root runtime barrel as an explicit retained convenience API
- updated the active architecture and guide surface so the import rule is clear
- updated roadmap front doors and durable inventory

## Next Task

Take the next retained-runtime audit on the `client` / `runtime` seam, and
decide whether any helpers currently exposed from `runtime` should instead live
under `client`, especially where browser/runtime orchestration and transport
helpers still sit close together.
