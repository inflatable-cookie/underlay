import type { PillTone } from "@inflatable-cookie/poodle-svelte";
import type { FetchFn, PagedListResult } from "./primitives";

export interface UserSessionItem {
  id: string;
  status: string;
  ipAddress?: string | null;
  userAgent?: string | null;
  createdAt: string;
  lastUsedAt: string;
}

export interface UserActivityActor {
  email?: string | null;
}

export interface UserActivityItem {
  id: string;
  occurredAt: string;
  action: string;
  resourceType: string;
  resourceId?: string | null;
  actor?: UserActivityActor | null;
}

export interface UserTabListRequest {
  page: number;
  limit: number;
}

export type UserSessionListLoader = (
  userId: string,
  fetch: FetchFn,
  token: string,
  request: UserTabListRequest,
) => Promise<PagedListResult<UserSessionItem>>;

export type UserActivityListLoader = (
  userId: string,
  fetch: FetchFn,
  token: string,
  request: UserTabListRequest,
) => Promise<PagedListResult<UserActivityItem>>;

export type UserSessionRevokeAction = (
  session: UserSessionItem,
  fetch: FetchFn,
  token: string,
) => Promise<void>;

export function getUserSessionStatusTone(status: string): PillTone {
  switch (status) {
    case "active":
      return "success";
    case "revoked":
      return "danger";
    default:
      return "neutral";
  }
}

export function getUserActivityActionTone(action: string): PillTone {
  switch (action) {
    case "delete":
    case "revoke":
      return "danger";
    default:
      return "neutral";
  }
}
