import { beforeEach, describe, expect, it, vi } from "vitest";
import { UnderlayHttpError } from "../../../src/client/errors";
import { authRoutes, createCookiesMock } from "./fixtures";

const mocks = vi.hoisted(() => ({
	createHttpClient: vi.fn(),
	createAuthCommands: vi.fn(),
}));

vi.mock("../../../src/client/http", () => ({
	createHttpClient: mocks.createHttpClient,
}));

vi.mock("../../../src/client/auth", () => ({
	createAuthCommands: mocks.createAuthCommands,
}));

describe("client/sveltekit refresh adapter", () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	it("uses default refresh request through auth refresh adapter", async () => {
		const { createAuthHandle } = await import("../../../src/client/sveltekit");

		mocks.createHttpClient.mockReturnValue({});
		mocks.createAuthCommands.mockReturnValue({ session: vi.fn().mockResolvedValue({ id: "ok" }) });

		const event = {
			cookies: createCookiesMock(),
			fetch: vi.fn(),
			locals: {},
			url: new URL("https://example.com/private"),
		} as any;
		const resolve = vi.fn(async () => new Response("ok", { status: 200 }));

		const handle = createAuthHandle({
			baseUrl: "https://api.example.com",
			routes: authRoutes,
			cookies: { accessTokenCookie: "access", refreshTokenCookie: "refresh" },
		});

		await handle({ event, resolve } as any);
		const cfg = mocks.createHttpClient.mock.calls[0][0];
		const refresh = cfg.auth.refresh as (ctx: {
			rawRequest: (req: any) => Promise<any>;
			getRefreshToken: () => Promise<string | null>;
		}) => Promise<any>;

		const rawRequest = vi
			.fn()
			.mockResolvedValueOnce({ data: { accessToken: "a2", refreshToken: "r2" } })
			.mockRejectedValueOnce(new Error("boom"));

		await expect(
			refresh({
				rawRequest,
				getRefreshToken: vi.fn().mockResolvedValue("rt"),
			})
		).resolves.toEqual({
			success: true,
			accessToken: "a2",
			refreshToken: "r2",
		});
		expect(rawRequest).toHaveBeenNthCalledWith(1, {
			method: "POST",
			path: "/refresh",
			body: { refreshToken: "rt" },
			headers: undefined,
		});

		await expect(
			refresh({
				rawRequest,
				getRefreshToken: vi.fn().mockResolvedValue(null),
			})
		).resolves.toEqual({
			success: false,
			accessToken: null,
			refreshToken: null,
		});
		expect(rawRequest).toHaveBeenNthCalledWith(2, {
			method: "POST",
			path: "/refresh",
			body: undefined,
			headers: undefined,
		});
	});

	it("exposes raw http adapter methods when using custom refresh request", async () => {
		const { createAuthHandle } = await import("../../../src/client/sveltekit");

		mocks.createHttpClient.mockReturnValue({});
		mocks.createAuthCommands.mockReturnValue({ session: vi.fn().mockResolvedValue({ id: "ok" }) });

		const event = {
			cookies: createCookiesMock(),
			fetch: vi.fn(),
			locals: {},
			url: new URL("https://example.com/private"),
		} as any;
		const resolve = vi.fn(async () => new Response("ok", { status: 200 }));

		const refreshRequest = vi.fn(async ({ rawHttp }) => {
			await rawHttp.request({ method: "GET", path: "/r1" });
			await rawHttp.requestWithMeta({ method: "POST", path: "/r2", body: { a: 1 } });
			await rawHttp.get("/r3", { x: "1" });
			await rawHttp.getWithMeta("/r4");
			await rawHttp.post("/r5", { b: 2 });
			await rawHttp.put("/r6", { c: 3 });
			await rawHttp.patch("/r7", { d: 4 });
			await rawHttp.delete("/r8");
			return { accessToken: "a3", refreshToken: "r3" };
		});

		const handle = createAuthHandle({
			baseUrl: "https://api.example.com",
			routes: authRoutes,
			cookies: { accessTokenCookie: "access", refreshTokenCookie: "refresh" },
			refreshRequest,
		});

		await handle({ event, resolve } as any);
		const cfg = mocks.createHttpClient.mock.calls[0][0];
		const refresh = cfg.auth.refresh as (ctx: {
			rawRequest: (req: any) => Promise<any>;
			rawRequestWithMeta?: (req: any, opts?: any) => Promise<any>;
			getRefreshToken: () => Promise<string | null>;
		}) => Promise<any>;
		const rawRequest = vi.fn(async (req) => ({ ok: true, req }));
		const rawRequestWithMeta = vi.fn(async (req, opts) => ({
			status: 201,
			headers: { "x-req": "meta" },
			body: { ok: true, req, opts },
		}));

		await expect(
			refresh({
				rawRequest,
				rawRequestWithMeta,
				getRefreshToken: vi.fn().mockResolvedValue("rt"),
			})
		).resolves.toEqual({ success: true, accessToken: "a3", refreshToken: "r3" });
		expect(rawRequest).toHaveBeenCalledTimes(6);
		expect(rawRequestWithMeta).toHaveBeenCalledTimes(2);
		expect(refreshRequest).toHaveBeenCalled();
	});

	it("preserves real status, headers, and acceptedStatuses in rawHttp *WithMeta", async () => {
		const { createAuthHandle } = await import("../../../src/client/sveltekit");

		mocks.createHttpClient.mockReturnValue({});
		mocks.createAuthCommands.mockReturnValue({ session: vi.fn().mockResolvedValue({ id: "ok" }) });

		const event = {
			cookies: createCookiesMock(),
			fetch: vi.fn(),
			locals: {},
			url: new URL("https://example.com/private"),
		} as any;
		const resolve = vi.fn(async () => new Response("ok", { status: 200 }));

		let observedPostMeta: any;
		let observedGetMeta: any;
		const refreshRequest = vi.fn(async ({ rawHttp }) => {
			observedPostMeta = await rawHttp.requestWithMeta(
				{ method: "POST", path: "/refresh-check" },
				{ acceptedStatuses: [202] }
			);
			observedGetMeta = await rawHttp.getWithMeta(
				"/session-check",
				{ "x-test": "1" },
				{ acceptedStatuses: [304] }
			);
			return { accessToken: "a4", refreshToken: "r4" };
		});

		const handle = createAuthHandle({
			baseUrl: "https://api.example.com",
			routes: authRoutes,
			cookies: { accessTokenCookie: "access", refreshTokenCookie: "refresh" },
			refreshRequest,
		});

		await handle({ event, resolve } as any);
		const cfg = mocks.createHttpClient.mock.calls[0][0];
		const refresh = cfg.auth.refresh as (ctx: {
			rawRequest: (req: any) => Promise<any>;
			rawRequestWithMeta?: (req: any, opts?: any) => Promise<any>;
			getRefreshToken: () => Promise<string | null>;
		}) => Promise<any>;

		const rawRequestWithMeta = vi
			.fn()
			.mockResolvedValueOnce({ status: 202, headers: { "x-accepted": "yes" }, body: { queued: true } })
			.mockResolvedValueOnce({ status: 304, headers: { etag: '"v1"' }, body: null });

		await expect(
			refresh({
				rawRequest: vi.fn(),
				rawRequestWithMeta,
				getRefreshToken: vi.fn().mockResolvedValue("rt"),
			})
		).resolves.toEqual({ success: true, accessToken: "a4", refreshToken: "r4" });

		expect(observedPostMeta).toEqual({
			status: 202,
			headers: { "x-accepted": "yes" },
			body: { queued: true },
		});
		expect(observedGetMeta).toEqual({
			status: 304,
			headers: { etag: '"v1"' },
			body: null,
		});
		expect(rawRequestWithMeta).toHaveBeenNthCalledWith(
			1,
			{ method: "POST", path: "/refresh-check" },
			{ acceptedStatuses: [202] }
		);
		expect(rawRequestWithMeta).toHaveBeenNthCalledWith(
			2,
			{ method: "GET", path: "/session-check", headers: { "x-test": "1" } },
			{ acceptedStatuses: [304] }
		);

		// Missing rawRequestWithMeta fails closed instead of fabricating { status: 200, headers: {} }
		await expect(
			refresh({
				rawRequest: vi.fn(),
				getRefreshToken: vi.fn().mockResolvedValue("rt"),
			})
		).resolves.toEqual({ success: false, accessToken: null, refreshToken: null });
	});

	it("rethrows transient 5xx and network failures while returning success:false for 401/403 rejections", async () => {
		const { createAuthHandle } = await import("../../../src/client/sveltekit");

		mocks.createHttpClient.mockReturnValue({});
		mocks.createAuthCommands.mockReturnValue({ session: vi.fn().mockResolvedValue({ id: "ok" }) });

		const event = {
			cookies: createCookiesMock(),
			fetch: vi.fn(),
			locals: {},
			url: new URL("https://example.com/private"),
		} as any;
		const resolve = vi.fn(async () => new Response("ok", { status: 200 }));

		const handle = createAuthHandle({
			baseUrl: "https://api.example.com",
			routes: authRoutes,
			cookies: { accessTokenCookie: "access", refreshTokenCookie: "refresh" },
		});

		await handle({ event, resolve } as any);
		const cfg = mocks.createHttpClient.mock.calls[0][0];
		const refresh = cfg.auth.refresh as (ctx: {
			rawRequest: (req: any) => Promise<any>;
			getRefreshToken: () => Promise<string | null>;
		}) => Promise<any>;

		const serverOutage = new UnderlayHttpError(503, "Service Unavailable");
		const networkOutage = new UnderlayHttpError(0, "Network error");
		const rejected401 = new UnderlayHttpError(401, "Invalid refresh token");
		const rejected403 = new UnderlayHttpError(403, "Account disabled");

		const rawRequest = vi
			.fn()
			.mockRejectedValueOnce(serverOutage)
			.mockRejectedValueOnce(networkOutage)
			.mockRejectedValueOnce(rejected401)
			.mockRejectedValueOnce(rejected403);

		await expect(
			refresh({ rawRequest, getRefreshToken: vi.fn().mockResolvedValue("rt") })
		).rejects.toBe(serverOutage);
		await expect(
			refresh({ rawRequest, getRefreshToken: vi.fn().mockResolvedValue("rt") })
		).rejects.toBe(networkOutage);
		await expect(
			refresh({ rawRequest, getRefreshToken: vi.fn().mockResolvedValue("rt") })
		).resolves.toEqual({ success: false, accessToken: null, refreshToken: null });
		await expect(
			refresh({ rawRequest, getRefreshToken: vi.fn().mockResolvedValue("rt") })
		).resolves.toEqual({ success: false, accessToken: null, refreshToken: null });
	});
});
