# Contract: API Canonical Path Cutovers and Compatibility Retirement

Status: active
Owner: repo maintainers
Depends on: `020-http-transport-and-server-boundary.md`, `025-rust-app-runtime-assembly-and-router-topology.md`, `026-route-families-and-access-model.md`, `118-front-and-shared-read-api-shapes.md`

## Purpose

Define how Underlay apps should migrate from older API paths to canonical
paths without leaving compatibility posture implicit.

This contract covers:

- when a canonical path cutover is justified
- what kind of compatibility alias is allowed during migration
- client and server sequencing
- when old paths can be retired
- what should not be migrated in the same batch

It does not define the route-family taxonomy itself. That stays with `026`.
It does not define read envelopes. That stays with `118`.

## Sources of Truth

Primary shared sources:

- [`docs/contracts/020-http-transport-and-server-boundary.md`](/Users/tom/Dev/projects/underlay/docs/contracts/020-http-transport-and-server-boundary.md)
- [`docs/contracts/025-rust-app-runtime-assembly-and-router-topology.md`](/Users/tom/Dev/projects/underlay/docs/contracts/025-rust-app-runtime-assembly-and-router-topology.md)
- [`docs/contracts/026-route-families-and-access-model.md`](/Users/tom/Dev/projects/underlay/docs/contracts/026-route-families-and-access-model.md)
- [`docs/contracts/118-front-and-shared-read-api-shapes.md`](/Users/tom/Dev/projects/underlay/docs/contracts/118-front-and-shared-read-api-shapes.md)

Reference migration evidence:

- [`docs/roadmaps/g05/009-rust-runtime-contract-audit-and-next-contract-set.md`](/Users/tom/Dev/projects/underlay/docs/roadmaps/g05/009-rust-runtime-contract-audit-and-next-contract-set.md)
- [`loophole/composer/composer-api/crates/api/src/routes/admin.rs`](/Users/tom/Dev/projects/loophole/composer/composer-api/crates/api/src/routes/admin.rs)
- [`loophole/composer/composer-api/crates/api/src/routes/shared.rs`](/Users/tom/Dev/projects/loophole/composer/composer-api/crates/api/src/routes/shared.rs)
- [`loophole/composer/composer-api-client/src/commands/vendor-commands.ts`](/Users/tom/Dev/projects/loophole/composer/composer-api-client/src/commands/vendor-commands.ts)
- [`loophole/composer/composer-api-client/src/commands/product-commands.ts`](/Users/tom/Dev/projects/loophole/composer/composer-api-client/src/commands/product-commands.ts)
- [`loophole/composer/composer-api-client/src/commands/parameter-commands.ts`](/Users/tom/Dev/projects/loophole/composer/composer-api-client/src/commands/parameter-commands.ts)
- [`loophole/composer/composer-api-client/src/commands/mapping-commands.ts`](/Users/tom/Dev/projects/loophole/composer/composer-api-client/src/commands/mapping-commands.ts)
- [`loophole/composer/composer-api-client/src/commands/hardware-commands.ts`](/Users/tom/Dev/projects/loophole/composer/composer-api-client/src/commands/hardware-commands.ts)

If these diverge, the contract plus the cleanest current cutover proof win.

## Contract Goal

Underlay should make API path cleanup predictable.

A new app should not have to rediscover:

- whether to keep old and new paths live at the same time
- whether reads and writes should be retired together
- whether client adoption must happen before path retirement
- whether a family is a real redesign or only a canonical-path cleanup

The goal is one declared cutover pattern instead of app-local alias policy.

## Scope Boundary

In scope:

- API route renames and canonical-path migrations
- temporary compatibility aliases
- retirement order for older paths
- client/server sequencing for path cutovers

Out of scope:

- DTO or envelope migrations
- auth or role semantics
- product-level permission redesign
- broad route-family taxonomy

## Shared Boundary

### Canonical path rule

When a route family already has a clearer canonical placement under `026`, new
clients and new server-side examples should use the canonical path.

Typical examples:

- canonical shared auth:
  - `/v1/auth/*`
- canonical admin writes:
  - `/v1/admin/*`

Rules:

- canonical paths should be introduced before old paths are retired
- canonical paths should reflect the real family:
  - shared
  - front/public
  - admin
- do not mint a canonical path that lies about the real access model

### Compatibility alias rule

Compatibility aliases are allowed as a temporary migration seam.

Allowed forms:

- old path and canonical path both route to the same handler
- old path remains server-live while clients are moved
- instrumentation and docs point at the canonical path first

Rules:

- aliases are compatibility posture, not the preferred shared posture
- aliases must be explicit in roadmap or audit inventory
- aliases should be narrow:
  - same handler
  - same semantics
  - same envelope
- do not stack multiple generations of aliases

### Read and write split rule

Do not assume reads and writes retire together.

Rules:

- when a family has genuinely shared or front-facing reads, those reads may
  stay flat or mixed while writes move to canonical admin paths
- a mixed family may validly end in:
  - flat/shared reads
  - canonical admin writes
- do not force mixed reads under `/v1/admin/*` just for symmetry

This was the right outcome for the `composer-api` catalog families.

### Client-first retirement rule

Retire old paths only after the corresponding callers already use the canonical
path.

Rules:

- client command surfaces should move first
- server retirement should follow once the old path is no longer the live
  caller path
- do not retire old paths and repoint all clients in the same opaque batch
  unless the caller set is trivially local and verified

### Mutation-first retirement rule

Prefer retiring write-path compatibility before read-path compatibility.

Rules:

- admin-only mutations are the safest first removal target
- mixed-family reads should be left alone unless the read family is itself
  being redesigned
- helper reads should not be retired during a write-path cleanup batch unless
  they are genuinely obsolete

### No fake redesign rule

Do not describe a narrow path cleanup as a full resource redesign.

Examples of narrow cutovers:

- `/v1/auth/local/*` to `/v1/auth/*`
- flat admin-only writes to `/v1/admin/*`

Rules:

- if the resource shape, envelope, and access model stay the same, this is a
  compatibility retirement, not a new API generation
- avoid dragging envelope, payload, or product-flow changes into the same batch
  unless the redesign is truly intentional

## Migration Sequence

Default sequence:

1. classify the family honestly
   - canonical shared
   - canonical admin
   - mixed reads with admin writes
2. introduce canonical path aliases on the server
3. repoint live clients and workflow commands
4. update docs, OpenAPI, and inventory to treat the canonical path as primary
5. retire old write aliases
6. retire old read aliases only if the read family is also being redesigned

## What Good Looks Like

Good outcomes:

- `/v1/auth/*` is primary and `/v1/auth/local/*` is an explicit temporary alias
- admin mutations live only under `/v1/admin/*`
- mixed catalog families keep flat/shared reads when that is their real shape
- roadmap and inventory record where compatibility still exists

Bad outcomes:

- old and new paths both stay primary indefinitely
- aliases exist but are undocumented
- reads are moved under `/v1/admin/*` even though they are shared/product reads
- path cleanup batches also change envelopes, role rules, and product behavior
  without saying so

## Questions This Contract Should Settle

- Which remaining compatibility aliases are real debt versus deliberate shared
  shape?
- When is a family done once canonical paths exist?
- Which path retirements are safe to do by family rather than one route at a
  time?

## Assessment State

Assessed across all six consumer APIs by `g09.057` on 2026-08-27.

Verdict: `drifting` in three explicit mutation-alias families.

- Songsprout and Acowtancy retain passkey connect aliases over canonical
  register handlers.
- Composer retains `/v1/auth/local/{login,refresh,logout}` aliases over the
  canonical shared auth paths.
- Composer and Songsprout in-repo callers are canonical; Acowtancy still needs
  a client-first move.
- none of the three families records an authorised external compatibility
  window or removal trigger.

No other same-handler mutation alias was found. Mixed Composer catalog reads
remain an intentional shared-read/admin-write split, not compatibility debt.
See the
[`g09.057` assessment](../logs/2026-08/27-175930-g09-057-canonical-path-runtime-workflow-assessment.md).
Decision-gated follow-through is preserved in `g09.058`.

## Next Task

Resolve the per-target compatibility windows in `g09.058` before promoting any
alias retirement.
