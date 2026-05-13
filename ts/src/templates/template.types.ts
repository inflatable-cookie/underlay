import type { Snippet } from "svelte";
import type {
  LogActionType,
  LogActor,
  LogEntry,
  TableColumn,
  TableRow,
  TableRowAction
} from "@poodle/svelte";
import type { QueryParams, SortDirection } from "../client/query";
import type { ReorderController, ReorderableItem } from "../patterns/reorder-controller.svelte";

// Cross-package Svelte Snippet identity is brittle in linked local workspaces.
// Keep the shared template boundary permissive so consumers can pass local snippets cleanly.
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type TemplateSurface = Snippet | ((...args: any[]) => any);

export interface TemplateFilterOption {
  value: string;
  label: string;
}

export interface TemplateSortField {
  key: string;
  label: string;
  defaultDirection?: SortDirection;
}

export interface FilterConfig {
  id: string;
  type: "search" | "select" | "date" | "number" | "sort";
  label: string;
  options?: TemplateFilterOption[];
  loadOptions?: () => Promise<TemplateFilterOption[]>;
  loadKey?: string;
  placeholder?: string;
  sortFields?: TemplateSortField[];
}

export interface BatchDialogContext {
  ids: string[];
  onSubmit: (values: Record<string, unknown>) => void;
  onCancel: () => void;
}

export interface BatchDialogConfig {
  title: string;
  content: TemplateSurface;
}

export interface BatchActionConfirm {
  title: string;
  description: string | ((count: number) => string);
  confirmLabel?: string;
  cancelLabel?: string;
}

export interface BatchActionConfig {
  id: string;
  label: string;
  tone?: "default" | "danger" | "warning";
  icon?: string;
  confirm?: boolean | BatchActionConfirm;
  dialog?: BatchDialogConfig;
  handler: (ids: string[], values?: Record<string, unknown>) => Promise<void>;
}

export interface InlineReorderConfig {
  enabled: boolean;
  handler: (orderedIds: string[]) => Promise<void>;
  strategy?: "inline";
  successMessage?: string;
}

export interface LoadedReorderConfig<TItem> {
  enabled: boolean;
  strategy: "loaded";
  loadItems: (
    fetch: FetchFn,
    token: string | null,
    query: QueryParams
  ) => Promise<{ items: TItem[]; error?: string }>;
  mapItems?: (items: TItem[]) => Array<TItem & ReorderableItem>;
  handler: (orderedIds: string[]) => Promise<void>;
  successMessage?: string;
}

export interface CustomReorderConfig<TItem> {
  enabled: boolean;
  strategy: "custom";
}

export type ReorderConfig<TItem = unknown> =
  | InlineReorderConfig
  | LoadedReorderConfig<TItem>
  | CustomReorderConfig<TItem>;

export interface PagedListResult<TItem> {
  data: TItem[];
  total?: number | null;
  hasMore?: boolean;
}

export interface ReorderErrorResult {
  message: string;
  highlightedIds?: string[];
}

export type FetchFn = (
  input: RequestInfo | URL,
  init?: RequestInit
) => Promise<Response>;

export type EntityListDataLoader<TItem> = (
  fetch: FetchFn,
  token: string | null,
  query: QueryParams
) => Promise<PagedListResult<TItem>>;

export interface DetailMetaItemConfig {
  label: string;
  value: string | TemplateSurface;
  separator?: boolean;
}

export interface DetailItemConfig {
  label: string;
  value: string | Snippet;
  description?: string;
  emptyText?: string;
  truncateValue?: boolean;
  layout?: "inline" | "stacked";
  presentation?: "simple" | "surface";
  span?: "full" | "half" | null;
}

export interface DetailTabConfig<TItem> {
  id: string;
  label: string;
  count?: number;
  content?: TemplateSurface;
  separator?: boolean;
}

export interface DetailActionConfirm {
  title: string;
  description: string;
  confirmLabel?: string;
  cancelLabel?: string;
}

export interface DetailActionConfig {
  label: string;
  tone?: "default" | "danger" | "warning";
  handler: () => void;
  confirm?: boolean | DetailActionConfirm;
}

export interface SystemIndexCardConfig {
  href: string;
  title: string;
  description: string;
  accent?: string;
  icon?: TemplateSurface;
}

export interface ErrorLogListRequest {
  statusCode?: number;
  limit: number;
  offset: number;
}

export interface ErrorLogListItem {
  id: string;
  occurredAt: string;
  statusCode: number;
  method: string;
  endpoint: string;
  errorCode: string;
  message: string | null;
}

export interface ErrorLogDetailItem extends ErrorLogListItem {
  correlationId: string;
  context?: unknown;
}

export interface ErrorLogStatsSummary {
  totalLast24h: number;
  serverErrorsLast24h: number;
  clientErrorsLast24h: number;
}

export type ErrorLogListLoader = (
  fetch: FetchFn,
  token: string,
  request: ErrorLogListRequest
) => Promise<PagedListResult<ErrorLogListItem>>;

export type ErrorLogDetailLoader = (
  id: string,
  fetch: FetchFn,
  token: string
) => Promise<ErrorLogDetailItem>;

export type ErrorLogStatsLoader = (
  fetch: FetchFn,
  token: string
) => Promise<ErrorLogStatsSummary | null>;

export interface AdminDashboardSectionConfig {
  id: string;
  title?: string;
  content: TemplateSurface;
}

export type TableRowActionFactory<TItem> = (
  row: TableRow<TItem>
) => { value: string; label: string }[];

export type LogEntryMapper<TItem> = (items: TItem[]) => LogEntry[];

export type LogActionTypeResolver = (action: string) => LogActionType;

export type LogActionFormatter = (action: string) => string;

export type LogResourceTypeFormatter = (resourceType: string) => string;

export type LogActorHrefResolver = (actor: LogActor) => string;

export type LogResourceHrefResolver = (
  resourceType: string,
  resourceId: string,
  action: string
) => string | null;

export interface EntityListSharedProps<TItem> {
  dataLoader: EntityListDataLoader<TItem>;
  reloadKey?: string | number;
  idField?: string;
  presentation: "cards" | "table" | "log";
  renderItem?: TemplateSurface;
  renderReorderItem?: TemplateSurface;
  columns?: TableColumn[];
  rowActions?: TableRowActionFactory<TItem>;
  showRowActions?: boolean;
  renderCell?: TemplateSurface;
  renderExpandedRow?: TemplateSurface;
  expandedRowIds?: string[];
  onRowActionSelect?: (row: TableRow<TItem>, action: TableRowAction) => void;
  toLogEntries?: LogEntryMapper<TItem>;
  actionIcon?: TemplateSurface;
  entryDetails?: TemplateSurface;
  getActionType?: LogActionTypeResolver;
  formatAction?: LogActionFormatter;
  formatResourceType?: LogResourceTypeFormatter;
  getActorHref?: LogActorHrefResolver;
  getResourceHref?: LogResourceHrefResolver;
  filters?: FilterConfig[];
  batchActions?: BatchActionConfig[];
  reorder?: ReorderConfig<TItem>;
  customReorderContent?: TemplateSurface;
  onAdd?: () => void;
  addLabel?: string;
  onDataChange?: () => void;
  onSelectedIdsChange?: (ids: string[]) => void;
  query?: QueryParams;
  onQueryChange?: (query: QueryParams) => void;
  onReorderError?: (context: {
    error: unknown;
    controller: ReorderController<ReorderableItem & TItem>;
    items: Array<ReorderableItem & TItem>;
  }) => Promise<string | ReorderErrorResult | void> | string | ReorderErrorResult | void;
}
