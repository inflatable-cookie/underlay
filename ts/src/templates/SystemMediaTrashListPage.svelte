<script lang="ts">
  import { getAuthConfig } from "../runtime/auth";
  import { useToasts } from "../runtime/feedback";
  import { default as EntityListPage } from "./EntityListPage.svelte";
  import type { QueryParams } from "../client/query";
  import type {
    FilterConfig,
    SystemMediaTrashAction,
    SystemMediaTrashItem,
    SystemMediaTrashListLoader
  } from "./template.types";
  import { default as SystemMediaTrashListCard } from "./SystemMediaTrashListCard.svelte";

  interface Props {
    title?: string;
    backHref?: string;
    backLabel?: string;
    dataLoader: SystemMediaTrashListLoader;
    restoreAction: SystemMediaTrashAction;
    purgeAction: SystemMediaTrashAction;
    getMediaHref?: (media: SystemMediaTrashItem) => string | null;
    onMediaClick?: (media: SystemMediaTrashItem) => void;
    restoreSuccessMessage?: string;
    purgeSuccessMessage?: string;
    enableFilters?: boolean;
    query: QueryParams;
    onQueryChange: (query: QueryParams) => void;
    searchFilterId?: string;
  }

  let {
    title = "Media Trash",
    backHref = "/media",
    backLabel = "Back to media",
    dataLoader,
    restoreAction,
    purgeAction,
    getMediaHref = defaultMediaHref,
    onMediaClick,
    restoreSuccessMessage = "Media restored",
    purgeSuccessMessage = "Media permanently deleted",
    enableFilters = false,
    query,
    onQueryChange,
    searchFilterId = "search"
  }: Props = $props();

  const toastStore = useToasts();
  const authConfig = getAuthConfig();

  const filters = $derived.by((): FilterConfig[] =>
    enableFilters
      ? [
          {
            id: searchFilterId,
            type: "search",
            label: "Search",
            placeholder: "Search deleted media"
          },
          {
            id: "sort",
            type: "sort",
            label: "Sort",
            sortFields: [
              { key: "deletedAt", label: "Deleted", defaultDirection: "desc" },
              { key: "title", label: "Title" },
              { key: "kind", label: "Kind" }
            ]
          }
        ]
      : []
  );

  async function loadMedia(fetch: typeof globalThis.fetch, token: string | null, nextQuery: QueryParams) {
    if (!token) throw new Error("Not authenticated");
    return await dataLoader(fetch, token, nextQuery);
  }

  async function runAction(
    media: SystemMediaTrashItem,
    action: SystemMediaTrashAction,
    successMessage: string,
    failureMessage: string
  ): Promise<void> {
    const token = authConfig?.getToken?.() ?? null;
    if (!token) {
      toastStore.push({ variant: "error", message: "Not authenticated" });
      return;
    }

    try {
      await action(media, fetch, token);
      toastStore.push({ variant: "success", message: successMessage });
    } catch (error) {
      const message = error instanceof Error ? error.message : failureMessage;
      toastStore.push({ variant: "error", message });
      throw error;
    }
  }

  function defaultMediaHref(media: SystemMediaTrashItem): string {
    return `/media/${media.id}`;
  }
</script>

{#snippet renderItem(item, ctx)}
  <SystemMediaTrashListCard
    media={item}
    href={onMediaClick ? null : getMediaHref(item)}
    onClick={onMediaClick ? () => onMediaClick(item) : undefined}
    onRestore={async (media) => {
      await runAction(media, restoreAction, restoreSuccessMessage, "Failed to restore media");
      await ctx.refetch();
    }}
    onPurge={async (media) => {
      await runAction(media, purgeAction, purgeSuccessMessage, "Failed to delete media");
      await ctx.refetch();
    }}
  />
{/snippet}

<EntityListPage
  {title}
  {backHref}
  {backLabel}
  dataLoader={loadMedia}
  presentation="cards"
  {renderItem}
  {query}
  {onQueryChange}
  filters={filters.length > 0 ? filters : undefined}
/>
