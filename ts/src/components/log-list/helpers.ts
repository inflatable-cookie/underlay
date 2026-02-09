import type { LogActionType } from "../LogList.svelte";

export type LogActionBadgeVariant =
  | "default"
  | "success"
  | "warning"
  | "danger"
  | "info"
  | "muted";

export function getDefaultActionType(action: string): LogActionType {
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
  ) {
    return "security";
  }
  return "other";
}

export function getActionVariant(actionType: LogActionType): LogActionBadgeVariant {
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

export function formatDefaultAction(action: string): string {
  return action.replace(/_/g, " ");
}

export function formatDefaultResourceType(resourceType: string): string {
  return resourceType.replace(/_/g, " ");
}
