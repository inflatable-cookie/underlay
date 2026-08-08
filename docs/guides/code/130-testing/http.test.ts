import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { createHttpClient } from "@inflatable-cookie/underlay/client/http";
import { UnderlayHttpError } from "@inflatable-cookie/underlay/client/errors";

describe("underlay createHttpClient", () => {
  let fetchMock: ReturnType<typeof vi.fn>;

  beforeEach(() => {
    fetchMock = vi.fn();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("makes GET requests", async () => {
    fetchMock.mockResolvedValue(
      new Response(JSON.stringify({ data: { test: true } }), {
        status: 200,
        headers: { "content-type": "application/json" },
      })
    );

    const http = createHttpClient({
      baseUrl: "http://localhost:3000/v1/",
      fetch: fetchMock,
    });

    const result = await http.get<{ data: { test: boolean } }>("/test");
    expect(result.data.test).toBe(true);

    expect(fetchMock).toHaveBeenCalledTimes(1);
    const [url, requestInit] = fetchMock.mock.calls[0];
    expect(String(url)).toContain("/test");
    expect(requestInit).toEqual(expect.objectContaining({ method: "GET" }));
  });

  it("throws UnderlayHttpError with envelope on JSON error", async () => {
    fetchMock.mockResolvedValue(
      new Response(
        JSON.stringify({
          error: { code: "test.error", message: "Test error" },
        }),
        {
          status: 400,
          headers: { "content-type": "application/json" },
        }
      )
    );

    const http = createHttpClient({
      baseUrl: "http://localhost:3000/v1/",
      fetch: fetchMock,
    });

    try {
      await http.get("/test");
      throw new Error("expected request to throw");
    } catch (err) {
      expect(err).toBeInstanceOf(UnderlayHttpError);
      const underlayErr = err as UnderlayHttpError;
      expect(underlayErr.status).toBe(400);
      expect(underlayErr.envelope?.error.code).toBe("test.error");
    }
  });
});
