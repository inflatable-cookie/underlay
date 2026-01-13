import type { ErrorEnvelope } from "./types";

export class UnderlayHttpError extends Error {
  readonly status: number;
  readonly envelope?: ErrorEnvelope;

  constructor(status: number, message: string, envelope?: ErrorEnvelope) {
    super(message);
    this.name = "UnderlayHttpError";
    this.status = status;
    this.envelope = envelope;
  }
}

export function isErrorEnvelope(value: unknown): value is ErrorEnvelope {
  if (typeof value !== "object" || value === null) return false;
  const v = value as Record<string, unknown>;
  if (typeof v.error !== "object" || v.error === null) return false;
  const err = v.error as Record<string, unknown>;
  return typeof err.code === "string" && typeof err.message === "string";
}

/**
 * Shape of an API error from HTTP clients.
 * Compatible with UnderlayHttpError and similar error types.
 */
export interface ApiError {
  status?: number;
  code?: string;
  message?: string;
}

/**
 * Convert an API error to a user-friendly message.
 *
 * Handles common error codes and HTTP status codes with appropriate messaging.
 * Use this in form actions and error boundaries to show meaningful messages.
 *
 * @example
 * ```ts
 * try {
 *   await authCommands.login(credentials, fetch);
 * } catch (error) {
 *   return fail(400, { error: toUserMessage(error) });
 * }
 * ```
 */
export function toUserMessage(error: unknown): string {
  const apiError = error as ApiError | null;
  const status = apiError?.status;
  const code = apiError?.code;
  const message = (error as Error | null)?.message;

  // Handle timeout errors
  if (code === "timeout") {
    return "The server took too long to respond.";
  }

  // If no status, use message or generic fallback
  if (typeof status !== "number") {
    return message?.trim() || "Something went wrong while talking to the server.";
  }

  // 401 Unauthorized
  if (status === 401) {
    if (
      code === "auth.session_expired" ||
      code === "auth.session_revoked" ||
      code === "auth.token_invalid" ||
      code === "auth.token_fingerprint_mismatch"
    ) {
      return "Your session has expired. Please sign in again.";
    }

    return message?.trim() || "Please sign in to continue.";
  }

  // 403 Forbidden
  if (status === 403) {
    return message?.trim() || "You do not have permission to do that.";
  }

  // 404 Not Found
  if (status === 404) {
    return message?.trim() || "That resource was not found.";
  }

  // 400 Bad Request
  if (status === 400) {
    return message?.trim() || "That request wasn't accepted.";
  }

  // 5xx Server Error or network error (status 0)
  if (status >= 500 || status === 0) {
    return "The server is unavailable right now.";
  }

  // Fallback for other status codes
  return message?.trim() || "Something went wrong while talking to the server.";
}
