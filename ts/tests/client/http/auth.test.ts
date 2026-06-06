import { describe, it, expect, beforeEach, vi, afterEach } from "vitest";
import { createHttpClient } from "../../../src/client/http";
import { UnderlayHttpError } from "../../../src/client/errors";
import {
  mockFetchSuccess,
  mockFetchSequence,
  FakeTokenStore,
  getFetchCallArgs,
  expectAuthHeader,
  expectNoAuthHeader,
} from "../../utils/http-mocks";

describe("createHttpClient authentication", () => {
  let fetchMock: ReturnType<typeof vi.fn>;

  beforeEach(() => {
    fetchMock = vi.fn();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("should include access token in Authorization header", async () => {
    fetchMock = mockFetchSuccess({});

    const tokenStore = new FakeTokenStore();
    tokenStore.seedTokens("access-token-123", "refresh-token");

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      auth: { tokenStore },
      fetch: fetchMock,
    });

    await client.get("/protected");

    expectAuthHeader(fetchMock, "access-token-123");
  });

  it("should not include Authorization header when no token", async () => {
    fetchMock = mockFetchSuccess({});

    const tokenStore = new FakeTokenStore();

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      auth: { tokenStore },
      fetch: fetchMock,
    });

    await client.get("/public");

    expectNoAuthHeader(fetchMock);
  });

  it("should refresh token on 401 and retry request", async () => {
    const tokenStore = new FakeTokenStore();
    tokenStore.seedTokens("old-token", "refresh-token");

    fetchMock = mockFetchSequence(
      // First call: 401 with old token
      {
        ok: false,
        status: 401,
        error: { code: "auth.token_expired", message: "Token expired" },
      },
      // Refresh call: returns new tokens
      {
        ok: true,
        status: 200,
        data: {
          accessToken: "new-access-token",
          refreshToken: "new-refresh-token",
        },
      },
      // Retry call: succeeds with new token
      { ok: true, status: 200, data: { id: "123" } },
    );

    const refresh = vi.fn(async ({ rawRequest, getRefreshToken }) => {
      const refreshToken = await getRefreshToken();
      if (!refreshToken) return { success: false };

      const response = await rawRequest<{
        data: { accessToken: string; refreshToken: string };
      }>({
        method: "POST",
        path: "/auth/refresh",
        body: { refreshToken },
      });

      return {
        success: true,
        accessToken: response.data.accessToken,
        refreshToken: response.data.refreshToken,
      };
    });

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      auth: { tokenStore, refresh },
      fetch: fetchMock,
    });

    const result = await client.get<{ id: string }>("/protected");

    expect(refresh).toHaveBeenCalledTimes(1);
    expect(fetchMock).toHaveBeenCalledTimes(3);
    tokenStore.expectTokens("new-access-token", "new-refresh-token");
    expect(result).toEqual({ data: { id: "123" } });
  });

  it("should clear tokens and throw error if refresh fails", async () => {
    const tokenStore = new FakeTokenStore();
    tokenStore.seedTokens("old-token", "invalid-refresh-token");

    fetchMock = mockFetchSequence(
      // First call: 401
      {
        ok: false,
        status: 401,
        error: { code: "auth.token_expired", message: "Token expired" },
      },
      // Refresh call: fails
      {
        ok: false,
        status: 401,
        error: {
          code: "auth.refresh_invalid",
          message: "Invalid refresh token",
        },
      },
    );

    const refresh = vi.fn(async () => ({ success: false }));

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      auth: { tokenStore, refresh },
      fetch: fetchMock,
    });

    await expect(client.get("/protected")).rejects.toThrow(UnderlayHttpError);

    tokenStore.expectTokens(null, null);
  });

  it("rethrows 401 errors when no refresh handler is configured", async () => {
    const tokenStore = new FakeTokenStore();
    tokenStore.seedTokens("expired-token", "refresh-token");
    fetchMock = mockFetchSequence({
      ok: false,
      status: 401,
      error: { code: "auth.token_expired", message: "Token expired" },
    });

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      auth: { tokenStore },
      fetch: fetchMock,
    });

    await expect(client.get("/protected")).rejects.toThrow(UnderlayHttpError);
    expect(fetchMock).toHaveBeenCalledTimes(1);
  });

  it("does not call refresh for non-401 auth errors", async () => {
    fetchMock = mockFetchSequence({
      ok: false,
      status: 403,
      error: { code: "auth.forbidden", message: "Forbidden" },
    });
    const refresh = vi.fn(async () => ({ success: true }));
    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      auth: { refresh },
      fetch: fetchMock,
    });

    await expect(client.get("/protected")).rejects.toThrow(UnderlayHttpError);
    expect(refresh).not.toHaveBeenCalled();
  });

  it("retries refresh-success requests even without token providers", async () => {
    fetchMock = mockFetchSequence(
      {
        ok: false,
        status: 401,
        error: { code: "auth.token_expired", message: "Expired" },
      },
      { ok: true, status: 200, data: { id: "123" } },
    );

    const refresh = vi.fn(async () => ({ success: true }));
    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      auth: { refresh },
      fetch: fetchMock,
    });

    const result = await client.get<{ id: string }>("/protected");
    expect(result).toEqual({ data: { id: "123" } });
    expect(refresh).toHaveBeenCalledTimes(1);
  });

  it("provides null refresh token when no refresh source exists", async () => {
    fetchMock = mockFetchSequence(
      {
        ok: false,
        status: 401,
        error: { code: "auth.token_expired", message: "Expired" },
      },
      { ok: true, status: 200, data: { id: "123" } },
    );

    const observed: Array<string | null> = [];
    const refresh = vi.fn(async ({ getRefreshToken }) => {
      observed.push(await getRefreshToken());
      return { success: true };
    });
    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      auth: { refresh },
      fetch: fetchMock,
    });

    await client.get("/protected");

    expect(observed).toEqual([null]);
  });

  it("should not override explicit Authorization headers", async () => {
    fetchMock = mockFetchSuccess({ ok: true });

    const tokenStore = new FakeTokenStore();
    tokenStore.seedTokens("store-access-token", "store-refresh-token");

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      auth: { tokenStore },
      fetch: fetchMock,
    });

    await client.get("/protected", { Authorization: "Bearer explicit-token" });

    const { headers } = getFetchCallArgs(fetchMock);
    expect(headers.Authorization).toBe("Bearer explicit-token");
  });

  it("passes refresh-context token setters through to token store", async () => {
    const tokenStore = new FakeTokenStore();
    tokenStore.seedTokens("expired-token", "refresh-token");

    fetchMock = mockFetchSequence(
      {
        ok: false,
        status: 401,
        error: { code: "auth.token_expired", message: "Token expired" },
      },
      { ok: true, status: 200, data: { id: "123" } },
    );

    const refresh = vi.fn(async ({ setAccessToken, setRefreshToken }) => {
      await setAccessToken("fresh-access-token");
      await setRefreshToken("fresh-refresh-token");
      return { success: true };
    });

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      auth: { tokenStore, refresh },
      fetch: fetchMock,
    });

    await client.get("/protected");

    tokenStore.expectTokens("fresh-access-token", "fresh-refresh-token");
    expect(refresh).toHaveBeenCalledTimes(1);
  });

  it("rethrows non-http errors from refresh handler", async () => {
    fetchMock = mockFetchSequence({
      ok: false,
      status: 401,
      error: { code: "auth.token_expired", message: "Expired" },
    });
    const refresh = vi.fn(async () => {
      throw new Error("refresh exploded");
    });
    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      auth: { refresh },
      fetch: fetchMock,
    });

    await expect(client.get("/protected")).rejects.toThrow("refresh exploded");
    expect(refresh).toHaveBeenCalledTimes(1);
  });

  it("shares in-flight refresh across concurrent 401 requests", async () => {
    fetchMock = mockFetchSequence(
      {
        ok: false,
        status: 401,
        error: { code: "auth.token_expired", message: "Expired" },
      },
      {
        ok: false,
        status: 401,
        error: { code: "auth.token_expired", message: "Expired" },
      },
      { ok: true, status: 200, data: { id: "a" } },
      { ok: true, status: 200, data: { id: "b" } },
    );

    const refresh = vi.fn(
      () =>
        new Promise<{ success: true }>((resolve) => {
          setTimeout(() => resolve({ success: true }), 0);
        }),
    );

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      auth: { refresh },
      fetch: fetchMock,
    });

    const [a, b] = await Promise.all([
      client.get<{ id: string }>("/one"),
      client.get<{ id: string }>("/two"),
    ]);

    expect(a).toEqual({ data: { id: "a" } });
    expect(b).toEqual({ data: { id: "b" } });
    expect(refresh).toHaveBeenCalledTimes(1);
  });
});
