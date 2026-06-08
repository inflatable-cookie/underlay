<script lang="ts">
  import { IconButton } from "@poodle/svelte";
  import type { QueryParams } from "../client/query";
  import { copyToClipboard, useToasts } from "../runtime/feedback";
  import { MediaKind, MediaVisibility } from "../runtime/media";
  import { default as EntityListPage } from "./EntityListPage.svelte";
  import { default as MediaListCard } from "./MediaListCard.svelte";
  import type {
    BatchActionConfig,
    EntityListDataLoader,
    FilterConfig,
    MediaListPageItem
  } from "./template.types";

  interface Props<TMedia extends MediaListPageItem = MediaListPageItem> {
    section?: string;
    title?: string;
    hideTitle?: boolean;
    subtitle?: string;
    eyebrow?: string;
    headerLevel?: 1 | 2 | 3 | 4 | 5 | 6;
    backHref?: string;
    backLabel?: string;
    dataLoader: EntityListDataLoader<TMedia>;
    query?: QueryParams;
    onQueryChange?: (query: QueryParams) => void;
    onUpload?: () => void;
    onViewTrash?: () => void;
    onDeleteMedia?: (mediaId: string) => Promise<void>;
    onBatchDelete?: (ids: string[]) => Promise<number | { deleted?: number } | void>;
    onDataChange?: () => void;
    searchFilterId?: string;
    mediaKindOptions?: Array<{ value: string; label: string }>;
    filters?: FilterConfig[];
    listHref?: string;
    detailHref?: (media: TMedia) => string;
    contextLabel?: string;
    filterToolbarCollapsed?: boolean;
  }

  let {
    section,
    title = "Media Library",
    hideTitle = false,
    subtitle,
    eyebrow,
    headerLevel = 2,
    backHref = "/",
    backLabel = "Back to dashboard",
    dataLoader,
    query,
    onQueryChange,
    onUpload,
    onViewTrash,
    onDeleteMedia,
    onBatchDelete,
    onDataChange,
    searchFilterId = "title",
    mediaKindOptions = [
      { value: "All", label: "All types" },
      { value: MediaKind.Image, label: "Image" },
      { value: MediaKind.Pdf, label: "PDF" },
      { value: MediaKind.Document, label: "Document" },
      { value: MediaKind.Video, label: "Video" },
      { value: MediaKind.Audio, label: "Audio" }
    ],
    filters = defaultFilters(searchFilterId, mediaKindOptions),
    listHref = "/media",
    detailHref,
    contextLabel = "Media",
    filterToolbarCollapsed = true
  }: Props = $props();

  const toastStore = useToasts();
  const batchActions = $derived<BatchActionConfig[]>(
    onBatchDelete
      ? [
          {
            id: "delete",
            label: "Delete",
            tone: "danger",
            icon: "trash-2",
            confirm: {
              title: "Move media to trash",
              description: (count: number) =>
                `Are you sure you want to move ${count} media ${count === 1 ? "item" : "items"} to trash?`,
              confirmLabel: "Move to trash",
              cancelLabel: "Keep media"
            },
            handler: async (ids: string[]) => {
              const result = await onBatchDelete?.(ids);
              const deleted =
                typeof result === "number"
                  ? result
                  : typeof result?.deleted === "number"
                    ? result.deleted
                    : ids.length;
              toastStore.push({
                variant: "success",
                message: `Moved ${deleted} ${deleted === 1 ? "item" : "items"} to trash`
              });
            }
          }
        ]
      : []
  );

  function defaultFilters(
    titleFilterId: string,
    kindOptions: Array<{ value: string; label: string }>
  ): FilterConfig[] {
    return [
      {
        id: titleFilterId,
        type: "search",
        label: "Title",
        placeholder: "Search by title..."
      },
      {
        id: "kind",
        type: "select",
        label: "Type",
        options: kindOptions
      },
      {
        id: "visibility",
        type: "select",
        label: "Visibility",
        options: [
          { value: "All", label: "All visibility" },
          { value: MediaVisibility.Public, label: "Public" },
          { value: MediaVisibility.Restricted, label: "Restricted" }
        ]
      },
      {
        id: "sort",
        type: "sort",
        label: "Sort",
        sortFields: [
          { key: "title", label: "Title" },
          { key: "kind", label: "Kind" },
          { key: "updatedAt", label: "Updated", defaultDirection: "desc" },
          { key: "createdAt", label: "Created", defaultDirection: "desc" }
        ]
      }
    ];
  }

  async function handleCopyId(mediaId: string): Promise<void> {
    await copyToClipboard(toastStore, mediaId, "Copied media ID");
  }

  async function handleDeleteMedia(mediaId: string, refetch: () => Promise<void>): Promise<void> {
    if (!onDeleteMedia) return;

    try {
      await onDeleteMedia(mediaId);
      toastStore.push({ variant: "success", message: "Media moved to trash" });
      onDataChange?.();
      await refetch();
    } catch (error) {
      const message = error instanceof Error ? error.message : "Failed to move media to trash";
      toastStore.push({ variant: "error", message });
    }
  }
</script>

{#snippet headerLeadingActions(ctx: { selectionMode: boolean; reorderMode: boolean; actionSize?: "xs" | "sm" })}
  {#if onViewTrash}
    <IconButton
      type="button"
      variant="secondary"
      tone="danger"
      size={ctx.actionSize}
      icon="trash-2"
      ariaLabel="View trash"
      tooltip="View trash"
      disabled={ctx.selectionMode || ctx.reorderMode}
      onClick={onViewTrash}
    />
  {/if}
{/snippet}

{#snippet renderItem(media: MediaListPageItem, ctx: { selectionMode: boolean; reorderMode: boolean; selected: boolean; onToggle: (selected: boolean) => void; refetch: () => Promise<void> })}
  <MediaListCard
    {media}
    {listHref}
    detailHref={detailHref as never}
    {contextLabel}
    reorderMode={ctx.reorderMode}
    selectionMode={ctx.selectionMode}
    selected={ctx.selected}
    onSelectionChange={(_id, selected) => ctx.onToggle(selected)}
    onDelete={ctx.selectionMode || ctx.reorderMode || !onDeleteMedia
      ? undefined
      : (id) => void handleDeleteMedia(id, ctx.refetch)}
    onCopyId={ctx.selectionMode || ctx.reorderMode ? undefined : handleCopyId}
  />
{/snippet}

<EntityListPage
  {section}
  {title}
  {hideTitle}
  {subtitle}
  {eyebrow}
  {headerLevel}
  {backHref}
  {backLabel}
  {dataLoader}
  presentation="cards"
  renderItem={renderItem as never}
  {filters}
  {query}
  batchActions={batchActions}
  {onQueryChange}
  {onDataChange}
  onAdd={onUpload}
  addLabel="Upload media"
  headerLeadingActions={headerLeadingActions as never}
  {filterToolbarCollapsed}
/>
