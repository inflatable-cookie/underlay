import { UnderlayHttpError, isErrorEnvelope } from "./errors";

export type HttpMethod = "GET" | "POST" | "PUT" | "PATCH" | "DELETE";

export interface HttpClientOptions {
  baseUrl: string;
  defaultHeaders?: Record<string, string>;
  fetch?: typeof globalThis.fetch;
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

  async function request<T>(req: HttpRequest): Promise<T> {
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

    const res = await fetchImpl(url, {
      method: req.method,
      headers,
      body,
    });

    const contentType = res.headers.get("content-type") ?? "";
    const hasJson = contentType.includes("application/json");

    if (!res.ok) {
      const parsed = hasJson ? await res.json().catch(() => undefined) : undefined;
      const envelope = isErrorEnvelope(parsed) ? parsed : undefined;
      const message = envelope?.error.message ?? `HTTP ${res.status}`;
      throw new UnderlayHttpError(res.status, message, envelope);
    }

    if (res.status === 204) {
      return undefined as T;
    }

    if (!hasJson) {
      return (await res.text()) as unknown as T;
    }

    return (await res.json()) as T;
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
