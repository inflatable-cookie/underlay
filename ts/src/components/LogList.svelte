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
  import Badge from "./Badge.svelte";
  import Button from "./Button.svelte";
  import Select from "./Select.svelte";
  import TextInput from "./TextInput.svelte";
  import { formatRelative } from "../patterns/i18n";
  import User from "lucide-svelte/icons/user";
  import Plus from "lucide-svelte/icons/plus";
  import Pencil from "lucide-svelte/icons/pencil";
  import Trash2 from "lucide-svelte/icons/trash-2";
  import RotateCcw from "lucide-svelte/icons/rotate-ccw";
  import Upload from "lucide-svelte/icons/upload";
  import LogIn from "lucide-svelte/icons/log-in";
  import LogOut from "lucide-svelte/icons/log-out";
  import Shield from "lucide-svelte/icons/shield";
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
    formatResourceType: customFormatResourceType
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
          <li class="log-entry">
            <div class="log-entry__icon log-entry__icon--{actionType}">
              {#if actionIcon}
                {@render actionIcon(actionType)}
              {:else if actionType === "create"}
                <Plus size={14} />
              {:else if actionType === "update"}
                <Pencil size={14} />
              {:else if actionType === "delete"}
                <Trash2 size={14} />
              {:else if actionType === "restore"}
                <RotateCcw size={14} />
              {:else if actionType === "upload"}
                <Upload size={14} />
              {:else if actionType === "login"}
                <LogIn size={14} />
              {:else if actionType === "logout"}
                <LogOut size={14} />
              {:else if actionType === "security"}
                <Shield size={14} />
              {:else}
                <User size={14} />
              {/if}
            </div>

            <div class="log-entry__body">
              <div class="log-entry__main">
                <span class="log-entry__actor">
                  {#if entry.actor}
                    {entry.actor.name ?? entry.actor.email ?? `User ${entry.actor.id.slice(0, 8)}`}
                  {:else}
                    System
                  {/if}
                </span>
                <Badge variant={getActionVariant(actionType)} size="sm">
                  {formatAction(entry.action)}
                </Badge>
                <span class="log-entry__resource-type">
                  {formatResourceType(entry.resourceType)}
                </span>
                {#if entry.resourceLabel}
                  <span class="log-entry__resource-label">"{entry.resourceLabel}"</span>
                {/if}
              </div>

              {#if entryDetails}
                <div class="log-entry__details">
                  {@render entryDetails(entry)}
                </div>
              {/if}

              <time class="log-entry__time" datetime={entry.occurredAt}>
                {formatRelative(entry.occurredAt)}
              </time>
            </div>
          </li>
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

  .log-entry {
    display: flex;
    gap: 0.75rem;
    padding: 0.875rem 1rem;
    border-bottom: 1px solid var(--underlay-color-border-subtle, #334155);
  }

  .log-entry:last-child {
    border-bottom: none;
  }

  .log-entry:hover {
    background: var(--underlay-color-surface-hover, rgba(255, 255, 255, 0.02));
  }

  /* Entry icon */
  .log-entry__icon {
    width: 1.75rem;
    height: 1.75rem;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    background: var(--underlay-color-surface-raised, #283548);
    color: var(--underlay-color-text-muted, #94a3b8);
  }

  .log-entry__icon--create {
    background: rgba(34, 197, 94, 0.15);
    color: #22c55e;
  }

  .log-entry__icon--update {
    background: rgba(59, 130, 246, 0.15);
    color: #3b82f6;
  }

  .log-entry__icon--delete {
    background: rgba(239, 68, 68, 0.15);
    color: #ef4444;
  }

  .log-entry__icon--restore {
    background: rgba(34, 197, 94, 0.15);
    color: #22c55e;
  }

  .log-entry__icon--upload {
    background: rgba(168, 85, 247, 0.15);
    color: #a855f7;
  }

  .log-entry__icon--login,
  .log-entry__icon--logout {
    background: rgba(148, 163, 184, 0.15);
    color: #94a3b8;
  }

  .log-entry__icon--security {
    background: rgba(245, 158, 11, 0.15);
    color: #f59e0b;
  }

  /* Entry body */
  .log-entry__body {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .log-entry__main {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    flex-wrap: wrap;
    font-size: 0.875rem;
    line-height: 1.4;
  }

  .log-entry__actor {
    font-weight: 500;
    color: var(--underlay-color-text, #f1f5f9);
  }

  .log-entry__resource-type {
    color: var(--underlay-color-text-muted, #94a3b8);
  }

  .log-entry__resource-label {
    color: var(--underlay-color-text-secondary, #cbd5e1);
    font-style: italic;
  }

  .log-entry__details {
    font-size: 0.8125rem;
    color: var(--underlay-color-text-muted, #94a3b8);
  }

  .log-entry__time {
    font-size: 0.75rem;
    color: var(--underlay-color-text-muted, #64748b);
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
