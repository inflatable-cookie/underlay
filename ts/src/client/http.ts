import { UnderlayHttpError, isErrorEnvelope } from "./errors";

export type HttpMethod = "GET" | "POST" | "PUT" | "PATCH" | "DELETE";

export interface TokenStore {
  getAccessToken(): string | null | Promise<string | null>;
  setAccessToken(token: string | null): void | Promise<void>;

  getRefreshToken(): string | null | Promise<string | null>;
  setRefreshToken(token: string | null): void | Promise<void>;

  clear(): void | Promise<void>;
}

export class MemoryTokenStore implements TokenStore {
  private accessToken: string | null = null;
  private refreshToken: string | null = null;

  getAccessToken(): string | null {
    return this.accessToken;
  }

  setAccessToken(token: string | null): void {
    this.accessToken = token;
  }

  getRefreshToken(): string | null {
    return this.refreshToken;
  }

  setRefreshToken(token: string | null): void {
    this.refreshToken = token;
  }

  clear(): void {
    this.accessToken = null;
    this.refreshToken = null;
  }
}

export interface HttpClientOptions {
  baseUrl: string;
  defaultHeaders?: Record<string, string>;
  fetch?: typeof globalThis.fetch;

  auth?: HttpAuthOptions;

  /**
   * Request timeout in milliseconds for idempotent requests (GET, DELETE).
   * Non-idempotent requests (POST, PUT, PATCH) are not subject to timeout.
   * @default 8000
   */
  timeoutMs?: number;

  /**
   * Maximum number of retry attempts for retryable operations.
   * Retries are only attempted for idempotent requests (GET, DELETE) on:
   * - 502 Bad Gateway
   * - 503 Service Unavailable
   * - 504 Gateway Timeout
   * - Custom statuses specified in `retryStatuses`
   * @default 3
   */
  maxRetries?: number;

  /**
   * Additional HTTP status codes to retry (beyond default 502, 503, 504).
   * Example: [429] to retry on rate limit.
   */
  retryStatuses?: number[];

  /**
   * Enable debug logging to console.
   * @default false
   */
  debug?: boolean;
}

export interface HttpAuthOptions {
  tokenStore?: TokenStore;

  getAccessToken?: () => string | null | Promise<string | null>;
  setAccessToken?: (token: string | null) => void | Promise<void>;

  getRefreshToken?: () => string | null | Promise<string | null>;
  setRefreshToken?: (token: string | null) => void | Promise<void>;

  /**
   * Called when a request fails with 401; return true to retry.
   *
   * Use `rawRequest` so refresh calls do not include Authorization headers.
   */
  refresh?: (ctx: RefreshContext) => Promise<RefreshResult>;
}

export interface RefreshContext {
  rawRequest<T>(req: HttpRequest): Promise<T>;
  tokenStore?: TokenStore;

  getRefreshToken(): Promise<string | null>;
  setAccessToken(token: string | null): Promise<void>;
  setRefreshToken(token: string | null): Promise<void>;
}

export interface RefreshResult {
  /** If provided, the client will update the token store. */
  accessToken?: string | null;
  /** If provided, the client will update the token store. */
  refreshToken?: string | null;

  /** Whether to retry the original request after refresh. */
  retry: boolean;
}

export interface HttpRequest {
  method: HttpMethod;
  path: string;
  headers?: Record<string, string>;
  body?: unknown;
}

export interface HttpClient {
  request<T>(req: HttpRequest): Promise<T>;
  get<T>(path: string, headers?: Record<string, string>): Promise<T>;
  post<T>(path: string, body?: unknown, headers?: Record<string, string>): Promise<T>;
  put<T>(path: string, body?: unknown, headers?: Record<string, string>): Promise<T>;
  patch<T>(path: string, body?: unknown, headers?: Record<string, string>): Promise<T>;
  delete<T>(path: string, headers?: Record<string, string>): Promise<T>;
}

export function createHttpClient(options: HttpClientOptions): HttpClient {
  const fetchImpl = options.fetch ?? globalThis.fetch;
  const timeoutMs = options.timeoutMs ?? 8000;
  const maxRetries = options.maxRetries ?? 3;
  const retryStatuses = new Set([502, 503, 504, ...(options.retryStatuses ?? [])]);
  const debug = options.debug ?? false;

  function log(...args: unknown[]): void {
    if (debug) {
      console.log("[HttpClient]", ...args);
    }
  }

  const tokenStore = options.auth?.tokenStore;

  const getAccessToken =
    options.auth?.getAccessToken ??
    (tokenStore ? () => tokenStore.getAccessToken() : undefined);
  const setAccessToken =
    options.auth?.setAccessToken ??
    (tokenStore ? (t: string | null) => tokenStore.setAccessToken(t) : undefined);

  const getRefreshToken =
    options.auth?.getRefreshToken ??
    (tokenStore ? () => tokenStore.getRefreshToken() : async () => null);
  const setRefreshToken =
    options.auth?.setRefreshToken ??
    (tokenStore ? (t: string | null) => tokenStore.setRefreshToken(t) : undefined);

  let refreshInFlight: Promise<RefreshResult> | null = null;

  async function rawRequest<T>(req: HttpRequest, opts?: { skipRetry?: boolean }): Promise<T> {
    const url = new URL(req.path, options.baseUrl);
    const headers: Record<string, string> = {
      ...options.defaultHeaders,
      ...req.headers,
    };

    let body: BodyInit | undefined;
    if (req.body !== undefined) {
      headers["content-type"] ??= "application/json";
      body = JSON.stringify(req.body);
    }

    // Determine if this request is idempotent (can be retried)
    const isIdempotent = req.method === "GET" || req.method === "DELETE";
    const shouldRetry = isIdempotent && !opts?.skipRetry;

    let attempt = 0;

    while (true) {
      attempt += 1;

      // Set up timeout for idempotent requests
      const controller =
        typeof AbortController !== "undefined" && isIdempotent
          ? new AbortController()
          : undefined;

      const timeout =
        controller != null
          ? setTimeout(() => controller.abort(), timeoutMs)
          : undefined;

      try {
        log(`${req.method} ${url} (attempt ${attempt}/${maxRetries})`);

        const res = await fetchImpl(url, {
          method: req.method,
          headers,
          body,
          signal: controller?.signal,
        });

        if (timeout != null) {
          clearTimeout(timeout);
        }

        const contentType = res.headers.get("content-type") ?? "";
        const hasJson = contentType.includes("application/json");

        if (!res.ok) {
          const parsed = hasJson ? await res.json().catch(() => undefined) : undefined;
          const envelope = isErrorEnvelope(parsed) ? parsed : undefined;
          const message = envelope?.error.message ?? `HTTP ${res.status}`;

          log(`Error response (${res.status}):`, envelope);

          // Check if we should retry this error
          const canRetry =
            shouldRetry &&
            retryStatuses.has(res.status) &&
            attempt < maxRetries;

          if (canRetry) {
            log(`Retrying due to status ${res.status}...`);
            // Exponential backoff: 100ms, 200ms, 400ms, etc.
            const backoffMs = Math.min(100 * Math.pow(2, attempt - 1), 3000);
            await new Promise((resolve) => setTimeout(resolve, backoffMs));
            continue;
          }

          throw new UnderlayHttpError(res.status, message, envelope);
        }

        if (res.status === 204) {
          log(`${req.method} ${url} -> 204 No Content`);
          return undefined as T;
        }

        if (!hasJson) {
          return (await res.text()) as unknown as T;
        }

        return (await res.json()) as T;
      } catch (err) {
        if (timeout != null) {
          clearTimeout(timeout);
        }

        // If it's an HTTP error (not network/timeout), don't retry - we already handled that above
        if (err instanceof UnderlayHttpError) {
          throw err;
        }

        // Network/timeout error - retry if allowed
        const canRetry = shouldRetry && attempt < maxRetries;

        if (canRetry) {
          log(`Network error, retrying:`, err);
          const backoffMs = Math.min(100 * Math.pow(2, attempt - 1), 3000);
          await new Promise((resolve) => setTimeout(resolve, backoffMs));
          continue;
        }

        throw err;
      }
    }
  }

  async function request<T>(req: HttpRequest): Promise<T> {
    const authHeader = req.headers?.authorization ?? req.headers?.Authorization;
    const token = authHeader ? null : await getAccessToken?.();

    const headers = token
      ? {
          ...req.headers,
          authorization: `Bearer ${token}`,
        }
      : req.headers;

    try {
      return await rawRequest<T>({ ...req, headers });
    } catch (err) {
      if (!(err instanceof UnderlayHttpError)) {
        throw err;
      }

      if (err.status !== 401 || !options.auth?.refresh) {
        throw err;
      }

      if (!refreshInFlight) {
        refreshInFlight = options.auth
          .refresh({
            rawRequest,
            tokenStore,
            getRefreshToken: async () => (await getRefreshToken?.()) ?? null,
            setAccessToken: async (t) => {
              await setAccessToken?.(t);
            },
            setRefreshToken: async (t) => {
              await setRefreshToken?.(t);
            },
          })
          .finally(() => {
            refreshInFlight = null;
          });
      }

      const refreshed = await refreshInFlight;

      if (refreshed.accessToken !== undefined) {
        await setAccessToken?.(refreshed.accessToken);
      }
      if (refreshed.refreshToken !== undefined) {
        await setRefreshToken?.(refreshed.refreshToken);
      }

      if (!refreshed.retry) {
        throw err;
      }

      const retryToken = await getAccessToken?.();
      const retryHeaders = retryToken
        ? {
            ...req.headers,
            authorization: `Bearer ${retryToken}`,
          }
        : req.headers;

      return await rawRequest<T>({ ...req, headers: retryHeaders });
    }
  }

  return {
    request,
    get: (path, headers) => request({ method: "GET", path, headers }),
    post: (path, body, headers) => request({ method: "POST", path, body, headers }),
    put: (path, body, headers) => request({ method: "PUT", path, body, headers }),
    patch: (path, body, headers) => request({ method: "PATCH", path, body, headers }),
    delete: (path, headers) => request({ method: "DELETE", path, headers }),
  };
}
