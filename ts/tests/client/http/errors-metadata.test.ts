import { describe, it, expect, beforeEach, vi, afterEach } from "vitest";
import { createHttpClient } from "../../../src/client/http";
import { UnderlayHttpError } from "../../../src/client/errors";
import { mockFetchSuccess, mockFetchError } from "../../utils/http-mocks";

describe("createHttpClient errors, debug, and metadata", () => {
  let fetchMock: ReturnType<typeof vi.fn>;

  beforeEach(() => {
    fetchMock = vi.fn();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("should throw UnderlayHttpError with error envelope", async () => {
    fetchMock = mockFetchError("validation.failed", "Validation failed", 400, {
      email: "Invalid email",
    });

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      fetch: fetchMock,
    });

    try {
      await client.post("/users", { email: "invalid" });
      expect.fail("Should have thrown error");
    } catch (error) {
      expect(error).toBeInstanceOf(UnderlayHttpError);
      const httpError = error as UnderlayHttpError;
      expect(httpError.status).toBe(400);
      expect(httpError.code).toBe("validation.failed");
      expect(httpError.message).toBe("Validation failed");
      expect(httpError.fieldErrors).toEqual({ email: "Invalid email" });
    }
  });

  it("should throw UnderlayHttpError for network errors", async () => {
    fetchMock.mockRejectedValueOnce(new Error("Network error"));

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      fetch: fetchMock,
    });

    await expect(client.get("/resource")).rejects.toThrow(UnderlayHttpError);
  });

  it("rethrows non-http errors from requestWithMeta setup", async () => {
    const client = createHttpClient({
      baseUrl: "not-a-valid-url",
      fetch: fetchMock,
    });

    await expect(client.get("/resource")).rejects.toBeInstanceOf(TypeError);
  });

  it("should map non-Error throws to a generic network message", async () => {
    fetchMock.mockRejectedValueOnce("boom");

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      fetch: fetchMock,
    });

    await expect(client.get("/resource")).rejects.toMatchObject({
      status: 0,
      message: "Network error",
    });
  });

  it("falls back to HTTP status message when error json parsing fails", async () => {
    fetchMock = vi.fn().mockResolvedValueOnce({
      ok: false,
      status: 502,
      headers: new Headers({
        "content-type": "application/json",
      }),
      json: async () => {
        throw new Error("Invalid JSON");
      },
    });

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      maxRetries: 0,
      fetch: fetchMock,
    });

    await expect(client.get("/resource")).rejects.toMatchObject({
      status: 502,
      message: "HTTP 502",
    });
  });

  it("falls back to HTTP status message for non-json error responses", async () => {
    fetchMock = vi.fn().mockResolvedValueOnce({
      ok: false,
      status: 500,
      headers: new Headers({
        "content-type": "text/plain",
      }),
      text: async () => "no-json",
    });

    const client = createHttpClient({
      baseUrl: "https://api.example.com",
      maxRetries: 0,
      fetch: fetchMock,
    });

    await expect(client.get("/resource")).rejects.toMatchObject({
      status: 500,
      message: "HTTP 500",
    });
  });

  describe("debug logging", () => {
    it("should log requests when debug is enabled", async () => {
      const consoleSpy = vi.spyOn(console, "log").mockImplementation(() => {});
      fetchMock = mockFetchSuccess({});

      const client = createHttpClient({
        baseUrl: "https://api.example.com",
        debug: true,
        fetch: fetchMock,
      });

      await client.get("/test");

      expect(consoleSpy).toHaveBeenCalledWith(
        expect.stringContaining("[HTTP]"),
        expect.stringContaining("GET"),
        expect.stringContaining("/test"),
      );

      consoleSpy.mockRestore();
    });
  });

  describe("response metadata", () => {
    it("should return status, headers, and body from getWithMeta", async () => {
      fetchMock = vi.fn().mockResolvedValueOnce({
        ok: true,
        status: 200,
        headers: new Headers({
          "content-type": "application/json",
          etag: 'W/"abc123"',
        }),
        json: async () => ({ data: { id: "123" } }),
      });

      const client = createHttpClient({
        baseUrl: "https://api.example.com",
        fetch: fetchMock,
      });

      const response = await client.getWithMeta<{ data: { id: string } }>(
        "/resource",
      );
      expect(response.status).toBe(200);
      expect(response.headers.etag).toBe('W/"abc123"');
      expect(response.body).toEqual({ data: { id: "123" } });
    });

    it("should allow accepted 304 responses without throwing", async () => {
      fetchMock = vi.fn().mockResolvedValueOnce({
        ok: false,
        status: 304,
        headers: new Headers({
          etag: 'W/"abc123"',
        }),
        json: async () => {
          throw new Error("Not modified");
        },
      });

      const client = createHttpClient({
        baseUrl: "https://api.example.com",
        fetch: fetchMock,
      });

      const response = await client.getWithMeta("/resource", undefined, {
        acceptedStatuses: [304],
      });
      expect(response.status).toBe(304);
      expect(response.headers.etag).toBe('W/"abc123"');
      expect(response.body).toBeNull();
    });

    it("should parse text responses when content-type is non-json", async () => {
      fetchMock = vi.fn().mockResolvedValueOnce({
        ok: true,
        status: 200,
        headers: new Headers({
          "content-type": "text/plain",
        }),
        text: async () => "plain-text-response",
      });

      const client = createHttpClient({
        baseUrl: "https://api.example.com",
        fetch: fetchMock,
      });

      const response = await client.getWithMeta<string>("/resource");
      expect(response.status).toBe(200);
      expect(response.body).toBe("plain-text-response");
    });

    it("returns empty headers when response headers are missing iterable APIs", async () => {
      fetchMock = vi.fn().mockResolvedValueOnce({
        ok: true,
        status: 200,
        headers: {},
        json: async () => ({ data: true }),
      });

      const client = createHttpClient({
        baseUrl: "https://api.example.com",
        fetch: fetchMock,
      });

      const response = await client.getWithMeta<{ data: boolean }>("/resource");
      expect(response.headers).toEqual({});
      expect(response.body).toEqual({ data: true });
    });
  });
});
