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

export interface InlineListDialogContext {
  close: () => void;
  refetch: () => Promise<void>;
}

export interface InlineListDialogConfig {
  title: string;
  description?: string;
  width?: "sm" | "md" | "lg" | "xl" | "full";
  content: TemplateSurface;
}

export interface InlineListItemActionConfig<TItem> {
  label: string;
  handler: (item: TItem) => void | Promise<void>;
  disabled?: boolean;
  destructive?: boolean;
  separator?: boolean;
}

export interface InlineListItemDeleteConfig<TItem> {
  title: string;
  description: string;
  confirmLabel: string;
  entityLabel?: (item: TItem) => string | null;
  handler: (item: TItem) => void | Promise<void>;
}

export interface ReorderActionState {
  active: boolean;
  available: boolean;
  dirty: boolean;
  saving: boolean;
  enter: () => void | Promise<void>;
  save: () => void | Promise<void>;
  cancel: () => void;
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
  disabled?: boolean | ((ids: string[]) => boolean);
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
  code?: boolean;
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

export type SystemJobStatus =
  | "pending"
  | "claimed"
  | "running"
  | "succeeded"
  | "failed"
  | "cancelled"
  | string;

export interface SystemJobListItem {
  id: string;
  jobType: string;
  status: SystemJobStatus;
  attempts: number;
  maxAttempts: number;
  createdAt: string;
  finishedAt?: string | null;
  errorMessage?: string | null;
}

export interface SystemJobDetailItem extends SystemJobListItem {
  scheduledFor?: string | null;
  startedAt?: string | null;
  finishedAt?: string | null;
  claimedAt?: string | null;
  heartbeatAt?: string | null;
  claimedBy?: string | null;
  priority?: number | null;
  payload?: unknown;
  progress?: unknown;
  lastError?: string | null;
  errorHistory?: unknown;
}

export interface SystemJobStatsSummary {
  pending: number;
  running: number;
  failed: number;
  succeededRecent: number;
}

export interface SystemJobListRequest {
  status?: SystemJobStatus;
  page: number;
  limit: number;
}

export type SystemJobListLoader<TJob extends SystemJobListItem = SystemJobListItem> = (
  fetch: FetchFn,
  token: string,
  request: SystemJobListRequest
) => Promise<PagedListResult<TJob>>;

export type SystemJobStatsLoader = (
  fetch: FetchFn,
  token: string
) => Promise<SystemJobStatsSummary | null>;

export type SystemJobAction<TJob extends SystemJobListItem = SystemJobListItem> = (
  job: TJob,
  fetch: FetchFn,
  token: string
) => Promise<void>;

export type SystemJobDetailLoader<
  TJob extends SystemJobDetailItem = SystemJobDetailItem
> = (
  id: string,
  fetch: FetchFn,
  token: string
) => Promise<TJob>;

export interface SystemScheduledTaskListItem {
  id: string;
  name: string;
  jobType: string;
  schedule: string;
  enabled: boolean;
  lastScheduledAt?: string | null;
  lastCompletedAt?: string | null;
  createdAt?: string | null;
}

export interface SystemScheduledTaskDetailItem extends SystemScheduledTaskListItem {
  priority?: number | null;
  maxAttempts?: number | null;
  timeoutSeconds?: number | null;
  allowOverlap?: boolean | null;
  updatedAt?: string | null;
  payload?: unknown;
}

export interface SystemScheduledTaskListRequest {
  enabled?: boolean;
  page: number;
  limit: number;
}

export type SystemScheduledTaskListLoader<
  TTask extends SystemScheduledTaskListItem = SystemScheduledTaskListItem
> = (
  fetch: FetchFn,
  token: string,
  request: SystemScheduledTaskListRequest
) => Promise<PagedListResult<TTask>>;

export type SystemScheduledTaskAction<
  TTask extends SystemScheduledTaskListItem = SystemScheduledTaskListItem
> = (
  task: TTask,
  fetch: FetchFn,
  token: string
) => Promise<{ jobId?: string | null } | void>;

export type SystemScheduledTaskDetailLoader<
  TTask extends SystemScheduledTaskDetailItem = SystemScheduledTaskDetailItem
> = (
  id: string,
  fetch: FetchFn,
  token: string
) => Promise<TTask>;

export type SystemScheduledTaskJobRunsLoader<
  TTask extends SystemScheduledTaskDetailItem = SystemScheduledTaskDetailItem,
  TJob extends SystemJobListItem = SystemJobListItem
> = (
  task: TTask,
  fetch: FetchFn,
  token: string
) => Promise<PagedListResult<TJob> | TJob[]>;

export interface SystemAuditActor {
  id: string;
  email?: string | null;
  name?: string | null;
}

export interface SystemAuditLogEntry {
  id: string;
  occurredAt: string;
  actor?: SystemAuditActor | null;
  action: string;
  resourceType: string;
  resourceId: string;
  resourceLabel?: string | null;
  details?: Record<string, unknown> | unknown;
}

export interface SystemAuditLogListRequest {
  action?: string;
  resourceType?: string;
  page: number;
  limit: number;
}

export type SystemAuditLogListLoader = (
  fetch: FetchFn,
  token: string,
  request: SystemAuditLogListRequest
) => Promise<PagedListResult<SystemAuditLogEntry>>;

export interface SystemMediaTrashItem {
  id: string;
  kind: string;
  title?: string | null;
  originalFilename?: string | null;
  thumbnailUrl?: string | null;
  byteSize?: number | null;
  deletedAt?: string | null;
}

export type SystemMediaTrashListLoader<
  TMedia extends SystemMediaTrashItem = SystemMediaTrashItem
> = (
  fetch: FetchFn,
  token: string
) => Promise<PagedListResult<TMedia>>;

export type SystemMediaTrashAction<
  TMedia extends SystemMediaTrashItem = SystemMediaTrashItem
> = (
  media: TMedia,
  fetch: FetchFn,
  token: string
) => Promise<void>;

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
