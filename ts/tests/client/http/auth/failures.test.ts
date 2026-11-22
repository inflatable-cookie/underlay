import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { UnderlayHttpError } from "../../../../src/client/errors";
import { createHttpClient } from "../../../../src/client/http";
import { FakeTokenStore, mockFetchSequence } from "../../../utils/http-mocks";

describe("createHttpClient authentication failures", () => {
	let fetchMock: ReturnType<typeof vi.fn>;

	beforeEach(() => {
		fetchMock = vi.fn();
	});

	it.each(["https://cdn.example.com/private", "//cdn.example.com/private"])(
		"keeps base-origin credentials when %s returns 401",
		async (path) => {
			const tokenStore = new FakeTokenStore();
			tokenStore.seedTokens("access-token", "refresh-token");
			const refresh = vi.fn(async () => ({ success: false }));
			fetchMock = vi.fn(async () => new Response(null, { status: 401 }));
			const client = createHttpClient({
				baseUrl: "https://api.example.com",
				auth: { tokenStore, refresh },
				fetch: fetchMock,
			});

			await expect(client.get(path)).rejects.toMatchObject({ status: 401 });
			expect(refresh).not.toHaveBeenCalled();
			expect(fetchMock).toHaveBeenCalledOnce();
			tokenStore.expectTokens("access-token", "refresh-token");
		},
	);

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

	it("clears tokens and rethrows when refresh handler throws", async () => {
		const tokenStore = new FakeTokenStore();
		tokenStore.seedTokens("old-token", "refresh-token");

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
			auth: { tokenStore, refresh },
			fetch: fetchMock,
		});

		await expect(client.get("/protected")).rejects.toThrow("refresh exploded");
		expect(refresh).toHaveBeenCalledTimes(1);
		tokenStore.expectTokens(null, null);
	});

	it("clears tokenStore once without redundant setAccessToken(null)/setRefreshToken(null) calls", async () => {
		const tokenStore = new FakeTokenStore();
		tokenStore.seedTokens("old-token", "invalid-refresh-token");

		fetchMock = mockFetchSequence({
			ok: false,
			status: 401,
			error: { code: "auth.token_expired", message: "Token expired" },
		});

		const refresh = vi.fn(async () => ({ success: false }));
		const client = createHttpClient({
			baseUrl: "https://api.example.com",
			auth: { tokenStore, refresh },
			fetch: fetchMock,
		});

		await expect(client.get("/protected")).rejects.toThrow(UnderlayHttpError);
		expect(tokenStore.clear).toHaveBeenCalledTimes(1);
		expect(tokenStore.setAccessToken).not.toHaveBeenCalled();
		expect(tokenStore.setRefreshToken).not.toHaveBeenCalled();
	});

	it("does not auto-refresh or retry when caller supplied an explicit Authorization header", async () => {
		const tokenStore = new FakeTokenStore();
		tokenStore.seedTokens("store-access", "store-refresh");

		fetchMock = mockFetchSequence({
			ok: false,
			status: 401,
			error: { code: "auth.invalid_token", message: "Invalid explicit token" },
		});

		const refresh = vi.fn(async () => ({
			success: true,
			accessToken: "new-access",
		}));
		const client = createHttpClient({
			baseUrl: "https://api.example.com",
			auth: { tokenStore, refresh },
			fetch: fetchMock,
		});

		await expect(
			client.get("/protected", { authorization: "Bearer caller-token" }),
		).rejects.toMatchObject({ status: 401 });

		expect(refresh).not.toHaveBeenCalled();
		expect(fetchMock).toHaveBeenCalledTimes(1);
		tokenStore.expectTokens("store-access", "store-refresh");
	});

	it("does not auto-refresh or retry when Authorization is supplied via defaultHeaders", async () => {
		const tokenStore = new FakeTokenStore();
		tokenStore.seedTokens("store-access", "store-refresh");

		fetchMock = mockFetchSequence({
			ok: false,
			status: 401,
			error: { code: "auth.invalid_token", message: "Invalid default token" },
		});

		const refresh = vi.fn(async () => ({
			success: true,
			accessToken: "new-access",
		}));
		const client = createHttpClient({
			baseUrl: "https://api.example.com",
			defaultHeaders: { Authorization: "Bearer caller-default" },
			auth: { tokenStore, refresh },
			fetch: fetchMock,
		});

		await expect(client.get("/protected")).rejects.toMatchObject({
			status: 401,
		});

		expect(refresh).not.toHaveBeenCalled();
		expect(fetchMock).toHaveBeenCalledTimes(1);
		expect(fetchMock.mock.calls[0]?.[1]?.headers).toMatchObject({
			Authorization: "Bearer caller-default",
		});
		tokenStore.expectTokens("store-access", "store-refresh");
	});

	it("preserves stored tokens when refresh throws a transient 5xx or network UnderlayHttpError", async () => {
		const tokenStore = new FakeTokenStore();
		tokenStore.seedTokens("old-token", "refresh-token");

		fetchMock = mockFetchSequence({
			ok: false,
			status: 401,
			error: { code: "auth.token_expired", message: "Expired" },
		});

		const transientError = new UnderlayHttpError(503, "Service Unavailable");
		const refresh = vi.fn(async () => {
			throw transientError;
		});
		const client = createHttpClient({
			baseUrl: "https://api.example.com",
			auth: { tokenStore, refresh },
			fetch: fetchMock,
		});

		await expect(client.get("/protected")).rejects.toBe(transientError);
		expect(tokenStore.clear).not.toHaveBeenCalled();
		tokenStore.expectTokens("old-token", "refresh-token");
	});
});
