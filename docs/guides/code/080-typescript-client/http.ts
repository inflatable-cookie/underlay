import {
  createHttpClient,
  type HttpClient,
  UnderlayHttpError,
  type ErrorEnvelope,
  isErrorEnvelope,
} from "@decodelabs/underlay/client";

export interface StemHttpConfig {
  baseUrl: string;
  apiVersion: string;
  fetchFn?: typeof fetch;
  getToken: () => string | null;
}

export function createStemHttp(config: StemHttpConfig): HttpClient {
  // API routes are under /v1; versioning (if used) is via header.
  const baseUrl = new URL("/v1/", config.baseUrl).toString();

  return createHttpClient({
    baseUrl,
    fetch: config.fetchFn,
    defaultHeaders: {
      "X-Api-Version": config.apiVersion,
    },
  });
}

export function authHeaders(getToken: () => string | null): Record<string, string> {
  const token = getToken();
  return token ? { Authorization: `Bearer ${token}` } : {};
}

export function getErrorEnvelope(err: unknown): ErrorEnvelope | null {
  if (err instanceof UnderlayHttpError) {
    return err.envelope ?? null;
  }
  return null;
}

// Example: Using isErrorEnvelope type guard
export function handleError(err: unknown): string {
  if (isErrorEnvelope(err)) {
    return err.error.message;
  }
  if (err instanceof UnderlayHttpError) {
    return err.message;
  }
  return "An unexpected error occurred";
}
