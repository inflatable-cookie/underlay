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

The route or app-local modules still own:

- versions rendering
- usage rendering
- preview surfaces
- renditions rendering
- edit, restore, delete, activate, and preview dialogs
- app-local media action menus

## Usage

```svelte
<script lang="ts">
  import { MediaDetailWorkflowPage } from "@decodelabs/underlay/templates";

  let media = $state<MediaDetail | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let activeTab = $state("details");
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
  <!-- route-owned versions, renditions, and metadata modules -->
{/snippet}

{#snippet previewTab(loadedMedia)}
  <!-- route-owned preview module -->
{/snippet}

{#snippet usageTab(loadedMedia)}
  <!-- route-owned usage module -->
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
```

## Loading modes

`MediaDetailWorkflowPage` supports two normal loading postures:

- `dataLoader={...}` when the template should own the fetch/load/error shell
- `item` plus `loading` / `error` / `onRetry` when the route already owns the
  media-detail orchestration

Use the second mode when the route already stitches:

- versions and usage fetches
- action-session wiring
- local helper modules
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

- media versions module
- media usage module
- media preview module
- rendition module
- media actions menu
- edit and version dialogs
- app-local media business logic

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
