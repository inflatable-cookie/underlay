import { describe, it, expect, beforeEach, vi, afterEach } from "vitest";
import { createHttpClient } from "../../../src/client/http";
import { UnderlayHttpError } from "../../../src/client/errors";
import {
  mockFetchError,
  mockFetchSequence,
  mockFetchWithDelay,
} from "../../utils/http-mocks";

describe("createHttpClient retry and timeout", () => {
  let fetchMock: ReturnType<typeof vi.fn>;

  beforeEach(() => {
    fetchMock = vi.fn();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("should retry on 502/503/504 for GET requests", async () => {
    fetchMock = mockFetchSequence(
      {
        ok: false,
        status: 503,
        error: { code: "server.unavailable", message: "Service unavailable" },
      },
      {
        ok: false,
        status: 503,
        error: { code: "server.unavailable", message: "Service unavailable" },
      },
      { ok: true, status: 200, data: { id: "123" } },
    );

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      maxRetries: 3,
      fetch: fetchMock,
    });

    const result = await client.get<{ id: string }>("/resource");

    expect(fetchMock).toHaveBeenCalledTimes(3);
    expect(result).toEqual({ data: { id: "123" } });
  });

  it("should not retry on 502/503/504 for POST requests", async () => {
    fetchMock = mockFetchError(
      "server.unavailable",
      "Service unavailable",
      503,
    );

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      maxRetries: 3,
      fetch: fetchMock,
    });

    await expect(client.post("/resource", {})).rejects.toThrow(
      UnderlayHttpError,
    );

    expect(fetchMock).toHaveBeenCalledTimes(1); // No retries for POST
  });

  it("should retry on custom retry statuses", async () => {
    fetchMock = mockFetchSequence(
      {
        ok: false,
        status: 429,
        error: { code: "rate_limit", message: "Too many requests" },
      },
      { ok: true, status: 200, data: { id: "123" } },
    );

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      maxRetries: 3,
      retryStatuses: [429],
      fetch: fetchMock,
    });

    const result = await client.get<{ id: string }>("/resource");

    expect(fetchMock).toHaveBeenCalledTimes(2);
    expect(result).toEqual({ data: { id: "123" } });
  });

  it("should respect maxRetries limit", async () => {
    // Mock will return 503 indefinitely
    fetchMock = vi.fn().mockResolvedValue({
      ok: false,
      status: 503,
      json: async () => ({
        error: { code: "server.unavailable", message: "Service unavailable" },
      }),
    });

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      maxRetries: 2,
      fetch: fetchMock,
    });

    await expect(client.get("/resource")).rejects.toThrow(UnderlayHttpError);

    expect(fetchMock).toHaveBeenCalledTimes(3); // Original + 2 retries
  });

  describe("timeout", () => {
    it("should timeout GET requests after specified time", async () => {
      vi.useFakeTimers();
      fetchMock = vi.fn().mockImplementation((_url, init?: RequestInit) => {
        return new Promise((_resolve, reject) => {
          init?.signal?.addEventListener("abort", () =>
            reject(new Error("Aborted")),
          );
        });
      });

      const client = createHttpClient({
        baseUrl: "https://api.example.com",
        timeoutMs: 5000, // 5 seconds
        fetch: fetchMock,
      });

      const promise = client.get("/slow-endpoint");
      const rejection = expect(promise).rejects.toMatchObject({
        status: 0,
        message: "Aborted",
      });

      // Advance timers to trigger timeout
      await vi.advanceTimersByTimeAsync(5000);

      await rejection;
      vi.useRealTimers();
    });

    it.skip("should not timeout POST requests", async () => {
      fetchMock = mockFetchWithDelay({ id: "123" }, 10000);

      const client = createHttpClient({
        baseUrl: "https://api.example.com",
        timeoutMs: 5000,
        fetch: fetchMock,
      });

      const promise = client.post("/upload", { large: "data" });

      // Advance timers past timeout
      await vi.advanceTimersByTimeAsync(10000);

      const result = await promise;
      expect(result).toEqual({ data: { id: "123" } });
    });
  });
});
