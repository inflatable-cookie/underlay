import type { FieldErrors } from "./types";

export function hasFieldErrors(
  data: unknown,
): data is { fieldErrors: FieldErrors } {
  return (
    typeof data === "object" &&
    data !== null &&
    "fieldErrors" in data &&
    typeof (data as Record<string, unknown>).fieldErrors === "object"
  );
}

export function extractErrorMessage(
  error: unknown,
  fallback: string = "An error occurred",
): string {
  if (typeof error === "string") return error;
  if (error instanceof Error) return error.message;

  if (typeof error === "object" && error !== null) {
    const obj = error as Record<string, unknown>;
    if (typeof obj.message === "string") return obj.message;
    if (typeof obj.error === "string") return obj.error;
  }

  return fallback;
}

export function mergeFieldErrors(
  ...sources: (FieldErrors | undefined | null)[]
): FieldErrors {
  const result: FieldErrors = {};
  for (const source of sources) {
    if (source) {
      Object.assign(result, source);
    }
  }
  return result;
}
