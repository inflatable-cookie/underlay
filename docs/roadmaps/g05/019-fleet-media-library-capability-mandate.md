# g05.019 — Fleet Media Library Capability Mandate

## Why

`g05.004` proved the retained media workflow family across the four admin apps
that already owned a real media library:

- `underlay-reference/acme-admin`
- `acowtancy/dairy`
- `contact-patch/cp-admin`
- `loophole/composer/composer-admin`

That did **not** settle the fleet-wide policy across all six consumer apps.

Current reality is still split:

- full media admin family present in 4 apps
- partial or absent in:
  - `compli-me`
  - `songsprout`

The missing question is no longer “what should the shared media shells be?”
That is already answered. The missing question is “what media capability set is
required across the whole fleet?”

## Goal

Define the required media-library capability posture across all six Underlay
consumer apps, then either:

- roll the missing apps to that posture
- or record explicit exceptions with a real contract-backed reason

## Scope

Primary targets:

- six-site media capability inventory
- required versus optional media features
- full / partial / absent classification rules
- whether every site must own the same admin media family
- rollout plan for `compli-me` and `songsprout` if the answer is yes

Likely outputs:

- one capability matrix artifact
- one contract or contract tightening around fleet media expectations
- follow-on rollout cards if missing apps need implementation work

## Current Inventory

### Full admin media family present

- `underlay-reference/acme-admin`
- `acowtancy/dairy`
- `contact-patch/cp-admin`
- `loophole/composer/composer-admin`

Shared retained posture already proved there:

- media root via app-local `MediaList` wrappers
- `MediaUploadPage`
- `MediaDetailWorkflowPage`
- media trash on `EntityTrashPage`

### Media admin family absent or partial

- `compli-me/admin`
  - no `/media`
  - no `/media/upload`
  - no `/media/[id]`
  - no `/media/trash`
- `songsprout/greenhouse`
  - no admin media route family present in the current app shell

### API side

Strong media API family present:

- `underlay-reference/acme-api`
- `acowtancy/farmyard`
- `contact-patch/cp-api`
- `loophole/composer/composer-api`

Partial or absent relative to that family:

- `compli-me/api`
- `songsprout/nursery`

## Decision Pressure

There are only three honest fleet postures:

1. every consumer app must own the full admin media family
2. media is a product-capability family, so some apps may remain explicitly
   absent
3. there is a smaller required shared baseline, with optional richer media
   workflows on top

`g05.004` did not answer this. It only converged the four apps that already had
the family.

## Consumer Upgrade Impact

Expected:

- clearer fleet-wide media expectations
- less ambiguity about whether missing media features are drift or product
  choice
- bounded rollout work if the missing apps need media brought in

Landed:

- [`docs/contracts/051-media-library-fleet-capability-policy.md`](/Users/tom/Dev/projects/underlay/docs/contracts/051-media-library-fleet-capability-policy.md)
- [`docs/contracts/media-capability/fleet-media-capability-matrix.csv`](/Users/tom/Dev/projects/underlay/docs/contracts/media-capability/fleet-media-capability-matrix.csv)

## Outcome

The fleet policy is now explicit:

- all six consumer admin apps are expected to own the full media admin family
- `compli-me` is currently `absent`
- `songsprout` is currently `partial`
- the other four app families are already `full`

## Current State

`g05.019` is complete.

The next useful execution lane is:

- `g05.020` compli-me and songsprout media family rollout

## Next Task

Execute `g05.020`: roll the missing media family into `songsprout` first, then
take `compli-me` as the second rollout batch.
