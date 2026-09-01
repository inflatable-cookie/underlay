# Contract: Media Library Fleet Capability Policy

Status: active
Owner: repo maintainers
Depends on: `040-storage-blob-and-media-systems.md`, `050-media-library-and-usage.md`, `110-admin-template-system.md`, `111-consumer-template-adoption-and-exception-policy.md`

## Purpose

Define the fleet-wide media-library capability requirement across the current
Underlay consumer app family.

This contract covers:

- which admin media capabilities are required across the five consumer apps
- how apps are classified as full, partial, or absent
- what counts as fleet drift versus an explicit exception
- the route and shell posture expected when an app owns the media family

It does not redefine the lower storage/media semantics in `040` and `050`. It
defines whether the full media admin family is required across the consumer
fleet.

## Sources of Truth

Shared media contracts:

- [`040-storage-blob-and-media-systems.md`](./040-storage-blob-and-media-systems.md)
- [`050-media-library-and-usage.md`](./050-media-library-and-usage.md)
- [`110-admin-template-system.md`](./110-admin-template-system.md)
- [`111-consumer-template-adoption-and-exception-policy.md`](./111-consumer-template-adoption-and-exception-policy.md)

Convergence evidence:

- [`docs/roadmaps/g05/004-cross-app-media-library-template-consolidation.md`](../roadmaps/g05/004-cross-app-media-library-template-consolidation.md)
- [`docs/contracts/media-capability/fleet-media-capability-matrix.csv`](./media-capability/fleet-media-capability-matrix.csv)

If these diverge, the contract plus the capability matrix win.

## Contract Goal

Underlay should stop treating media-library presence as an accidental per-app
difference.

The current consumer fleet should have one declared answer to:

- whether every admin app must own the media library
- whether missing media routes are drift
- what the minimum required media route family is

The goal is a fleet capability rule, not another local media-proof exercise.

## Scope Boundary

In scope:

- admin media capability across the five consumer apps
- API support required for that admin family
- full versus partial versus absent classification
- shared route/shell posture for required media capability

Out of scope:

- public/front media surfaces
- app-specific media business rules
- lower blob/storage contracts already covered elsewhere

## Shared Boundary

### Fleet requirement rule

The current five-app admin fleet is expected to own the full admin media family.

Affected apps:

- `underlay-reference/apps/acme-admin`
- `acowtancy/apps/dairy`
- `compli-me/apps/admin`
- `contact-patch/apps/cp-admin`
- `songsprout/apps/greenhouse`

Rules:

- missing the media admin family is fleet drift, not just a local product gap
- an app may stay outside this rule only if a later roadmap or contract opens a
  deliberate exception explicitly
- until such an exception exists, the target posture is full capability in all
  six admin apps

### Required admin route family

The required admin media route family is:

- media root
- media upload
- media detail
- media trash

Expected route shape:

- `/media`
- `/media/upload`
- `/media/[mediaId]` or equivalent stable id segment
- `/media/trash`

Rules:

- small parameter-name variation is acceptable
- route-family absence is not acceptable

### Required shared shell rule

When an app owns the media family, it should use the retained shared posture:

- media root through an app-local `MediaList` wrapper over `EntityListPage`
- media upload through `MediaUploadPage`
- media detail through `MediaDetailWorkflowPage`
- media trash through `EntityTrashPage`

Rules:

- do not re-open app-local shell dialects for these surfaces
- route-owned workflow internals may still differ where the template contract
  allows that

### API support rule

An app with the required admin media family must also expose the supporting API
family for:

- media list/detail
- upload initiate/finalise
- versions
- usage
- soft-delete/restore/purge or the equivalent fleet trash workflow

Rules:

- API implementation may vary internally
- operator-visible capability absence still counts as fleet drift if the admin
  app is expected to support the full media family

### Classification rule

Use these statuses in the capability matrix:

- `full`
- `partial`
- `absent`

Meaning:

- `full`
  - owns the full required admin media route family and supporting API
- `partial`
  - owns some lower media/storage wiring or some API/media routes, but not the
    full required admin family
- `absent`
  - does not currently expose the required media family

Rules:

- the matrix records current state and target state separately
- for this fleet policy, target state is `full` unless an explicit exception is
  recorded later

## Current Policy Read

Current full apps:

- `underlay-reference`
- `acowtancy`
- `contact-patch`

Current fleet status:

- all five consumer admin apps are now `full`
- `songsprout` and `compli-me` reached full status in `g05.020`

## What Good Looks Like

Good outcomes:

- every admin app in the fleet has the same media quartet
- the API family behind that quartet is consistently present
- future audits classify missing media as drift immediately instead of
  rediscovering the policy

Bad outcomes:

- some apps silently remain media-less with no explicit exception
- the shared media shells are retained but only half the fleet can use them
- lower blob support is mistaken for full media-library capability

## Next Task

Keep the capability matrix current if a future app falls behind, or if a later
fleet policy explicitly carves out an exception.
