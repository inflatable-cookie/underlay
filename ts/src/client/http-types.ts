export type HttpMethod = "GET" | "POST" | "PUT" | "PATCH" | "DELETE";

export interface TokenStore {
  getAccessToken(): string | null | Promise<string | null>;
  setAccessToken(token: string | null): void | Promise<void>;

  getRefreshToken(): string | null | Promise<string | null>;
  setRefreshToken(token: string | null): void | Promise<void>;

  clear(): void | Promise<void>;
}

export interface HttpClientOptions {
  baseUrl: string;
  defaultHeaders?: Record<string, string>;
  fetch?: typeof globalThis.fetch;
  auth?: HttpAuthOptions;
  timeoutMs?: number;
  maxRetries?: number;
  retryStatuses?: number[];
  debug?: boolean;
  credentials?: RequestCredentials;
}

export interface HttpAuthOptions {
  tokenStore?: TokenStore;

  getAccessToken?: () => string | null | Promise<string | null>;
  setAccessToken?: (token: string | null) => void | Promise<void>;

  getRefreshToken?: () => string | null | Promise<string | null>;
  setRefreshToken?: (token: string | null) => void | Promise<void>;

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
  success: boolean;
  accessToken?: string | null;
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
  requestWithMeta<T>(
    req: HttpRequest,
    options?: HttpRequestOptions,
  ): Promise<HttpResponse<T>>;
  get<T>(path: string, headers?: Record<string, string>): Promise<T>;
  getWithMeta<T>(
    path: string,
    headers?: Record<string, string>,
    options?: HttpRequestOptions,
  ): Promise<HttpResponse<T>>;
  post<T>(
    path: string,
    body?: unknown,
    headers?: Record<string, string>,
  ): Promise<T>;
  put<T>(
    path: string,
    body?: unknown,
    headers?: Record<string, string>,
  ): Promise<T>;
  patch<T>(
    path: string,
    body?: unknown,
    headers?: Record<string, string>,
  ): Promise<T>;
  delete<T>(path: string, headers?: Record<string, string>): Promise<T>;
}
