import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { UnderlayHttpError } from "../../../../src/client/errors";
import { createHttpClient } from "../../../../src/client/http";
import { FakeTokenStore, mockFetchSequence } from "../../../utils/http-mocks";

describe("createHttpClient authentication failures", () => {
  let fetchMock: ReturnType<typeof vi.fn>;

  beforeEach(() => {
    fetchMock = vi.fn();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("should clear tokens and throw error if refresh fails", async () => {
    const tokenStore = new FakeTokenStore();
    tokenStore.seedTokens("old-token", "invalid-refresh-token");

    fetchMock = mockFetchSequence(
      {
        ok: false,
        status: 401,
        error: {
          code: "auth.token_expired",
          message: "Token expired",
        },
      },
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
});
