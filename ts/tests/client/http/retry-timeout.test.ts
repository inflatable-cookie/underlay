import { describe, it, expect, beforeEach, vi, afterEach } from "vitest";
import { createHttpClient } from "../../../src/client/http";
import { UnderlayHttpError } from "../../../src/client/errors";
import { mockFetchError, mockFetchSequence, mockFetchWithDelay } from "../../utils/http-mocks";

describe("createHttpClient retry and timeout", () => {
	let fetchMock: ReturnType<typeof vi.fn>;

	beforeEach(() => {
		fetchMock = vi.fn();
	});

	afterEach(() => {
		vi.restoreAllMocks();
		vi.useRealTimers();
	});

	it("rejects a pre-aborted request without fetching", async () => {
		const client = createHttpClient({ baseUrl: "https://api.example.com", fetch: fetchMock });

		await expect(
			client.getWithMeta("/endpoint", undefined, { signal: AbortSignal.abort() }),
		).rejects.toMatchObject({ status: 0, envelope: { error: { code: "aborted" } } });
		expect(fetchMock).not.toHaveBeenCalled();
	});

	it.each(["caller", "timeout"])("propagates %s cancellation to fetch", async (source) => {
		vi.useFakeTimers();
		const caller = new AbortController();
		let fetchSignal: AbortSignal | null | undefined;
		fetchMock.mockImplementation((_url, init: RequestInit) => {
			fetchSignal = init.signal;
			return new Promise((_resolve, reject) => {
				fetchSignal?.addEventListener("abort", () => reject(fetchSignal?.reason));
			});
		});
		const client = createHttpClient({
			baseUrl: "https://api.example.com",
			fetch: fetchMock,
			timeoutMs: 5000,
		});
		const pending = client.getWithMeta("/endpoint", undefined, { signal: caller.signal });
		const rejection = expect(pending).rejects.toMatchObject({
			status: 0,
			envelope: { error: { code: source === "caller" ? "aborted" : "timeout" } },
		});
		await vi.advanceTimersByTimeAsync(0);
		if (source === "caller") caller.abort(new Error("user cancelled"));
		else await vi.advanceTimersByTimeAsync(5000);
		await rejection;

		expect(fetchSignal?.aborted).toBe(true);
		expect(caller.signal.aborted).toBe(source === "caller");
		expect(vi.getTimerCount()).toBe(0);
	});

	it("cancels a pending retry backoff", async () => {
		vi.useFakeTimers();
		fetchMock.mockImplementation(async () => new Response(null, { status: 503 }));
		const caller = new AbortController();
		const client = createHttpClient({ baseUrl: "https://api.example.com", fetch: fetchMock });
		const pending = client.getWithMeta("/endpoint", undefined, { signal: caller.signal });
		const rejection = expect(pending).rejects.toMatchObject({
			status: 0,
			envelope: { error: { code: "aborted" } },
		});
		await vi.advanceTimersByTimeAsync(0);
		expect(vi.getTimerCount()).toBe(1);
		caller.abort();
		await rejection;

		expect(vi.getTimerCount()).toBe(0);
		expect(fetchMock).toHaveBeenCalledOnce();
	});

	it("cancels a caller waiting for refresh while keeping refresh available to other requests", async () => {
		const caller = new AbortController();
		const startedRefreshing = Promise.withResolvers<void>();
		const finishRefreshing = Promise.withResolvers<void>();
		const refresh = vi.fn(async () => {
			startedRefreshing.resolve();
			await finishRefreshing.promise;
			return { success: true };
		});
		fetchMock
			.mockResolvedValueOnce(new Response(null, { status: 401 }))
			.mockResolvedValueOnce(new Response(null, { status: 401 }))
			.mockResolvedValueOnce(new Response(null, { status: 204 }));
		const client = createHttpClient({
			baseUrl: "https://api.example.com",
			fetch: fetchMock,
			auth: { refresh },
		});

		const pending = client.getWithMeta("/endpoint", undefined, { signal: caller.signal });
		const rejection = expect(pending).rejects.toMatchObject({
			status: 0,
			envelope: { error: { code: "aborted" } },
		});
		await startedRefreshing.promise;
		caller.abort();
		await rejection;

		const second = client.getWithMeta("/endpoint");
		await vi.waitFor(() => expect(fetchMock).toHaveBeenCalledTimes(2));
		finishRefreshing.resolve();
		await expect(second).resolves.toMatchObject({ status: 204 });
		expect(refresh).toHaveBeenCalledOnce();
		expect(fetchMock).toHaveBeenCalledTimes(3);
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
		fetchMock = mockFetchError("server.unavailable", "Service unavailable", 503);

		const client = createHttpClient({
			baseUrl: "https://api.example.com",
			maxRetries: 3,
			fetch: fetchMock,
		});

		await expect(client.post("/resource", {})).rejects.toThrow(UnderlayHttpError);

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
					init?.signal?.addEventListener("abort", () => reject(new Error("Aborted")));
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
				message: "Request timed out",
				envelope: { error: { code: "timeout" } },
			});

			// Advance timers to trigger timeout
			await vi.advanceTimersByTimeAsync(5000);

			await rejection;
			vi.useRealTimers();
		});

		it("should not timeout POST requests", async () => {
			vi.useFakeTimers();
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
			vi.useRealTimers();
		});

		it("times out stalled res.json() response body reads", async () => {
			vi.useFakeTimers();
			fetchMock = vi.fn().mockImplementation((_url, init?: RequestInit) => {
				return Promise.resolve({
					ok: true,
					status: 200,
					headers: new Headers({ "content-type": "application/json" }),
					json: () =>
						new Promise((_resolve, reject) => {
							init?.signal?.addEventListener("abort", () =>
								reject(new DOMException("Aborted", "AbortError")),
							);
						}),
				});
			});

			const client = createHttpClient({
				baseUrl: "https://api.example.com",
				timeoutMs: 2500,
				fetch: fetchMock,
			});

			const promise = client.get("/stalled-json");
			const rejection = expect(promise).rejects.toMatchObject({
				status: 0,
				message: "Request timed out",
				envelope: { error: { code: "timeout" } },
			});

			await vi.advanceTimersByTimeAsync(2500);
			await rejection;
			vi.useRealTimers();
		});

		it("times out stalled res.text() response body reads", async () => {
			vi.useFakeTimers();
			fetchMock = vi.fn().mockImplementation((_url, init?: RequestInit) => {
				return Promise.resolve({
					ok: true,
					status: 200,
					headers: new Headers({ "content-type": "text/plain" }),
					text: () =>
						new Promise((_resolve, reject) => {
							init?.signal?.addEventListener("abort", () =>
								reject(new DOMException("Aborted", "AbortError")),
							);
						}),
				});
			});

			const client = createHttpClient({
				baseUrl: "https://api.example.com",
				timeoutMs: 2500,
				fetch: fetchMock,
			});

			const promise = client.get("/stalled-text");
			const rejection = expect(promise).rejects.toMatchObject({
				status: 0,
				message: "Request timed out",
				envelope: { error: { code: "timeout" } },
			});

			await vi.advanceTimersByTimeAsync(2500);
			await rejection;
			vi.useRealTimers();
		});
	});
});
