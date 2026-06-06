import { describe, it, expect, beforeEach, vi, afterEach } from "vitest";
import { createHttpClient } from "../../../src/client/http";
import {
  mockFetchSuccess,
  mockFetchNoContent,
  getFetchCallArgs,
} from "../../utils/http-mocks";

describe("createHttpClient basic requests", () => {
  let fetchMock: ReturnType<typeof vi.fn>;

  beforeEach(() => {
    fetchMock = vi.fn();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("uses global fetch when no fetch implementation is provided", async () => {
    const globalFetch = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      headers: new Headers({ "content-type": "application/json" }),
      json: async () => ({ data: { id: "global" } }),
    });
    vi.stubGlobal("fetch", globalFetch);

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
    });

    await expect(client.get("/global")).resolves.toEqual({
      data: { id: "global" },
    });
    expect(globalFetch).toHaveBeenCalledTimes(1);
  });

  it("should make GET request with correct URL and headers", async () => {
    fetchMock = mockFetchSuccess({ id: "123", name: "Test" });

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      fetch: fetchMock,
    });

    const response = await client.get<{ id: string; name: string }>(
      "/users/123",
    );

    expect(fetchMock).toHaveBeenCalledTimes(1);
    const { url, method, headers } = getFetchCallArgs(fetchMock);
    expect(url).toBe("https://api.example.com/users/123");
    expect(method).toBe("GET");
    expect(headers.Accept).toBe("application/json");
    expect(response).toEqual({ data: { id: "123", name: "Test" } });
  });

  it("should make POST request with body", async () => {
    fetchMock = mockFetchSuccess({ id: "456", name: "Created" }, 201);

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      fetch: fetchMock,
    });

    const body = { name: "New User", email: "test@example.com" };
    const response = await client.post<{ id: string; name: string }>(
      "/users",
      body,
    );

    const {
      url,
      method,
      headers,
      body: sentBody,
    } = getFetchCallArgs(fetchMock);
    expect(url).toBe("https://api.example.com/users");
    expect(method).toBe("POST");
    expect(headers["Content-Type"]).toBe("application/json");
    expect(sentBody).toEqual(body);
    expect(response).toEqual({ data: { id: "456", name: "Created" } });
  });

  it("should handle 204 No Content responses", async () => {
    fetchMock = mockFetchNoContent();

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      fetch: fetchMock,
    });

    const response = await client.delete("/users/123");

    expect(response).toBeNull();
  });

  it("should add default headers to all requests", async () => {
    fetchMock = mockFetchSuccess({});

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      defaultHeaders: {
        "X-Client-Version": "1.0.0",
        "X-Custom-Header": "value",
      },
      fetch: fetchMock,
    });

    await client.get("/test");

    const { headers } = getFetchCallArgs(fetchMock);
    expect(headers["X-Client-Version"]).toBe("1.0.0");
    expect(headers["X-Custom-Header"]).toBe("value");
  });

  it("respects existing Accept header case-insensitively", async () => {
    fetchMock = mockFetchSuccess({});
    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      fetch: fetchMock,
    });

    await client.get("/custom-accept", { accept: "text/plain" });

    const { headers } = getFetchCallArgs(fetchMock);
    expect(headers.accept).toBe("text/plain");
    expect(headers.Accept).toBeUndefined();
  });

  it("should support PUT and PATCH helpers", async () => {
    fetchMock = mockFetchSuccess({ ok: true });

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      fetch: fetchMock,
    });

    await client.put("/users/123", { name: "Updated" });
    await client.patch("/users/123", { name: "Partial" });

    expect(fetchMock).toHaveBeenCalledTimes(2);
    expect(fetchMock.mock.calls[0]?.[1]?.method).toBe("PUT");
    expect(fetchMock.mock.calls[1]?.[1]?.method).toBe("PATCH");
  });
});
