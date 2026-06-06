import type { FieldErrors } from "./forms/types";

/** SvelteKit ActionResult type (simplified) */
export interface ActionResult {
  type: "success" | "failure" | "redirect" | "error";
  status?: number;
  data?: Record<string, unknown>;
  location?: string;
  error?: Error;
}

export interface ActionFailureResult {
  message: string;
  fieldErrors: FieldErrors;
}

/**
 * Parse action failure payload into a normalized message + field error map.
 */
export function resolveActionFailureResult(
  data: Record<string, unknown> = {},
): ActionFailureResult {
  const message =
    typeof data.error === "string"
      ? data.error
      : typeof data.message === "string"
        ? data.message
        : "Validation failed";

  const fieldErrors: FieldErrors = {};
  if (
    data.fieldErrors &&
    typeof data.fieldErrors === "object" &&
    !Array.isArray(data.fieldErrors)
  ) {
    for (const [key, value] of Object.entries(
      data.fieldErrors as Record<string, unknown>,
    )) {
      if (typeof value === "string") {
        fieldErrors[key] = value;
      }
    }
  }

  return {
    message,
    fieldErrors,
  };
}
