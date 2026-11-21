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

	it("exposes rawRequestWithMeta with real status, headers, and acceptedStatuses", async () => {
		fetchMock = vi
			.fn()
			.mockResolvedValueOnce({
				ok: false,
				status: 401,
				headers: new Headers({ "content-type": "application/json" }),
				json: async () => ({ error: { code: "auth.token_expired", message: "Expired" } }),
			})
			.mockResolvedValueOnce({
				ok: false,
				status: 304,
				headers: new Headers({ etag: '"refresh-etag"' }),
			})
			.mockResolvedValueOnce({
				ok: true,
				status: 200,
				headers: new Headers({ "content-type": "application/json" }),
				json: async () => ({ data: { id: "ok" } }),
			});

		let metaResult: unknown;
		const refresh = vi.fn(async ({ rawRequestWithMeta }) => {
			metaResult = await rawRequestWithMeta?.(
				{ method: "GET", path: "/auth/refresh-check" },
				{ acceptedStatuses: [304] },
			);
			return { success: true, accessToken: "new-token" };
		});

		const client = createHttpClient({
			baseUrl: "https://api.example.com",
			auth: { refresh },
			fetch: fetchMock,
		});

		await expect(client.get("/protected")).resolves.toEqual({ data: { id: "ok" } });
		expect(metaResult).toEqual({
			status: 304,
			headers: { etag: '"refresh-etag"' },
			body: null,
		});
	});

	it("writes rotated tokens once when concurrent waiters share a refresh", async () => {
		const tokenStore = new FakeTokenStore();
		tokenStore.seedTokens("old-token", "old-refresh-token");

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

		const refresh = vi.fn(async () => ({
			success: true as const,
			accessToken: "new-access-token",
			refreshToken: "new-refresh-token",
		}));

		const client = createHttpClient({
			baseUrl: "https://api.example.com",
			auth: { tokenStore, refresh },
			fetch: fetchMock,
		});

		await Promise.all([client.get("/one"), client.get("/two")]);

		expect(refresh).toHaveBeenCalledTimes(1);
		expect(tokenStore.setAccessToken).toHaveBeenCalledTimes(1);
		expect(tokenStore.setRefreshToken).toHaveBeenCalledTimes(1);
		tokenStore.expectTokens("new-access-token", "new-refresh-token");
	});

	it("retries a late 401 with the already-rotated token instead of refreshing again", async () => {
		const tokenStore = new FakeTokenStore();
		tokenStore.seedTokens("old-token", "old-refresh-token");

		const unauthorized = () => {
			const response = {
				ok: false,
				status: 401,
				headers: new Headers({ "content-type": "application/json" }),
				json: async () => ({ error: { code: "auth.token_expired", message: "Expired" } }),
			};
			return Promise.resolve(response);
		};
		const ok = (body: unknown) =>
			Promise.resolve({
				ok: true,
				status: 200,
				headers: new Headers({ "content-type": "application/json" }),
				json: async () => ({ data: body }),
			});

		let releaseSlowRequest: (() => void) | undefined;
		let markSlowStarted: (() => void) | undefined;
		const slowStarted = new Promise<void>((resolveStarted) => {
			markSlowStarted = resolveStarted;
		});

		fetchMock = vi.fn((url: string, init?: RequestInit) => {
			const { pathname } = new URL(url);
			const authorization = (init?.headers as Record<string, string> | undefined)?.Authorization;
			const usesOldToken = authorization === "Bearer old-token";

			if (pathname === "/slow") {
				if (!usesOldToken) return ok({ path: pathname });
				markSlowStarted?.();
				return new Promise((resolve) => {
					releaseSlowRequest = () => resolve(unauthorized());
				});
			}
			if (pathname === "/fast") {
				return usesOldToken ? unauthorized() : ok({ path: pathname });
			}
			if (pathname === "/auth/refresh") {
				return ok({ accessToken: "new-access-token", refreshToken: "new-refresh-token" });
			}
			return Promise.reject(new Error(`Unexpected request: ${pathname}`));
		});

		const refresh = vi.fn(async () => ({
			success: true as const,
			accessToken: "new-access-token",
			refreshToken: "new-refresh-token",
		}));

		const client = createHttpClient({
			baseUrl: "https://api.example.com",
			auth: { tokenStore, refresh },
			fetch: fetchMock,
		});

		// The slow request starts with the old token and is still in flight...
		const slow = client.get<{ data: { path: string } }>("/slow");
		await slowStarted;

		// ...while another request 401s, refreshes once, and retries with the new token.
		await client.get("/fast");
		expect(refresh).toHaveBeenCalledTimes(1);

		// The slow request's stale 401 lands after the rotation.
		releaseSlowRequest?.();

		await expect(slow).resolves.toEqual({ data: { path: "/slow" } });
		expect(refresh).toHaveBeenCalledTimes(1);
		expect(tokenStore.setAccessToken).toHaveBeenCalledTimes(1);

		const slowRetry = fetchMock.mock.calls[fetchMock.mock.calls.length - 1];
		expect((slowRetry[1] as RequestInit).headers).toMatchObject({
			Authorization: "Bearer new-access-token",
		});
	});
});
