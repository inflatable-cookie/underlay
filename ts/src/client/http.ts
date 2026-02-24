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
   * This is the number of *retries* (in addition to the initial attempt).
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

  /**
   * Fetch credentials mode.
   * - 'omit': Never send cookies (default)
   * - 'same-origin': Send cookies for same-origin requests
   * - 'include': Always send cookies (needed for cross-origin auth)
   * @default 'omit'
   */
  credentials?: RequestCredentials;
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
  /** Whether the refresh succeeded. */
  success: boolean;

  /** If provided, the client will update the token store. */
  accessToken?: string | null;
  /** If provided, the client will update the token store. */
  refreshToken?: string | null;
}

export interface HttpRequest {
  method: HttpMethod;
  path: string;
  headers?: Record<string, string>;
  body?: unknown;
}

export interface HttpResponse<T> {
  status: number;
  headers: Record<string, string>;
  body: T | null;
}

export interface HttpRequestOptions {
  acceptedStatuses?: number[];
}

export interface HttpClient {
  request<T>(req: HttpRequest): Promise<T>;
  requestWithMeta<T>(req: HttpRequest, options?: HttpRequestOptions): Promise<HttpResponse<T>>;
  get<T>(path: string, headers?: Record<string, string>): Promise<T>;
  getWithMeta<T>(
    path: string,
    headers?: Record<string, string>,
    options?: HttpRequestOptions
  ): Promise<HttpResponse<T>>;
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
  const credentials = options.credentials ?? "omit";

  function hasHeader(headers: Record<string, string>, name: string): boolean {
    const target = name.toLowerCase();
    return Object.keys(headers).some((k) => k.toLowerCase() === target);
  }

  function setHeaderIfMissing(headers: Record<string, string>, name: string, value: string): void {
    if (hasHeader(headers, name)) return;
    headers[name] = value;
  }

  function log(...args: unknown[]): void {
    if (debug) {
      console.log("[HTTP]", ...args);
    }
  }

  function headersToRecord(headers: Headers | undefined): Record<string, string> {
    const result: Record<string, string> = {};
    if (!headers || typeof headers.forEach !== "function") return result;
    headers.forEach((value, key) => {
      result[key] = value;
    });
    return result;
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

  async function rawRequest<T>(
    req: HttpRequest,
    opts?: { skipRetry?: boolean; acceptedStatuses?: number[] }
  ): Promise<HttpResponse<T>> {
    const url = new URL(req.path, options.baseUrl).toString();
    const headers: Record<string, string> = {
      ...options.defaultHeaders,
      ...req.headers,
    };

    setHeaderIfMissing(headers, "Accept", "application/json");

    let body: BodyInit | undefined;
    if (req.body !== undefined) {
      setHeaderIfMissing(headers, "Content-Type", "application/json");
      body = JSON.stringify(req.body);
    }

    // Determine if this request is idempotent (can be retried)
    const isIdempotent = req.method === "GET" || req.method === "DELETE";
    const shouldRetry = isIdempotent && !opts?.skipRetry;

    let attempt = 0;
    let retries = 0;

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
        log(req.method, req.path);

        const res = await fetchImpl(url, {
          method: req.method,
          headers,
          body,
          signal: controller?.signal,
          credentials,
        });

        if (timeout != null) {
          clearTimeout(timeout);
        }

        const contentType = res.headers?.get?.("content-type") ?? "application/json";
        const hasJson = contentType.includes("application/json");

        const acceptedStatuses = new Set(opts?.acceptedStatuses ?? []);
        const isAccepted = res.ok || acceptedStatuses.has(res.status);

        if (!isAccepted) {
          const parsed = hasJson ? await res.json().catch(() => undefined) : undefined;
          const envelope = isErrorEnvelope(parsed) ? parsed : undefined;
          const message = envelope?.error.message ?? `HTTP ${res.status}`;

          log(`Error response (${res.status}):`, envelope);

          // Check if we should retry this error
          const canRetry =
            shouldRetry &&
            retryStatuses.has(res.status) &&
            retries < maxRetries;

          if (canRetry) {
            retries += 1;
            log("Retrying due to status", res.status);
            // Exponential backoff: 100ms, 200ms, 400ms, etc.
            const backoffMs = Math.min(100 * Math.pow(2, retries - 1), 3000);
            await new Promise((resolve) => setTimeout(resolve, backoffMs));
            continue;
          }

          throw new UnderlayHttpError(res.status, message, envelope);
        }

        const responseHeaders = headersToRecord(res.headers);

        if (res.status === 204) {
          return { status: res.status, headers: responseHeaders, body: null };
        }

        if (res.status === 304) {
          return { status: res.status, headers: responseHeaders, body: null };
        }

        if (!hasJson) {
          return {
            status: res.status,
            headers: responseHeaders,
            body: (await res.text()) as unknown as T,
          };
        }

        return {
          status: res.status,
          headers: responseHeaders,
          body: (await res.json()) as T,
        };
      } catch (err) {
        if (timeout != null) {
          clearTimeout(timeout);
        }

        // If it's an HTTP error (not network/timeout), don't retry - we already handled that above
        if (err instanceof UnderlayHttpError) {
          throw err;
        }

        const message = err instanceof Error ? err.message : "Network error";
        throw new UnderlayHttpError(0, message);
      }
    }
  }

  async function rawRequestBody<T>(req: HttpRequest): Promise<T> {
    const response = await rawRequest<T>(req);
    return response.body as T;
  }

  async function requestWithMeta<T>(
    req: HttpRequest,
    requestOptions?: HttpRequestOptions
  ): Promise<HttpResponse<T>> {
    const token = hasHeader(req.headers ?? {}, "Authorization")
      ? null
      : (await getAccessToken?.()) ?? null;

    const headers: Record<string, string> = { ...(req.headers ?? {}) };
    if (token) {
      setHeaderIfMissing(headers, "Authorization", `Bearer ${token}`);
    }

    try {
      return await rawRequest<T>({ ...req, headers }, requestOptions);
    } catch (err) {
      if (!(err instanceof UnderlayHttpError)) throw err;

      if (err.status !== 401 || !options.auth?.refresh) {
        throw err;
      }

      if (!refreshInFlight) {
        refreshInFlight = options.auth
          .refresh({
            rawRequest: rawRequestBody,
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

      if (!refreshed.success) {
        await tokenStore?.clear();
        await setAccessToken?.(null);
        await setRefreshToken?.(null);
        throw err;
      }

      if (refreshed.accessToken !== undefined) {
        await setAccessToken?.(refreshed.accessToken);
      }
      if (refreshed.refreshToken !== undefined) {
        await setRefreshToken?.(refreshed.refreshToken);
      }

      const retryToken = (await getAccessToken?.()) ?? null;
      const retryHeaders: Record<string, string> = { ...(req.headers ?? {}) };
      if (retryToken) {
        setHeaderIfMissing(retryHeaders, "Authorization", `Bearer ${retryToken}`);
      }

      return await rawRequest<T>({ ...req, headers: retryHeaders }, requestOptions);
    }
  }

  async function request<T>(req: HttpRequest): Promise<T> {
    const response = await requestWithMeta<T>(req);
    return response.body as T;
  }

  return {
    request,
    requestWithMeta,
    get: (path, headers) => request({ method: "GET", path, headers }),
    getWithMeta: (path, headers, options) =>
      requestWithMeta({ method: "GET", path, headers }, options),
    post: (path, body, headers) => request({ method: "POST", path, body, headers }),
    put: (path, body, headers) => request({ method: "PUT", path, body, headers }),
    patch: (path, body, headers) => request({ method: "PATCH", path, body, headers }),
    delete: (path, headers) => request({ method: "DELETE", path, headers }),
  };
}
