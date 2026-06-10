export const ADMIN_PILL_ACCENTS = {
  live: "#22c55e",
  draft: "#ef4444",
  free: "#22c55e",
  restricted: "#f97316",
  success: "#22c55e",
  warning: "#f97316",
  danger: "#ef4444",
  info: "#3b82f6",
  neutral: "#64748b"
} as const;

export type AdminPillKind = keyof typeof ADMIN_PILL_ACCENTS;

