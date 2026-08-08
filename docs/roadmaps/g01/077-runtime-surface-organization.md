# 077 - Runtime Surface Organization

Status: Complete
Owner: Platform
Created: 2026-03-30
Depends on: 076

## Overview

`g01.076` introduced `@inflatable-cookie/underlay/runtime`, but the new namespace was
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

- `@inflatable-cookie/underlay/runtime/auth`
- `@inflatable-cookie/underlay/runtime/browser`
- `@inflatable-cookie/underlay/runtime/forms`
- `@inflatable-cookie/underlay/runtime/navigation`
- `@inflatable-cookie/underlay/runtime/feedback`
- `@inflatable-cookie/underlay/runtime/i18n`
- `@inflatable-cookie/underlay/runtime/data`
- `@inflatable-cookie/underlay/runtime/relations`
- `@inflatable-cookie/underlay/runtime/media`
- `@inflatable-cookie/underlay/runtime/ai`

The root `@inflatable-cookie/underlay/runtime` barrel remains stable for convenience,
but the supported retained helper surface is no longer one flat namespace.

## Consumer Upgrade Impact

- existing `@inflatable-cookie/underlay/runtime` root imports continue to work
- consumers can now adopt narrower runtime subpaths where that improves local
  clarity
- workflow/page shells still belong on `@inflatable-cookie/underlay/patterns`

## Next Task

The strongest next follow-on is a focused retained-runtime boundary challenge,
starting with the feedback helper family (`runtime/feedback`) and the question
of whether toast primitives now belong in Poodle rather than Underlay.
