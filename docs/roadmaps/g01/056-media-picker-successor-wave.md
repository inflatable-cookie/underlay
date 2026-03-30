# 056 - MediaPicker Successor Wave

Status: In Progress
Owner: Platform
Created: 2026-03-28
Depends on: 052, 055

## Overview

`g01.055` finished the `PageHeader` successor wave and retired public
Underlay `PageHeader`.

That leaves a smaller public successor queue again, and the next strongest
meaningful surface is `MediaPicker`. This is not dead wrapper residue, but it
is also not a true long-term retained boundary:

- Underlay `MediaPicker` is still a generic callback-driven media workflow shell
- Poodle already owns the browse and upload-status shell chrome
- the remaining Underlay gap is the reusable upload and duplicate-resolution
  workflow contract, not product-specific routing

This wave exists to move the reusable callback-driven media workflow out of
Underlay, place it in a Poodle-owned helper/controller layer over the existing
media surfaces, and retire public Underlay `MediaPicker` once the residue is
honestly clean.

## Research Basis

- Underlay:
  - `ts/src/components/MediaPicker.svelte`
  - `ts/src/components/media-picker/upload-flow.ts`
  - `ts/src/components/media-picker/browse.ts`
  - `ts/src/components/media-picker/state.ts`
- Poodle:
  - `../poodle/packages/svelte/composites/src/MediaPicker.svelte`
  - `../poodle/packages/svelte/composites/src/MediaBrowsePanel.svelte`
  - `../poodle/packages/svelte/composites/src/MediaUploadStatusPanel.svelte`
- caller sweep:
  - there are no current consumer-app `MediaPicker` callers in `acme-admin`,
    `cp-admin`, `acme-front`, or `dairy`
  - the remaining live surface is the Underlay public export plus nearby media
    workflow helpers and roadmap/docs posture

## Decision Summary

- Current Poodle `MediaPicker` is a simpler modal browse/upload shell over
  caller-owned items and upload events.
- Current Underlay `MediaPicker` still owns:
  - paginated browse loading
  - duplicate hash detection
  - metadata creation
  - upload initiation and finalisation
  - upload progress and duplicate-resolution state transitions
- The strict comparison shows the reusable workflow does not need to live
  inside a widened monolithic `MediaPicker` shell.
- The cleaner split is:
  - Poodle keeps the browse and upload-status presentation surfaces
  - a shared controller or workflow helper layer owns callback-driven browse,
    duplicate detection, upload orchestration, and upload-state transitions
- The next honest move is to prove that controller split in Poodle-owned media
  workflow helpers before retiring the public Underlay wrapper.

## Consumer Upgrade Impact

- Consumers should expect the eventual successor to stay callback-driven and
  backend-agnostic.
- Do not move app-specific media API or auth wiring into shared UI just to
  preserve the old shape mechanically.
- Prefer a reusable workflow contract over another long-lived Underlay wrapper.

## Planned Batches

## Batch 56.1 - Contract Reset

- [x] Reassess `MediaPicker` after `PageHeader` retirement.
- [x] Confirm that `MediaPicker` is the strongest next broad successor wave.
- [x] Open the focused roadmap and update front doors / durable inventory.

Completed in 56.1:
- `MediaPicker` is now the active successor wave.
- The meaningful gap is the generic upload and duplicate-resolution workflow
  layer, not the browse shell chrome.
- `DropdownMenu` remains the smaller later composition cleanup option if this
  wave stalls.

## Batch 56.2 - Poodle Media Workflow Review

- [x] Compare Underlay `MediaPicker` against current Poodle `MediaPicker`,
      `MediaBrowsePanel`, and `MediaUploadStatusPanel`.
- [x] Define the smallest honest reusable expansion or controller split for the
      callback-driven upload workflow.
- [x] Avoid pushing app-specific media API wiring into shared UI.

Completed in 56.2:
- Poodle `MediaPicker` is not the right expansion target for the remaining
  Underlay workflow logic. Its current role as a lighter local-item
  browse/upload shell is still correct.
- The reusable successor boundary is a shared media-workflow helper layer over
  Poodle `MediaBrowsePanel` and `MediaUploadStatusPanel`, not a second large
  composite that reabsorbs all workflow state into one component.
- The reusable helper/controller responsibilities are now explicit:
  - paginated browse page loading and merge behavior
  - duplicate-hash detection and upload decision branching
  - metadata creation plus upload initiation/finalisation orchestration
  - upload-step state transitions and progress reporting
- App-specific media API wiring, auth headers, route refresh, and
  post-selection behavior remain caller responsibilities.

## Batch 56.3 - Grouped Migration And Retirement

- [x] Move the reusable workflow helpers to a Poodle-owned media workflow
      layer or successor package shape over the existing media surfaces.
- [x] Replace Underlay `MediaPicker` with that Poodle-based workflow split in
      retained shared consumers or any surviving internal usage.
- [x] Retire public Underlay `MediaPicker` and update docs/inventory once the
      residue scan is honestly clean.

Completed in 56.3:
- The reusable media workflow helpers now live in Poodle over the existing
  media surfaces instead of inside a retained Underlay wrapper.
- Public Underlay `MediaPicker` and its local helper/story residue are gone.
- The media-library guide now teaches direct Poodle composition for both the
  lightweight local selector and the callback-driven workflow case.

## Completion

`g01.056` is complete. Underlay no longer exports `MediaPicker`.

## Next Task

Complete.
