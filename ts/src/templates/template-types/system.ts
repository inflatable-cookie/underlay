import type { FetchFn, PagedListResult, TemplateSurface } from "./primitives";

export interface SystemIndexCardConfig {
  href: string;
  title: string;
  description: string;
  accent?: string;
  icon?: TemplateSurface;
}

export interface AdminDashboardSectionConfig {
  id: string;
  title?: string;
  content: TemplateSurface;
}

export interface ErrorLogListRequest {
  variant?: string;
  statusClass?: "4xx" | "5xx";
  statusCode?: number;
  page: number;
  limit: number;
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
  request: ErrorLogListRequest,
) => Promise<PagedListResult<ErrorLogListItem>>;

export type ErrorLogDetailLoader = (
  id: string,
  fetch: FetchFn,
  token: string,
) => Promise<ErrorLogDetailItem>;

export type ErrorLogStatsLoader = (
  fetch: FetchFn,
  token: string,
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
  succeeded: number;
}

export interface SystemJobListRequest {
  variant?: string;
  status?: SystemJobStatus;
  page: number;
  limit: number;
}

export type SystemJobListLoader<
  TJob extends SystemJobListItem = SystemJobListItem,
> = (
  fetch: FetchFn,
  token: string,
  request: SystemJobListRequest,
) => Promise<PagedListResult<TJob>>;

export type SystemJobStatsLoader = (
  fetch: FetchFn,
  token: string,
) => Promise<SystemJobStatsSummary | null>;

export type SystemJobAction<
  TJob extends SystemJobListItem = SystemJobListItem,
> = (job: TJob, fetch: FetchFn, token: string) => Promise<void>;

export type SystemJobDetailLoader<
  TJob extends SystemJobDetailItem = SystemJobDetailItem,
> = (id: string, fetch: FetchFn, token: string) => Promise<TJob>;

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
  TTask extends SystemScheduledTaskListItem = SystemScheduledTaskListItem,
> = (
  fetch: FetchFn,
  token: string,
  request: SystemScheduledTaskListRequest,
) => Promise<PagedListResult<TTask>>;

export type SystemScheduledTaskAction<
  TTask extends SystemScheduledTaskListItem = SystemScheduledTaskListItem,
> = (
  task: TTask,
  fetch: FetchFn,
  token: string,
) => Promise<{ jobId?: string | null } | void>;

export type SystemScheduledTaskDetailLoader<
  TTask extends SystemScheduledTaskDetailItem = SystemScheduledTaskDetailItem,
> = (id: string, fetch: FetchFn, token: string) => Promise<TTask>;

export type SystemScheduledTaskJobRunsLoader<
  TTask extends SystemScheduledTaskDetailItem = SystemScheduledTaskDetailItem,
  TJob extends SystemJobListItem = SystemJobListItem,
> = (
  task: TTask,
  fetch: FetchFn,
  token: string,
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
  request: SystemAuditLogListRequest,
) => Promise<PagedListResult<SystemAuditLogEntry>>;
