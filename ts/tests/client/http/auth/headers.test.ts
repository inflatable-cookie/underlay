import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { createHttpClient } from "../../../../src/client/http";
import {
  expectAuthHeader,
  expectNoAuthHeader,
  FakeTokenStore,
  getFetchCallArgs,
  mockFetchSuccess,
} from "../../../utils/http-mocks";

describe("createHttpClient authentication headers", () => {
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
});
