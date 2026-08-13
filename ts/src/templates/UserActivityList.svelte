<script lang="ts">
  import {
    Callout,
    Code,
    DataTable,
    PageLoading,
    Pagination,
    Pill as PoodlePill,
    TimeAgo,
    type TableColumn,
    type TableRow,
    type TableRowAction
  } from "@inflatable-cookie/poodle-svelte";
  import type { PillTone } from "@inflatable-cookie/poodle-svelte";
  import { getAuthConfig, useAuthenticatedData } from "../runtime/auth";
  import { copyToClipboard, useToasts } from "../runtime/feedback";
  import {
    type UserActivityItem,
    type UserActivityListLoader
  } from "./template.types";
  import { getUserActivityActionTone } from "./template-types/user-tabs";
  import {
    buildUserTabRows,
    resolveUserTabCount,
    USER_TAB_LIST_FIXED_LIMIT,
    USER_TAB_LIST_PAGED_LIMIT
  } from "./user-tab-list.helpers";

  interface Props {
    userId: string;
    dataLoader: UserActivityListLoader;
    active?: boolean;
    paginated?: boolean;
    getActionTone?: (action: string) => PillTone;
    onCountChange?: (count: number) => void;
  }

  let {
    userId,
    dataLoader,
    active = true,
    paginated = false,
    getActionTone = getUserActivityActionTone,
    onCountChange
  }: Props = $props();

  const toastStore = useToasts();

  const columns: TableColumn[] = [
    { id: "occurredAt", label: "When", width: "110px" },
    { id: "action", label: "Action", width: "140px" },
    { id: "resourceType", label: "Resource", width: "1fr" },
    { id: "resourceId", label: "Resource ID", width: "1.5fr", hideOnMobile: true },
    { id: "actorEmail", label: "Actor", width: "1.5fr", hideOnMobile: true }
  ];

  let hasActivated = $state(false);
  let currentPage = $state(1);
  let lastNotifiedCount = $state<number | null>(null);

  // Lazy-load gate: the first activation latches, so deactivating the tab
  // later never unloads already-fetched data.
  $effect(() => {
    if (active) {
      hasActivated = true;
    }
  });

  const currentLimit = $derived(paginated ? USER_TAB_LIST_PAGED_LIMIT : USER_TAB_LIST_FIXED_LIMIT);

  async function loadActivity(fetch: typeof globalThis.fetch, token: string) {
    return await dataLoader(userId, fetch, token, {
      page: paginated ? currentPage : 1,
      limit: currentLimit
    });
  }

  const listData = useAuthenticatedData(loadActivity, {
    defaultValue: { data: [], total: 0 },
    // Resolve auth getters lazily (configureAuth may run after setup) and hold
    // the auto-fetch until the tab first activates.
    getAuthLoading: () => !hasActivated || (getAuthConfig()?.getAuthLoading?.() ?? false),
    getCurrentUser: () => getAuthConfig()?.getCurrentUser?.() ?? null,
    queryKey: () => JSON.stringify({ page: paginated ? currentPage : 1, limit: currentLimit })
  });

  const entries = $derived(listData.data?.data ?? []);
  const rows = $derived(
    buildUserTabRows(entries, (entry) => ({
      occurredAt: entry.occurredAt,
      action: entry.action,
      resourceType: entry.resourceType,
      resourceId: entry.resourceId ?? "",
      actorEmail: entry.actor?.email ?? ""
    }))
  );
  const totalCount = $derived(resolveUserTabCount(listData.data, entries.length));
  const totalPages = $derived(Math.max(1, Math.ceil(totalCount / currentLimit)));
  const showPagination = $derived(paginated && totalPages > 1);

  $effect(() => {
    if (lastNotifiedCount !== totalCount) {
      lastNotifiedCount = totalCount;
      onCountChange?.(totalCount);
    }
  });

  function getRowActions(row: TableRow<UserActivityItem>): TableRowAction[] {
    const entry = row.data;
    if (!entry) {
      return [];
    }

    return [
      { value: "copy-activity-id", label: "Copy Activity ID" },
      ...(entry.resourceId
        ? [{ value: "copy-resource-id", label: "Copy Resource ID" }]
        : [])
    ];
  }

  function handleRowActionSelect(event: { row: TableRow; action: TableRowAction }) {
    // Poodle's DataTable emits a non-generic TableRow; our row payload is
    // TableRow<UserActivityItem> at runtime.
    const entry = event.row.data as UserActivityItem | undefined;
    if (!entry) {
      return;
    }

    switch (event.action.value) {
      case "copy-activity-id":
        void copyToClipboard(toastStore, entry.id, "Copied activity ID");
        break;
      case "copy-resource-id":
        if (entry.resourceId) {
          void copyToClipboard(toastStore, entry.resourceId, "Copied resource ID");
        }
        break;
    }
  }

  function handlePageChange(page: number) {
    if (page === currentPage) {
      return;
    }
    currentPage = page;
  }
</script>

{#if hasActivated}
  {#if listData.loading && entries.length === 0}
    <PageLoading presentation="inline" message="Loading activity..." />
  {:else if listData.error}
    <Callout tone="danger" message={listData.error} announceMode="polite" />
  {:else}
    <DataTable
      {columns}
      {rows}
      rowActions={getRowActions as unknown as (row: TableRow) => TableRowAction[]}
      emptyMessage="No activity found"
      ariaLabel="User activity"
      onRowActionSelect={handleRowActionSelect}
    >
      {#snippet cell(column, row, value)}
        {@const entry = row.data as UserActivityItem | undefined}
        {#if column.id === "occurredAt" && entry}
          <TimeAgo datetime={entry.occurredAt} tooltipFormat="datetime" short />
        {:else if column.id === "action" && entry}
          <PoodlePill tone={getActionTone(entry.action)} appearance="badge" size="sm">{entry.action}</PoodlePill>
        {:else if column.id === "resourceId" && entry?.resourceId}
          <Code inline inlineVariant="plain" typography="inline" size="md" source={entry.resourceId} showCopyButton={false} />
        {:else}
          {value || "—"}
        {/if}
      {/snippet}
    </DataTable>

    {#if showPagination}
      <div class="underlay-user-activity-list__pagination">
        <Pagination
          {currentPage}
          {totalPages}
          showInfo={false}
          size="sm"
          compact
          variant="simple"
          ariaLabel="User activity pagination"
          onPageChange={handlePageChange}
        />
      </div>
    {/if}
  {/if}
{/if}

<style>
  .underlay-user-activity-list__pagination {
    display: flex;
    justify-content: flex-end;
    padding-top: 0.25rem;
  }
</style>
