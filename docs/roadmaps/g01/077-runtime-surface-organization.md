# 077 - Runtime Surface Organization

Status: Complete
Owner: Platform
Created: 2026-03-30
Depends on: 076

## Overview

`g01.076` introduced `@decodelabs/underlay/runtime`, but the new namespace was
still just a flat compatibility barrel over `patterns/*`. That was enough to
separate workflow shells from helpers, but not enough to make the retained
runtime surface legible.

This batch turns `runtime` into a deliberately organized helper package:

- keep the root runtime barrel stable
- add domain sub-barrels for the retained helper families
- expose public runtime subpaths in `package.json`
- update docs so consumers see `runtime` as an organized namespace instead of a
  catch-all dump

## Delivery

- added curated runtime sub-barrels in `ts/src/runtime/`:
  - `auth.ts`
  - `browser.ts`
  - `forms.ts`
  - `navigation.ts`
  - `feedback.ts`
  - `i18n.ts`
  - `data.ts`
  - `relations.ts`
  - `media.ts`
  - `ai.ts`
- narrowed `ts/src/runtime/index.ts` to compose from those sub-barrels instead
  of re-exporting a long flat list directly from `patterns/*`
- added matching public subpath exports in `package.json`
- updated the guide and roadmap surface so the retained runtime categories are
  explicit

## Boundary Result

Public Underlay runtime surface now has an intentional shape:

- `@decodelabs/underlay/runtime/auth`
- `@decodelabs/underlay/runtime/browser`
- `@decodelabs/underlay/runtime/forms`
- `@decodelabs/underlay/runtime/navigation`
- `@decodelabs/underlay/runtime/feedback`
- `@decodelabs/underlay/runtime/i18n`
- `@decodelabs/underlay/runtime/data`
- `@decodelabs/underlay/runtime/relations`
- `@decodelabs/underlay/runtime/media`
- `@decodelabs/underlay/runtime/ai`

The root `@decodelabs/underlay/runtime` barrel remains stable for convenience,
but the supported retained helper surface is no longer one flat namespace.

## Consumer Upgrade Impact

- existing `@decodelabs/underlay/runtime` root imports continue to work
- consumers can now adopt narrower runtime subpaths where that improves local
  clarity
- workflow/page shells still belong on `@decodelabs/underlay/patterns`

## Next Task

The strongest next follow-on is a focused retained-runtime boundary challenge,
starting with the feedback helper family (`runtime/feedback`) and the question
of whether toast primitives now belong in Poodle rather than Underlay.
