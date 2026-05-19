<script lang="ts">
  import { getAuthConfig, useAuthenticatedData } from "../runtime/auth";
  import { useToasts } from "../runtime/feedback";
  import type { QueryParams, SortField } from "../client/query";
  import {
    buildQueryString,
    parseQueryParams
  } from "../client/query";
  import { default as EntityTrashPage } from "./EntityTrashPage.svelte";
  import { default as SystemMediaTrashListCard } from "./SystemMediaTrashListCard.svelte";
  import {
    FilterToolbar,
    OrderBy,
    TextInput,
    type OrderByValue,
    type SortField as PoodleSortField
  } from "@poodle/svelte";
  import type {
    SystemMediaTrashAction,
    SystemMediaTrashItem,
    SystemMediaTrashListLoader
  } from "./template.types";

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
    query?: QueryParams;
    onQueryChange?: (query: QueryParams) => void;
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
  const fallbackQuery = $state<QueryParams>({ page: 1 });
  const activeQuery = $derived(query ?? fallbackQuery);
  const searchValue = $derived(getFilterValue(activeQuery, searchFilterId) ?? "");
  const orderByValue = $derived(
    (activeQuery.sort ?? []).map((field) => ({
      key: field.field,
      direction: field.direction
    })) as OrderByValue
  );
  const sortFields: PoodleSortField[] = [
    { key: "deletedAt", label: "Deleted", defaultDirection: "desc" },
    { key: "title", label: "Title" },
    { key: "kind", label: "Kind" }
  ];

  const pageData = useAuthenticatedData(
    async (fetch, token) => {
      const media = await dataLoader(fetch, token);
      return { media };
    },
    {
      defaultValue: {
        media: {
          data: [],
          total: 0,
          hasMore: false
        }
      }
    }
  );
  const visibleItems = $derived.by(() => {
    const items = pageData.data?.media.data ?? [];
    if (!enableFilters) return items;

    const searchTerm = getFilterValue(activeQuery, searchFilterId) ?? "";
    return items
      .filter((item) => matchesSearch(item, searchTerm))
      .sort((a, b) => compareMedia(a, b, activeQuery.sort ?? []));
  });

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
      await pageData.refetch();
    } catch (error) {
      const message = error instanceof Error ? error.message : failureMessage;
      toastStore.push({ variant: "error", message });
    }
  }

  function defaultMediaHref(media: SystemMediaTrashItem): string {
    return `/media/${media.id}`;
  }

  function getFilterValue(query: QueryParams, field: string): string | undefined {
    const filter = query.filters?.find((entry) => entry.field === field);
    if (!filter || filter.value === "" || filter.value === "All") {
      return undefined;
    }
    return filter.value;
  }

  function setQuery(nextQuery: QueryParams): void {
    if (onQueryChange) {
      onQueryChange(nextQuery);
      return;
    }

    const url = new URL(globalThis.location.href);
    url.search = buildQueryString(nextQuery);
    globalThis.history.replaceState(globalThis.history.state, "", url);
    Object.assign(fallbackQuery, parseQueryParams(url.searchParams));
  }

  function updateSearch(nextValue: string): void {
    const nextFilters = (activeQuery.filters ?? []).filter(
      (entry) => entry.field !== searchFilterId
    );
    if (nextValue.trim() !== "") {
      nextFilters.push({ field: searchFilterId, operator: "eq", value: nextValue });
    }
    setQuery({
      ...activeQuery,
      page: 1,
      filters: nextFilters
    });
  }

  function updateSort(nextValue: OrderByValue): void {
    setQuery({
      ...activeQuery,
      page: 1,
      sort: nextValue.map((entry) => ({
        field: entry.key,
        direction: entry.direction
      }))
    });
  }

  function matchesSearch(item: SystemMediaTrashItem, searchTerm: string): boolean {
    const needle = searchTerm.trim().toLowerCase();
    if (!needle) return true;

    return [
      item.title ?? "",
      item.originalFilename ?? "",
      item.kind,
      item.id
    ].some((value) => value.toLowerCase().includes(needle));
  }

  function compareMedia(
    a: SystemMediaTrashItem,
    b: SystemMediaTrashItem,
    sort: SortField[]
  ): number {
    const effectiveSort =
      sort.length > 0
        ? sort
        : [{ field: "deletedAt", direction: "desc" as const }];

    for (const field of effectiveSort) {
      let comparison = 0;

      switch (field.field) {
        case "deletedAt":
          comparison = (a.deletedAt ?? "").localeCompare(b.deletedAt ?? "");
          break;
        case "title":
          comparison = (a.title ?? a.originalFilename ?? "").localeCompare(
            b.title ?? b.originalFilename ?? ""
          );
          break;
        case "kind":
          comparison = a.kind.localeCompare(b.kind);
          break;
        default:
          comparison = 0;
      }

      if (comparison !== 0) {
        return field.direction === "desc" ? -comparison : comparison;
      }
    }

    return 0;
  }
</script>

{#snippet renderItem(item)}
  <SystemMediaTrashListCard
    media={item}
    href={onMediaClick ? null : getMediaHref(item)}
    onClick={onMediaClick ? () => onMediaClick(item) : undefined}
    onRestore={(media) => runAction(media, restoreAction, restoreSuccessMessage, "Failed to restore media")}
    onPurge={(media) => runAction(media, purgeAction, purgeSuccessMessage, "Failed to delete media")}
  />
{/snippet}

{#snippet trashFilters()}
  {#if enableFilters}
    <FilterToolbar ariaLabel="Media trash filters" summaryText="Filters" collapsible>
      <TextInput
        id="media-trash-search"
        type="search"
        placeholder="Search deleted media"
        ariaLabel="Search deleted media"
        value={searchValue}
        onValueChange={updateSearch}
        onClear={() => updateSearch("")}
      />
      <OrderBy
        fields={sortFields}
        value={orderByValue}
        onChange={updateSort}
        compact
      />
    </FilterToolbar>
  {/if}
{/snippet}

<EntityTrashPage
  {title}
  {backHref}
  {backLabel}
  loading={pageData.loading}
  loadingMessage="Loading trash..."
  error={pageData.error}
  statusMessage="Items in trash can be restored or permanently deleted. Permanently deleted items cannot be recovered."
  statusTone="warning"
  beforeItems={enableFilters ? trashFilters : undefined}
  items={visibleItems}
  renderItem={renderItem}
  emptyTitle="Trash is empty"
  emptyMessage="Deleted media items will appear here."
/>
