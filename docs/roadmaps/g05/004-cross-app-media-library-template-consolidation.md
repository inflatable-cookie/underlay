# g05.004 — Cross-App Media Library Template Consolidation

## Why

The media library is now the clearest repeated workflow family that still sits
in app-local divergence rather than one retained shared template set.

It exists across four admin apps:

- `underlay-reference/acme-admin`
- `acowtancy/dairy`
- `contact-patch/cp-admin`
- `loophole/composer/composer-admin`

The broad shape is the same in all four:

- media browse list
- media detail workflow page
- media upload workflow page
- media trash workflow page

The trash side is already on `EntityTrashPage`. The remaining shared value is
to condense the live best behavior from the four apps into one coherent media
template family rather than letting each app keep its own media-shell dialect.

## Goal

Define and prove a retained shared media-library template family that covers the
normal repeated admin media workflows without flattening real app behavior.

Target outcome:

- one shared media browse/list posture
- one shared media upload workflow shell
- one shared media detail workflow shell
- existing media trash stays on `EntityTrashPage`
- route-owned workflow specifics still stay local where they truly differ

## Current Inventory

### Shared today

- card/list posture is already mostly shared through `EntityListPage`,
  `EntityListCard`, and app-local list wrappers
- trash posture is now shared through `EntityTrashPage`
- all four admin apps already have the same route family:
  - media root
  - media upload
  - media detail
  - media trash

### Still divergent

- media upload routes
- media detail routes
- detail-level preview, usage, rendition, and action layout
- app-local naming and section ordering drift

### Route inventory

`underlay-reference/acme-admin`

- `/media`
- `/media/upload`
- `/media/[mediaId]`
- `/media/trash`

`contact-patch/cp-admin`

- `/media`
- `/media/upload`
- `/media/[mediaId]`
- `/media/trash`

`loophole/composer/composer-admin`

- `/media`
- `/media/upload`
- `/media/[mediaId]`
- `/media/trash`

`acowtancy/dairy`

- `/media`
- `/media/upload`
- `/media/[id]`
- `/media/trash`

### Media root posture

This is effectively already converged.

All four apps now use:

- an app-local `MediaList` wrapper
- `EntityListPage`
- `EntityListCard`-based media cards

Conclusion:

- do not open a second retained media-root template unless the existing generic
  list shell proves too weak during the broader media-family work
- the shared value is likely contract cleanup and naming alignment, not another
  top-level list template

### Media trash posture

This is already converged enough:

- all four admin apps now route media trash through `EntityTrashPage`

Conclusion:

- media trash stays under the retained `EntityTrashPage` family
- no new media-specific trash shell is needed

### Media upload posture

This is the strongest immediate template candidate.

Observed shape:

- upload header with back-link
- bulk queue mode
- replace-single-file mode
- duplicate detection
- upload progress rendering
- success/failure summary messaging

Current implementation state:

- `underlay-reference` and `contact-patch` are extremely close
- `dairy` has the same workflow shape, but more helper extraction under
  `src/lib/media-upload/*`
- `composer-admin` is the same family with slightly leaner state naming and a
  narrower render surface

Best proven ingredients:

- upload-pipeline extraction from `underlay-reference` / `contact-patch`
- queue and replace section decomposition from `dairy`
- leaner route posture from `composer-admin`

Conclusion:

- retain one shared `MediaUploadPage` shell
- probably retain one lower-level queue/replace module pair under it
- keep app-local upload pipeline rules outside Underlay unless they are truly
  identical

### Media detail posture

This is the heavier but still realistic template candidate.

Two implementation styles are present today.

Style A: `underlay-reference`, `contact-patch`, `composer-admin`

- shared outer posture around `EntityDetailPage`
- metadata header
- edit dialog
- versions section
- usage section
- preview dialog
- activate/delete version workflow
- media action menu

Style B: `dairy`

- route-owned media-detail workflow family under `src/lib/media-detail/*`
- stronger decomposition into:
  - header
  - preview tab
  - file details card
  - renditions section
  - usage tab
  - versions list
  - version dialogs
  - edit dialog helpers

Best proven ingredients:

- outer detail-shell posture from `underlay-reference` / `contact-patch` /
  `composer-admin`
- lower-level workflow module split from `dairy`

Conclusion:

- the likely retained shape is not plain `EntityDetailPage`
- it is a dedicated `MediaDetailWorkflowPage` family that can still compose
  `EntityDetailPage` internally or follow the same shell contract
- `dairy` should be treated as the strongest source for the lower-level section
  decomposition, not as an outlier to flatten away

### Participating apps

- `underlay-reference/acme-admin`
- `acowtancy/dairy`
- `contact-patch/cp-admin`
- `loophole/composer/composer-admin`

### Out of scope

- public/front media surfaces
- media consumers outside admin routes
- storage/backend contract changes unless a repeated UI seam proves they are
  necessary

## Proposed shared shape

Expected retained surfaces:

- no new retained media-root shell by default; prefer the current
  app-local-wrapper-over-`EntityListPage` posture unless the audit proves a real
  repeated seam
- `MediaUploadPage`
- `MediaDetailWorkflowPage`
- existing `EntityTrashPage` for trash

Expected supporting lower-level surfaces:

- reusable upload queue / replace modules if the four apps really share them
- reusable media detail modules for preview, versions, usage, and renditions if
  the current app-local splits prove broadly reusable

Expected non-goals:

- forcing media detail into plain `EntityDetailPage` if that drops usage,
  rendition, or preview workflow behavior
- moving app-specific media business rules into Underlay
- replacing `EntityTrashPage` with a media-specific trash shell

## Execution posture

1. Freeze the retained media-family shape from this audit:
   - media root stays on app-local wrappers over `EntityListPage`
   - media trash stays on `EntityTrashPage`
   - upload and detail become the real new retained shared work
2. Prove the shared media upload shell in `underlay-reference`.
3. Roll the shared upload shell across the other three admin apps.
4. Prove the shared media detail shell in `underlay-reference`.
5. Roll the shared detail shell across the other three admin apps.

## Progress

### Upload proof complete

The first extraction batch is done.

Shared retained surface added in Underlay:

- `MediaUploadPage`

Shared contract updates:

- `docs/contracts/110-admin-template-system.md`
- `docs/usage/templates/000-template-system-overview.md`
- `docs/usage/templates/template-api-reference.md`
- `docs/usage/templates/media-upload-page.md`

Proof coverage:

- `underlay-reference/acme-admin/src/routes/(app)/media/upload/+page.svelte`
- `contact-patch/cp-admin/src/routes/(app)/media/upload/+page.svelte`
- `loophole/composer/composer-admin/src/routes/media/upload/+page.svelte`
- `acowtancy/dairy/src/routes/(app)/media/upload/+page.svelte`

Retained shape proved by the four-app rollout:

- page header and back-link live in the shared shell
- route-level loading and error posture live in the shared shell
- upload queue, replace-mode form fields, duplicate handling, and submit logic
  stay route-owned

Key judgment:

- the right retained seam is the page shell, not the whole upload pipeline
- queue logic and app-local upload business rules still belong in consumers

### Detail proof complete

The second extraction batch is done.

Shared retained surface added in Underlay:

- `MediaDetailWorkflowPage`

Shared contract updates:

- `docs/contracts/110-admin-template-system.md`
- `docs/usage/templates/000-template-system-overview.md`
- `docs/usage/templates/template-api-reference.md`
- `docs/usage/templates/media-detail-workflow-page.md`

Proof coverage:

- `underlay-reference/acme-admin/src/routes/(app)/media/[mediaId]/+page.svelte`
- `contact-patch/cp-admin/src/routes/(app)/media/[mediaId]/+page.svelte`
- `loophole/composer/composer-admin/src/routes/media/[mediaId]/+page.svelte`
- `acowtancy/dairy/src/routes/(app)/media/[id]/+page.svelte`

Retained shape proved by the four-app rollout:

- shared media header, metadata, and deleted banner
- shared loading, error, and retry posture
- shared top-level tab shell
- route-owned preview, versions, usages, renditions, and dialogs preserved

Key judgment:

- the right retained seam is the media-detail workflow shell, not one generic
  lower-level section kit
- Dairy's lower-level split remains the best source for future shared
  extraction pressure if versions/usage/renditions later prove broadly reusable

## Current State

The media admin family is now converged at the retained-shell level:

- media root stays on app-local wrappers over `EntityListPage`
- media trash stays on `EntityTrashPage`
- media upload is on `MediaUploadPage`
- media detail is on `MediaDetailWorkflowPage`

This lane is complete.

## Next Task

Move to `g05.005`: prove the retained `SystemIndexPage` shell.
4. Migrate `contact-patch`, `composer-admin`, and `dairy` onto the retained
   media templates.
5. Update the contracts and usage docs so the media-library system is reference
   grade rather than an implied pattern.

## Consumer Upgrade Impact

Expected.

This lane should introduce or formalize:

- a retained `MediaUploadPage`
- a retained `MediaDetailWorkflowPage`
- any lower-level media workflow modules those two shells need

The media root and trash lanes should mostly stay on the already-proved generic
template shells.

## Next Task

Start the first proof on the upload side.

The audit now supports this narrower first move:

- define `MediaUploadPage`
- prove it in `underlay-reference`
- only then open the heavier `MediaDetailWorkflowPage` extraction
