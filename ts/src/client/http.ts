import { UnderlayHttpError, isErrorEnvelope } from "./errors";
import {
  headersToRecord,
  hasHeader,
  setHeaderIfMissing,
} from "./http-headers.js";
import type {
  HttpClient,
  HttpClientOptions,
  HttpRequest,
  HttpRequestOptions,
  HttpResponse,
  RefreshResult,
} from "./http-types.js";

export type {
  HttpAuthOptions,
  HttpClient,
  HttpClientOptions,
  HttpMethod,
  HttpRequest,
  HttpRequestOptions,
  HttpResponse,
  RefreshContext,
  RefreshResult,
  TokenStore,
} from "./http-types.js";
export { MemoryTokenStore } from "./http-token-store.js";

export function createHttpClient(options: HttpClientOptions): HttpClient {
  const fetchImpl = options.fetch ?? globalThis.fetch;
  const timeoutMs = options.timeoutMs ?? 8000;
  const maxRetries = options.maxRetries ?? 3;
  const retryStatuses = new Set([
    502,
    503,
    504,
    ...(options.retryStatuses ?? []),
  ]);
  const debug = options.debug ?? false;
  const credentials = options.credentials ?? "omit";

  function log(...args: unknown[]): void {
    if (debug) {
      console.log("[HTTP]", ...args);
    }
  }

  const tokenStore = options.auth?.tokenStore;

  const getAccessToken =
    options.auth?.getAccessToken ??
    (tokenStore ? () => tokenStore.getAccessToken() : undefined);
  const setAccessToken =
    options.auth?.setAccessToken ??
    (tokenStore
      ? (t: string | null) => tokenStore.setAccessToken(t)
      : undefined);

  const getRefreshToken =
    options.auth?.getRefreshToken ??
    (tokenStore ? () => tokenStore.getRefreshToken() : async () => null);
  const setRefreshToken =
    options.auth?.setRefreshToken ??
    (tokenStore
      ? (t: string | null) => tokenStore.setRefreshToken(t)
      : undefined);

  let refreshInFlight: Promise<RefreshResult> | null = null;

  async function rawRequest<T>(
    req: HttpRequest,
    opts?: { skipRetry?: boolean; acceptedStatuses?: number[] },
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

        const contentType =
          res.headers?.get?.("content-type") ?? "application/json";
        const hasJson = contentType.includes("application/json");

        const acceptedStatuses = new Set(opts?.acceptedStatuses ?? []);
        const isAccepted = res.ok || acceptedStatuses.has(res.status);

        if (!isAccepted) {
          const parsed = hasJson
            ? await res.json().catch(() => undefined)
            : undefined;
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
    requestOptions?: HttpRequestOptions,
  ): Promise<HttpResponse<T>> {
    const token = hasHeader(req.headers ?? {}, "Authorization")
      ? null
      : ((await getAccessToken?.()) ?? null);

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
        setHeaderIfMissing(
          retryHeaders,
          "Authorization",
          `Bearer ${retryToken}`,
        );
      }

      return await rawRequest<T>(
        { ...req, headers: retryHeaders },
        requestOptions,
      );
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
    post: (path, body, headers) =>
      request({ method: "POST", path, body, headers }),
    put: (path, body, headers) =>
      request({ method: "PUT", path, body, headers }),
    patch: (path, body, headers) =>
      request({ method: "PATCH", path, body, headers }),
    delete: (path, headers) => request({ method: "DELETE", path, headers }),
  };
}
