import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { createHttpClient } from "../../../../src/client/http";
import { FakeTokenStore, mockFetchSequence } from "../../../utils/http-mocks";

describe("createHttpClient authentication refresh", () => {
  let fetchMock: ReturnType<typeof vi.fn>;

  beforeEach(() => {
    fetchMock = vi.fn();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("should refresh token on 401 and retry request", async () => {
    const tokenStore = new FakeTokenStore();
    tokenStore.seedTokens("old-token", "refresh-token");

    fetchMock = mockFetchSequence(
      {
        ok: false,
        status: 401,
        error: { code: "auth.token_expired", message: "Token expired" },
      },
      {
        ok: true,
        status: 200,
        data: {
          accessToken: "new-access-token",
          refreshToken: "new-refresh-token",
        },
      },
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
