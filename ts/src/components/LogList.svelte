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
  import Button from "./Button.svelte";
  import LogEntryItem from "./log-list/LogEntryItem.svelte";
  import Select from "./Select.svelte";
  import TextInput from "./TextInput.svelte";
  import Activity from "lucide-svelte/icons/activity";
  import ChevronLeft from "lucide-svelte/icons/chevron-left";
  import ChevronRight from "lucide-svelte/icons/chevron-right";
  import Download from "lucide-svelte/icons/download";
  import RefreshCw from "lucide-svelte/icons/refresh-cw";
  import X from "lucide-svelte/icons/x";
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
  const hasFilters = $derived(filters.length > 0);
  const hasActiveFilters = $derived(
    Object.values(filterValues).some((v) => v && v.trim() !== "")
  );
  const totalPages = $derived(total ? Math.ceil(total / pageSize) : 1);
  const showPagination = $derived(total !== undefined && total > pageSize);

  // Default action type mapping
  function defaultGetActionType(action: string): LogActionType {
    const normalized = action.toLowerCase();
    if (normalized.includes("create")) return "create";
    if (normalized.includes("update") || normalized.includes("edit")) return "update";
    if (normalized.includes("delete") || normalized.includes("remove")) return "delete";
    if (normalized.includes("restore") || normalized.includes("recover")) return "restore";
    if (normalized.includes("upload")) return "upload";
    if (normalized === "login" || normalized === "sign_in") return "login";
    if (normalized === "logout" || normalized === "sign_out") return "logout";
    if (
      normalized.includes("role") ||
      normalized.includes("suspend") ||
      normalized.includes("permission")
    )
      return "security";
    return "other";
  }

  function getActionType(action: string): LogActionType {
    return (customGetActionType ?? defaultGetActionType)(action);
  }

  // Badge variant mapping
  type BadgeVariant = "default" | "success" | "warning" | "danger" | "info" | "muted";

  function getActionVariant(actionType: LogActionType): BadgeVariant {
    switch (actionType) {
      case "create":
      case "restore":
        return "success";
      case "delete":
        return "danger";
      case "update":
      case "upload":
        return "info";
      case "login":
      case "logout":
        return "muted";
      case "security":
        return "warning";
      default:
        return "default";
    }
  }

  // Formatting functions
  function formatAction(action: string): string {
    if (customFormatAction) return customFormatAction(action);
    return action.replace(/_/g, " ");
  }

  function formatResourceType(resourceType: string): string {
    if (customFormatResourceType) return customFormatResourceType(resourceType);
    return resourceType.replace(/_/g, " ");
  }

  // Event handlers
  function handleFilterChange(field: string, value: string) {
    onFilterChange?.(field, value);
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

<div class="log-list" class:log-list--loading={loading}>
  <!-- Filter bar -->
  {#if hasFilters || onRefresh || onExport}
    <div class="log-list__toolbar">
      {#if hasFilters}
        <div class="log-list__filters">
          {#each filters as filter}
            <div class="log-list__filter">
              <label class="log-list__filter-label" for="filter-{filter.field}">
                {filter.label}
              </label>
              {#if filter.type === "select" && filter.options}
                <Select
                  id="filter-{filter.field}"
                  name={filter.field}
                  items={[
                    { value: "", label: filter.placeholder ?? "All" },
                    ...filter.options
                  ]}
                  value={filterValues[filter.field] ?? ""}
                  onchange={(value) => handleFilterChange(filter.field, value)}
                />
              {:else if filter.type === "date"}
                <TextInput
                  id="filter-{filter.field}"
                  type="date"
                  value={filterValues[filter.field] ?? ""}
                  onchange={(value) => handleFilterChange(filter.field, value)}
                />
              {/if}
            </div>
          {/each}
          {#if hasActiveFilters && onClearFilters}
            <Button variant="subtle" size="sm" onclick={onClearFilters}>
              <X size={14} />
              Clear
            </Button>
          {/if}
        </div>
      {/if}

      <div class="log-list__actions">
        {#if onRefresh}
          <Button
            variant="subtle"
            size="sm"
            onclick={onRefresh}
            disabled={loading}
            title="Refresh"
          >
            <RefreshCw size={14} class={loading ? "spinning" : ""} />
          </Button>
        {/if}
        {#if onExport}
          <Button variant="subtle" size="sm" onclick={onExport} disabled={loading}>
            <Download size={14} />
            Export
          </Button>
        {/if}
      </div>
    </div>
  {/if}

  <!-- Content -->
  <div class="log-list__content">
    {#if loading && entries.length === 0}
      <div class="log-list__status">
        <Activity size={24} class="log-list__status-icon spinning" />
        <p>Loading log entries...</p>
      </div>
    {:else if error}
      <div class="log-list__status log-list__status--error">
        <p>{error}</p>
      </div>
    {:else if entries.length === 0}
      <div class="log-list__status">
        <Activity size={24} class="log-list__status-icon" />
        <p>{emptyMessage}</p>
      </div>
    {:else}
      <ul class="log-list__entries">
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
    <div class="log-list__pagination">
      <span class="log-list__pagination-info">
        {#if total}
          Showing {(page - 1) * pageSize + 1}–{Math.min(page * pageSize, total)} of {total}
        {/if}
      </span>
      <div class="log-list__pagination-controls">
        <Button
          variant="subtle"
          size="sm"
          onclick={handlePrevPage}
          disabled={page <= 1 || loading}
        >
          <ChevronLeft size={16} />
        </Button>
        <span class="log-list__pagination-page">
          Page {page} of {totalPages}
        </span>
        <Button
          variant="subtle"
          size="sm"
          onclick={handleNextPage}
          disabled={page >= totalPages || loading}
        >
          <ChevronRight size={16} />
        </Button>
      </div>
    </div>
  {/if}
</div>

<style>
  .log-list {
    display: flex;
    flex-direction: column;
    gap: 0;
    background: var(--underlay-color-surface, #1e293b);
    border: 1px solid var(--underlay-color-border-subtle, #334155);
    border-radius: 0.5rem;
    overflow: hidden;
  }

  .log-list--loading {
    opacity: 0.7;
    pointer-events: none;
  }

  /* Toolbar */
  .log-list__toolbar {
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.75rem 1rem;
    background: var(--underlay-color-surface-raised, #283548);
    border-bottom: 1px solid var(--underlay-color-border-subtle, #334155);
    flex-wrap: wrap;
  }

  .log-list__filters {
    display: flex;
    align-items: flex-end;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  .log-list__filter {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    min-width: 120px;
  }

  .log-list__filter-label {
    font-size: 0.7rem;
    font-weight: 500;
    color: var(--underlay-color-text-muted, #94a3b8);
    text-transform: uppercase;
    letter-spacing: 0.025em;
  }

  /* Ensure date inputs have consistent min-width */
  .log-list__filter :global(.underlay-input[type="date"]) {
    min-width: 130px;
  }

  .log-list__actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-left: auto;
  }

  /* Content */
  .log-list__content {
    min-height: 200px;
  }

  .log-list__status {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    padding: 3rem 1rem;
    color: var(--underlay-color-text-muted, #94a3b8);
  }

  .log-list__status p {
    margin: 0;
    font-size: 0.875rem;
  }

  .log-list__status--error {
    color: var(--underlay-color-danger, #ef4444);
  }

  :global(.log-list__status-icon) {
    opacity: 0.5;
  }

  :global(.spinning) {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  /* Entries */
  .log-list__entries {
    list-style: none;
    margin: 0;
    padding: 0;
  }

  /* Pagination */
  .log-list__pagination {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.75rem 1rem;
    background: var(--underlay-color-surface-raised, #283548);
    border-top: 1px solid var(--underlay-color-border-subtle, #334155);
    font-size: 0.8125rem;
  }

  .log-list__pagination-info {
    color: var(--underlay-color-text-muted, #94a3b8);
  }

  .log-list__pagination-controls {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .log-list__pagination-page {
    color: var(--underlay-color-text-secondary, #cbd5e1);
    min-width: 100px;
    text-align: center;
  }
</style>
