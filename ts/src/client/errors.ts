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
