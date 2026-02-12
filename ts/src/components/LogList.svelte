<script lang="ts" module>
  /**
   * Log entry representing an audit/activity event.
   */
  export interface LogEntry {
    id: string;
    occurredAt: string;
    actor?: LogActor | null;
    action: string;
    resourceType: string;
    resourceId: string;
    resourceLabel?: string;
    details?: Record<string, unknown>;
  }

  /**
   * Actor who performed the action.
   */
  export interface LogActor {
    id: string;
    email?: string;
    name?: string;
  }

  /**
   * Filter configuration for LogList.
   */
  export interface LogFilter {
    field: string;
    label: string;
    type: "select" | "date";
    options?: { value: string; label: string }[];
    placeholder?: string;
  }

  /**
   * Action type categories for styling.
   */
  export type LogActionType =
    | "create"
    | "update"
    | "delete"
    | "restore"
    | "upload"
    | "login"
    | "logout"
    | "security"
    | "other";
</script>

<script lang="ts">
  import LogEntryItem from "./log-list/LogEntryItem.svelte";
  import {
    formatDefaultAction,
    formatDefaultResourceType,
    getActionVariant,
    getDefaultActionType
  } from "./log-list/helpers";
  import LogListPagination from "./log-list/LogListPagination.svelte";
  import LogListStatus from "./log-list/LogListStatus.svelte";
  import LogListToolbar from "./log-list/LogListToolbar.svelte";
  import type { Snippet } from "svelte";

  interface Props {
    /** Log entries to display */
    entries: LogEntry[];
    /** Whether data is loading */
    loading?: boolean;
    /** Error message to display */
    error?: string | null;
    /** Message shown when no entries */
    emptyMessage?: string;

    /** Filter configuration */
    filters?: LogFilter[];
    /** Current filter values (keyed by field) */
    filterValues?: Record<string, string>;
    /** Callback when a filter changes */
    onFilterChange?: (field: string, value: string) => void;
    /** Callback to clear all filters */
    onClearFilters?: () => void;

    /** Current sort field */
    sortField?: string;
    /** Current sort direction */
    sortDirection?: "asc" | "desc";
    /** Callback when sort changes */
    onSortChange?: (field: string, direction: "asc" | "desc") => void;

    /** Current page (1-indexed) */
    page?: number;
    /** Items per page */
    pageSize?: number;
    /** Total number of items */
    total?: number;
    /** Callback when page changes */
    onPageChange?: (page: number) => void;

    /** Callback to refresh data */
    onRefresh?: () => void;
    /** Callback to export data */
    onExport?: () => void;

    /** Custom action icon snippet */
    actionIcon?: Snippet<[LogActionType]>;
    /** Custom entry details snippet */
    entryDetails?: Snippet<[LogEntry]>;
    /** Custom action-to-type mapping */
    getActionType?: (action: string) => LogActionType;
    /** Custom action label formatting */
    formatAction?: (action: string) => string;
    /** Custom resource type label formatting */
    formatResourceType?: (resourceType: string) => string;
    /** Get href for actor link (if provided, actors become links) */
    getActorHref?: (actor: LogActor) => string;
    /** Get href for resource link (if provided, resources become links) */
    getResourceHref?: (resourceType: string, resourceId: string, action: string) => string | null;
  }

  let {
    entries,
    loading = false,
    error = null,
    emptyMessage = "No log entries found",
    filters = [],
    filterValues = {},
    onFilterChange,
    onClearFilters,
    sortField,
    sortDirection = "desc",
    onSortChange,
    page = 1,
    pageSize = 50,
    total,
    onPageChange,
    onRefresh,
    onExport,
    actionIcon,
    entryDetails,
    getActionType: customGetActionType,
    formatAction: customFormatAction,
    formatResourceType: customFormatResourceType,
    getActorHref,
    getResourceHref
  }: Props = $props();

  // Computed values
  const showToolbar = $derived(filters.length > 0 || !!onRefresh || !!onExport);
  const totalPages = $derived(total ? Math.ceil(total / pageSize) : 1);
  const showPagination = $derived(total !== undefined && total > pageSize);

  function getActionType(action: string): LogActionType {
    return (customGetActionType ?? getDefaultActionType)(action);
  }

  // Formatting functions
  function formatAction(action: string): string {
    if (customFormatAction) return customFormatAction(action);
    return formatDefaultAction(action);
  }

  function formatResourceType(resourceType: string): string {
    if (customFormatResourceType) return customFormatResourceType(resourceType);
    return formatDefaultResourceType(resourceType);
  }

  function handlePrevPage() {
    if (page > 1) {
      onPageChange?.(page - 1);
    }
  }

  function handleNextPage() {
    if (page < totalPages) {
      onPageChange?.(page + 1);
    }
  }
</script>

<div class="underlay-log-list" class:underlay-log-list--loading={loading}>
  <!-- Filter bar -->
  {#if showToolbar}
    <LogListToolbar
      {filters}
      {filterValues}
      {loading}
      {onFilterChange}
      {onClearFilters}
      {onRefresh}
      {onExport}
    />
  {/if}

  <!-- Content -->
  <div class="underlay-log-list__content">
    {#if loading || error || entries.length === 0}
      <LogListStatus
        {loading}
        {error}
        entriesCount={entries.length}
        {emptyMessage}
      />
    {:else}
      <ul class="underlay-log-list__entries">
        {#each entries as entry (entry.id)}
          {@const actionType = getActionType(entry.action)}
          {@const actorName = entry.actor?.name ?? entry.actor?.email ?? (entry.actor ? `User ${entry.actor.id.slice(0, 8)}` : "System")}
          <LogEntryItem
            {entry}
            {actionType}
            actionVariant={getActionVariant(actionType)}
            actionLabel={formatAction(entry.action)}
            resourceTypeLabel={formatResourceType(entry.resourceType)}
            {actorName}
            actorHref={entry.actor && getActorHref ? getActorHref(entry.actor) : undefined}
            resourceHref={getResourceHref?.(entry.resourceType, entry.resourceId, entry.action) ?? null}
            {actionIcon}
            {entryDetails}
          />
        {/each}
      </ul>
    {/if}
  </div>

  <!-- Pagination -->
  {#if showPagination}
    <LogListPagination
      {page}
      {pageSize}
      {total}
      {totalPages}
      {loading}
      onPrevPage={handlePrevPage}
      onNextPage={handleNextPage}
    />
  {/if}
</div>

<style>
  .underlay-log-list {
    display: flex;
    flex-direction: column;
    gap: 0;
    background: var(--underlay-color-surface, #1e293b);
    border: 1px solid var(--underlay-color-border-subtle, #334155);
    border-radius: 0.5rem;
    overflow: hidden;
  }

  .underlay-log-list--loading {
    opacity: 0.7;
    pointer-events: none;
  }

  /* Content */
  .underlay-log-list__content {
    min-height: 200px;
  }

  /* Entries */
  .underlay-log-list__entries {
    list-style: none;
    margin: 0;
    padding: 0;
  }

</style>
