<script lang="ts">
  import { default as EntityListPage } from "./EntityListPage.svelte";
  import type { QueryParams } from "../client/query";
  import type {
    FilterConfig,
    SystemAuditLogEntry,
    SystemAuditLogListLoader
  } from "./template.types";
  import type { LogActor, LogEntry } from "@inflatable-cookie/poodle-svelte";
  import type { LogActorHrefResolver } from "./template.types";

  interface Props {
    title?: string;
    subtitle?: string;
    backHref?: string;
    backLabel?: string;
    dataLoader: SystemAuditLogListLoader;
    actionOptions?: FilterConfig["options"];
    resourceOptions?: FilterConfig["options"];
    resourceFilterType?: "select" | "search";
    query: QueryParams;
    onQueryChange: (query: QueryParams) => void;
    getActorHref?: LogActorHrefResolver;
    getResourceHref?: (resourceType: string, resourceId: string, action: string) => string | null;
    formatAction?: (action: string) => string;
    formatResourceType?: (resourceType: string) => string;
  }

  const defaultActionOptions = [
    { value: "All", label: "All actions" },
    { value: "create", label: "Create" },
    { value: "update", label: "Update" },
    { value: "delete", label: "Delete" },
    { value: "restore", label: "Restore" }
  ];

  let {
    title = "Audit Log",
    subtitle,
    backHref = "/system",
    backLabel = "Back to system",
    dataLoader,
    actionOptions = defaultActionOptions,
    resourceOptions = [],
    resourceFilterType = resourceOptions.length > 0 ? "select" : "search",
    query,
    onQueryChange,
    getActorHref,
    getResourceHref,
    formatAction = defaultFormatLabel,
    formatResourceType = defaultFormatLabel
  }: Props = $props();

  const filters = $derived.by((): FilterConfig[] => [
    {
      id: "action",
      type: "select",
      label: "Action",
      options: actionOptions
    },
    resourceFilterType === "search"
      ? {
          id: "resourceType",
          type: "search",
          label: "Resource",
          placeholder: "Filter by resource type"
        }
      : {
          id: "resourceType",
          type: "select",
          label: "Resource",
          options: resourceOptions
        }
  ]);

  function getFilterValue(nextQuery: QueryParams, field: string): string | undefined {
    const filter = nextQuery.filters?.find((entry) => entry.field === field);
    if (!filter || filter.value === "" || filter.value === "All") {
      return undefined;
    }
    return filter.value;
  }

  async function loadAuditEntries(fetch: typeof globalThis.fetch, token: string | null, nextQuery: QueryParams) {
    if (!token) throw new Error("Not authenticated");

    return await dataLoader(fetch, token, {
      action: getFilterValue(nextQuery, "action"),
      resourceType: getFilterValue(nextQuery, "resourceType"),
      page: nextQuery.page ?? 1,
      limit: nextQuery.limit ?? 30
    });
  }

  function toLogEntries(entries: SystemAuditLogEntry[]): LogEntry[] {
    return entries.map((entry) => {
      const logEntry: LogEntry = {
        id: entry.id,
        occurredAt: entry.occurredAt,
        actor: entry.actor
        ? {
            id: entry.actor.id,
            email: entry.actor.email ?? undefined,
            name: entry.actor.name ?? undefined
          }
        : null,
        action: entry.action,
        resourceType: entry.resourceType,
        resourceId: entry.resourceId,
        resourceLabel: entry.resourceLabel ?? undefined
      };

      if (isRecord(entry.details)) {
        logEntry.details = entry.details;
      }

      return logEntry;
    });
  }

  function isRecord(value: unknown): value is Record<string, unknown> {
    return Boolean(value) && typeof value === "object" && !Array.isArray(value);
  }

  function defaultFormatLabel(value: string): string {
    return value
      .replace(/_/g, " ")
      .replace(/\b\w/g, (char) => char.toUpperCase());
  }
</script>

<EntityListPage
  {title}
  {subtitle}
  {backHref}
  {backLabel}
  dataLoader={loadAuditEntries}
  presentation="log"
  {filters}
  {query}
  {onQueryChange}
  {toLogEntries}
  {getActorHref}
  {getResourceHref}
  {formatAction}
  {formatResourceType}
/>
