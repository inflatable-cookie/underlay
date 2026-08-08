import type { PillTone } from "@inflatable-cookie/poodle-svelte";
import type { QueryParams } from "../client/query";

export interface UsersListUser {
  id: string;
  email: string;
  displayName?: string | null;
  role: string;
  status: string;
  createdAt: string;
}

export type UsersListLoader<T extends UsersListUser = UsersListUser> = (
  fetchFn: typeof fetch,
  token: string | null,
  query: QueryParams
) => Promise<{ data: T[]; total: number; hasMore?: boolean }>;

export function getUserRoleTone(role: string): PillTone {
  return role === "superadmin" ? "danger" : "neutral";
}

export function getUserStatusTone(status: string): PillTone {
  switch (status) {
    case "active": return "success";
    case "deleted": return "danger";
    default: return "neutral";
  }
}

export const DEFAULT_USER_ROLE_OPTIONS = [
  { value: "All", label: "All roles" },
  { value: "user", label: "User" },
  { value: "tester", label: "Tester" },
  { value: "editor", label: "Editor" },
  { value: "admin", label: "Admin" },
  { value: "support", label: "Support" },
  { value: "superadmin", label: "Superadmin" }
];

export const DEFAULT_USER_STATUS_OPTIONS = [
  { value: "All", label: "All statuses" },
  { value: "active", label: "Active" },
  { value: "suspended", label: "Suspended" },
  { value: "deleted", label: "Deleted" }
];
