# Media Detail Workflow Page

**Status:** Implemented (`g05.004` proof batch 2)

`MediaDetailWorkflowPage` is the retained page shell for repeated admin
media-detail routes.

It owns the repeated outer structure:

- media detail header and back-link
- media metadata bar
- deleted-state banner
- top-level tabs
- loading and error posture

The route still owns:

- app-local media action menus
- API command wiring, auth wiring, and route-specific refetch behavior

For the repeated lower-level media-detail sections, prefer the retained shared
pieces before building route-local modules:

- `MediaFileDetailsCard`
- `MediaEditDialog`
- `MediaPreviewTab`
- `MediaRenditionsSection`
- `MediaVersionActionDialogs`
- `MediaVersionPreviewDialog`
- `MediaVersionsList`
- `MediaUsageList`

For repeated route-side media-detail helper logic, prefer the retained
`@decodelabs/underlay/runtime/media` helpers before adding app-local state or
predicate modules:

- `createMediaEditDialogDraft()`
- `createClosedMediaEditDialogState()`
- `createMediaVersionDialogStateController()`
- `isCurrentMediaVersion()`
- `canActivateMediaVersion()`
- `canDeleteMediaVersion()`
- `canPreviewMediaVersion()`
- `getMediaVersionPreviewUrl()`
- `isImageMedia()`
- `isPdfMedia()`
- `formatFileSize()`

## Usage

```svelte
<script lang="ts">
  import {
    MediaDetailWorkflowPage,
    MediaEditDialog,
    MediaFileDetailsCard,
    MediaPreviewTab,
    MediaRenditionsSection,
    MediaUsageList,
    MediaVersionActionDialogs,
    MediaVersionPreviewDialog,
    MediaVersionsList
  } from "@decodelabs/underlay/templates";
  import {
    canActivateMediaVersion,
    canDeleteMediaVersion,
    canPreviewMediaVersion,
    createClosedMediaEditDialogState,
    createMediaEditDialogDraft,
    createMediaVersionDialogStateController,
    formatFileSize,
    getMediaVersionPreviewUrl,
    isCurrentMediaVersion,
    isImageMedia,
    isPdfMedia
  } from "@decodelabs/underlay/runtime/media";

  let media = $state<MediaDetail | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let activeTab = $state("details");
  let editOpen = $state(false);
  let editError = $state<string | null>(null);
  let editSubmitting = $state(false);
  let editTitle = $state("");
  let editFilename = $state("");
  let editVisibility = $state("public");
  let previewOpen = $state(false);
  let previewVersion = $state<MediaVersion | null>(null);
  const versionDialogs = createMediaVersionDialogStateController<MediaVersion>();

  function openEditDialog() {
    if (!media) return;
    const draft = createMediaEditDialogDraft(media);
    editTitle = draft.title;
    editFilename = draft.filename;
    editVisibility = draft.visibility;
    editError = null;
    editOpen = true;
  }

  function closeEditDialog() {
    const next = createClosedMediaEditDialogState();
    editOpen = next.open;
    editError = next.error;
    editSubmitting = next.submitting;
  }

  function isCurrentVersion(version: MediaVersion) {
    return isCurrentMediaVersion(media?.currentVersionId, version);
  }

  function canActivateVersion(version: MediaVersion) {
    return canActivateMediaVersion(media?.currentVersionId, version);
  }

  function canDeleteVersion(version: MediaVersion) {
    return canDeleteMediaVersion(media?.currentVersionId, version);
  }

  function canPreviewVersion(version: MediaVersion) {
    return media ? canPreviewMediaVersion(media.kind, version) : false;
  }

  function openPreview(version: MediaVersion) {
    if (!canPreviewVersion(version)) return;
    previewVersion = version;
    previewOpen = true;
  }

  const mediaPreviewUrl = $derived.by(() =>
    media?.currentVersion ? getMediaVersionPreviewUrl(media.currentVersion) : null
  );
</script>

{#snippet headerActions(loadedMedia)}
  <MediaActionsMenu
    media={loadedMedia}
    onEditRequest={openEditDialog}
    onSoftDeleteSuccess={() => goto("/media")}
    onRestoreSuccess={reload}
  />
{/snippet}

{#snippet detailsTab(loadedMedia)}
  <MediaFileDetailsCard media={loadedMedia} {formatFileSize} />
  <MediaVersionsList
    versions={versions}
    onUploadNewVersion={openReplaceDialog}
    {canPreviewVersion}
    onOpenVersionPreview={openPreview}
    {formatFileSize}
    {isCurrentVersion}
    {canActivateVersion}
    {canDeleteVersion}
    onRequestActivate={versionDialogs.requestActivate}
    onRequestDelete={versionDialogs.requestDelete}
  />
  <MediaRenditionsSection renditions={loadedMedia.currentVersion?.renditions ?? []} {formatFileSize} />
{/snippet}

{#snippet previewTab(loadedMedia)}
  <MediaPreviewTab
    media={loadedMedia}
    mediaUrl={mediaPreviewUrl}
    isImage={isImageMedia}
    isPdf={isPdfMedia}
  />
{/snippet}

{#snippet usageTab(loadedMedia)}
  <MediaUsageList {usages} />
{/snippet}

<MediaDetailWorkflowPage
  {loading}
  {error}
  item={media}
  onRetry={reload}
  backHref="/media"
  backLabel="Back to media"
  headerActions={headerActions}
  tabs={[
    { id: "details", label: "Details", content: detailsTab },
    { id: "preview", label: "Preview", content: previewTab },
    { id: "usage", label: "Usage", count: usageCount, content: usageTab }
  ]}
  onTabChange={(tabId) => {
    activeTab = tabId;
  }}
  tabsHistoryKey="tab"
  keepMountedTabs
/>

<MediaEditDialog
  bind:open={editOpen}
  error={editError}
  submitting={editSubmitting}
  bind:titleValue={editTitle}
  bind:filenameValue={editFilename}
  bind:visibilityValue={editVisibility}
  visibilityOptions={visibilityOptions}
  onCancel={closeEditDialog}
  onSubmit={submitEdit}
/>

<MediaVersionActionDialogs
  bind:activateDialogOpen={versionDialogs.activateDialogOpen}
  bind:deleteDialogOpen={versionDialogs.deleteDialogOpen}
  selectedVersion={versionDialogs.selectedVersion}
  onConfirmActivate={confirmActivate}
  onCancelActivate={versionDialogs.clear}
  onConfirmDelete={confirmDelete}
  onCancelDelete={versionDialogs.clear}
/>

<MediaVersionPreviewDialog
  bind:open={previewOpen}
  {previewVersion}
  mediaKind={media?.kind ?? ""}
  getPreviewUrl={getMediaVersionPreviewUrl}
  isImage={isImageMedia}
  isPdf={isPdfMedia}
/>
```

## Loading modes

`MediaDetailWorkflowPage` supports two normal loading postures:

- `dataLoader={...}` when the template should own the fetch/load/error shell
- `item` plus `loading` / `error` / `onRetry` when the route already owns the
  media-detail orchestration

Use the second mode when the route already stitches:

- versions and usage fetches
- action-session wiring
- more complex retry/refetch rules

## Props

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `item` | media detail shape | No | Loaded media item |
| `loading` | `boolean` | No | External loading state |
| `error` | `string \| null` | No | External error state |
| `onRetry` | `() => void` | No | Retry callback for external loading mode |
| `dataLoader` | `(fetch, token) => Promise<T \| null>` | No | Template-owned loader |
| `reloadKey` | `string \| number \| null` | No | Force a refetch in loader mode |
| `section` | `string` | No | Header section label. Defaults to `"Media"` |
| `eyebrow` | `string` | No | Optional eyebrow above the title |
| `subtitle` | `string` | No | Optional subtitle |
| `showSubtitleWithBreadcrumbs` | `boolean` | No | Keep subtitle visible with breadcrumbs |
| `headerLevel` | `1 \| 2 \| 3 \| 4 \| 5 \| 6` | No | Heading level |
| `breadcrumbs` | `BreadcrumbItem[]` | No | Optional breadcrumb trail |
| `breadcrumbsMarkLastCurrent` | `boolean` | No | Mark final breadcrumb current |
| `backHref` | `string` | No | Back link URL |
| `backLabel` | `string` | No | Back link label |
| `deletedBannerMessage` | `string` | No | Deleted-state banner copy |
| `deletedBannerTone` | `"warning" \| "info" \| "danger" \| "success"` | No | Deleted-state banner tone |
| `loadingMessage` | `string` | No | Loading copy |
| `errorTitle` | `string` | No | Error callout title |
| `meta` | `DetailMetaItemConfig[]` | No | Extra meta items appended after the standard media items |
| `tabs` | `DetailTabConfig[]` | No | Top-level tab config |
| `content` | `Snippet` | No | Single-surface body when tabs are artificial |
| `onTabChange` | `(tabId: string) => void` | No | Active-tab callback |
| `tabsVariant` | `"underline" \| "card"` | No | Tabs visual variant |
| `tabsSize` | `"sm" \| "md" \| "lg"` | No | Tabs size |
| `tabsHistoryKey` | `string` | No | Optional tab history key |
| `keepMountedTabs` | `boolean` | No | Keep visited tabs mounted |
| `headerActions` | `Snippet` | No | Route-owned header actions |

## What It Provides

- shared media page header
- shared media metadata bar
- shared deleted-state banner
- top-level tab shell
- standard load/error/retry posture

## What You Bring

- media actions menu
- app-specific media command wiring
- app-local media business logic that is genuinely beyond the retained helper surface

Normally you should not bring fresh route-local versions, usage, preview,
rendition, file-details, dialog-state, or preview-predicate modules unless the
retained shared pieces are missing a real seam.

## Use It When

- the route is a normal admin media-detail workflow
- the page shape is still the repeated media header/meta/tab shell
- the route owns media-specific lower-level sections and dialogs

## Do Not Use It When

- the route is not a media-detail workflow
- the page is a broader media console or planner
- the route is really a non-admin asset workflow with a different shape

## See Also

- [Template System Overview](./000-template-system-overview.md)
- [Template API Reference](./template-api-reference.md)
- [Media Upload Page](./media-upload-page.md)
