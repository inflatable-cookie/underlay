import type {
  HttpMethod,
  HttpRequest,
  HttpRequestOptions,
  HttpResponse,
} from "../client/http";

export interface MockHttpCall {
  method: HttpMethod;
  path: string;
  body?: unknown;
  headers?: Record<string, string>;
  options?: HttpRequestOptions;
}

type MockResponder =
  | unknown
  | ((call: MockHttpCall) => unknown | Promise<unknown>);

export interface CreateMockHttpClientOptions {
  nextResponse?: unknown;
  responses?: Record<string, MockResponder>;
}

export interface MockHttpClient {
  calls: MockHttpCall[];
  nextResponse: unknown;
  responses: Map<string, MockResponder>;
  reset(): void;
  setNextResponse(response: unknown): void;
  setResponse(method: HttpMethod, path: string, response: MockResponder): void;
  request<T>(req: HttpRequest): Promise<T>;
  requestWithMeta<T>(
    req: HttpRequest,
    options?: HttpRequestOptions
  ): Promise<HttpResponse<T>>;
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

function responseKey(method: HttpMethod, path: string): string {
  return `${method} ${path}`;
}

function isMetaResponse<T>(value: unknown): value is HttpResponse<T> {
  if (typeof value !== "object" || value === null) return false;
  const candidate = value as Record<string, unknown>;
  return (
    typeof candidate.status === "number" &&
    typeof candidate.headers === "object" &&
    Object.prototype.hasOwnProperty.call(candidate, "body")
  );
}

async function resolveResponder(
  responder: MockResponder | undefined,
  call: MockHttpCall
): Promise<unknown> {
  if (typeof responder === "function") {
    return await responder(call);
  }
  return responder;
}

export function createMockHttpClient(
  options: CreateMockHttpClientOptions = {}
): MockHttpClient {
  const calls: MockHttpCall[] = [];
  const responses = new Map<string, MockResponder>(
    Object.entries(options.responses ?? {})
  );

  let nextResponse: unknown = options.nextResponse;

  async function resolveResponse(call: MockHttpCall): Promise<unknown> {
    const keyed = responses.get(responseKey(call.method, call.path));
    if (keyed !== undefined) {
      return await resolveResponder(keyed, call);
    }
    return await resolveResponder(nextResponse, call);
  }

  async function request<T>(req: HttpRequest): Promise<T> {
    const call: MockHttpCall = {
      method: req.method,
      path: req.path,
      body: req.body,
      headers: req.headers,
    };
    calls.push(call);
    return (await resolveResponse(call)) as T;
  }

  async function requestWithMeta<T>(
    req: HttpRequest,
    options?: HttpRequestOptions
  ): Promise<HttpResponse<T>> {
    const call: MockHttpCall = {
      method: req.method,
      path: req.path,
      body: req.body,
      headers: req.headers,
      options,
    };
    calls.push(call);
    const resolved = await resolveResponse(call);
    if (isMetaResponse<T>(resolved)) {
      return resolved;
    }
    return {
      status: 200,
      headers: {},
      body: resolved as T,
    };
  }

  return {
    calls,
    get nextResponse() {
      return nextResponse;
    },
    set nextResponse(value: unknown) {
      nextResponse = value;
    },
    responses,
    reset() {
      calls.length = 0;
      nextResponse = undefined;
      responses.clear();
    },
    setNextResponse(response: unknown) {
      nextResponse = response;
    },
    setResponse(method: HttpMethod, path: string, response: MockResponder) {
      responses.set(responseKey(method, path), response);
    },
    request,
    requestWithMeta,
    get<T>(path: string, headers?: Record<string, string>): Promise<T> {
      return request<T>({ method: "GET", path, headers });
    },
    getWithMeta<T>(
      path: string,
      headers?: Record<string, string>,
      options?: HttpRequestOptions
    ): Promise<HttpResponse<T>> {
      return requestWithMeta<T>({ method: "GET", path, headers }, options);
    },
    post<T>(path: string, body?: unknown, headers?: Record<string, string>): Promise<T> {
      return request<T>({ method: "POST", path, body, headers });
    },
    put<T>(path: string, body?: unknown, headers?: Record<string, string>): Promise<T> {
      return request<T>({ method: "PUT", path, body, headers });
    },
    patch<T>(path: string, body?: unknown, headers?: Record<string, string>): Promise<T> {
      return request<T>({ method: "PATCH", path, body, headers });
    },
    delete<T>(path: string, headers?: Record<string, string>): Promise<T> {
      return request<T>({ method: "DELETE", path, headers });
    },
  };
}
