import { UnderlayHttpError } from "@inflatable-cookie/underlay/client/errors";
import type { ErrorEnvelope } from "@inflatable-cookie/underlay/client/types";

export function isUnderlayHttpError(err: unknown): err is UnderlayHttpError {
  return err instanceof UnderlayHttpError;
}

export function getErrorCode(err: unknown): string | null {
  if (!isUnderlayHttpError(err)) return null;
  return err.envelope?.error.code ?? null;
}

export function getFieldErrors(err: unknown): Record<string, string> | null {
  if (!isUnderlayHttpError(err)) return null;
  return err.envelope?.error.fieldErrors ?? null;
}

export function getErrorEnvelope(err: unknown): ErrorEnvelope | null {
  if (!isUnderlayHttpError(err)) return null;
  return err.envelope ?? null;
}
