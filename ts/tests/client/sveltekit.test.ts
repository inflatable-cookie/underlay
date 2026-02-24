import { beforeEach, describe, expect, it, vi } from "vitest";
import { UnderlayHttpError } from "../../src/client/errors";

const mocks = vi.hoisted(() => ({
	createHttpClient: vi.fn(),
	createAuthCommands: vi.fn(),
}));

vi.mock("../../src/client/http", () => ({
	createHttpClient: mocks.createHttpClient,
}));

vi.mock("../../src/client/auth", () => ({
	createAuthCommands: mocks.createAuthCommands,
}));

function createCookiesMock() {
	const jar = new Map<string, string>();
	const deleted: Array<{ key: string; options: Record<string, unknown> }> = [];
	const setCalls: Array<{ key: string; value: string; options: Record<string, unknown> }> = [];

	return {
		jar,
		deleted,
		setCalls,
		get: vi.fn((key: string) => jar.get(key)),
		set: vi.fn((key: string, value: string, options: Record<string, unknown>) => {
			jar.set(key, value);
			setCalls.push({ key, value, options });
		}),
		delete: vi.fn((key: string, options: Record<string, unknown>) => {
			jar.delete(key);
			deleted.push({ key, options });
		}),
	};
}

describe("client/sveltekit", () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	it("reads/writes/clears auth cookies via helpers", async () => {
		const { createAuthCookieHelpers } = await import("../../src/client/sveltekit");
		const cookies = createCookiesMock();

		cookies.jar.set("access", " access-token ");
		cookies.jar.set("refresh", "refresh-token");

		const helpers = createAuthCookieHelpers({
			accessTokenName: "access",
			refreshTokenName: "refresh",
			options: { secure: true, maxAge: 42 },
		});

		expect(helpers.readAccessToken(cookies as any)).toBe("access-token");
		expect(helpers.readRefreshToken(cookies as any)).toBe("refresh-token");
		cookies.jar.set("access", "   ");
		cookies.jar.delete("refresh");
		expect(helpers.readAccessToken(cookies as any)).toBeNull();
		expect(helpers.readRefreshToken(cookies as any)).toBeNull();

		helpers.writeAuthTokens(cookies as any, {
			accessToken: "a1",
			refreshToken: "r1",
		});
		expect(cookies.set).toHaveBeenCalledTimes(2);
		expect(cookies.setCalls[0].options).toMatchObject({
			path: "/",
			httpOnly: true,
			sameSite: "lax",
			maxAge: 42,
			secure: true,
		});

		helpers.clearAuthTokens(cookies as any);
		expect(cookies.delete).toHaveBeenNthCalledWith(1, "access", { path: "/" });
		expect(cookies.delete).toHaveBeenNthCalledWith(2, "refresh", { path: "/" });
	});

	it("creates a cookie token store with get/set/clear behavior", async () => {
		const { createCookieTokenStore } = await import("../../src/client/sveltekit");
		const cookies = createCookiesMock();
		const event = { cookies } as any;

		const store = createCookieTokenStore(event, {
			accessTokenCookie: "access",
			refreshTokenCookie: "refresh",
			cookie: { sameSite: "strict" },
		});

		expect(await store.getAccessToken()).toBeNull();
		await store.setAccessToken("a1");
		expect(cookies.set).toHaveBeenCalledWith("access", "a1", { path: "/", sameSite: "strict" });
		await store.setAccessToken(null);
		expect(cookies.delete).toHaveBeenCalledWith("access", { path: "/" });

		expect(await store.getRefreshToken()).toBeNull();
		await store.setRefreshToken("r1");
		expect(await store.getRefreshToken()).toBe("r1");
		await store.setRefreshToken(null);
		expect(cookies.delete).toHaveBeenCalledWith("refresh", { path: "/" });
		await store.clear();
		expect(cookies.delete).toHaveBeenCalledWith("access", { path: "/" });
		expect(cookies.delete).toHaveBeenCalledWith("refresh", { path: "/" });
	});

	it("creates auth handle that decorates locals and resolves when unprotected", async () => {
		const { createAuthHandle } = await import("../../src/client/sveltekit");

		const rawHttp = { post: vi.fn() };
		const authedHttp = { request: vi.fn() };
		mocks.createHttpClient.mockReturnValueOnce(rawHttp).mockReturnValueOnce(authedHttp);
		mocks.createAuthCommands.mockReturnValue({ session: vi.fn().mockResolvedValue({ id: "ok" }) });

		const cookies = createCookiesMock();
		const event = {
			cookies,
			fetch: vi.fn(),
			locals: {},
			url: new URL("https://example.com/app"),
		} as any;
		const resolve = vi.fn(async () => new Response("ok", { status: 200 }));

		const handle = createAuthHandle({
			baseUrl: "https://api.example.com",
			routes: {
				register: "/register",
				loginPassword: "/login/password",
				loginPasskey: "/login/passkey",
				logout: "/logout",
				refresh: "/refresh",
				session: "/session",
			},
			cookies: { accessTokenCookie: "access", refreshTokenCookie: "refresh" },
			shouldProtect: () => false,
		});

		const response = await handle({ event, resolve } as any);
		expect(response.status).toBe(200);
		expect(resolve).toHaveBeenCalledWith(event);
		expect((event.locals as any).auth).toBeDefined();
		expect(mocks.createAuthCommands).toHaveBeenCalledWith(authedHttp, expect.any(Object));
	});

	it("returns onUnauthenticated response for protected requests with invalid session", async () => {
		const { createAuthHandle } = await import("../../src/client/sveltekit");

		mocks.createHttpClient.mockReturnValue({});
		mocks.createAuthCommands.mockReturnValue({
			session: vi.fn().mockRejectedValue(new UnderlayHttpError(401, "unauthorized")),
		});

		const cookies = createCookiesMock();
		cookies.jar.set("access", "a1");
		cookies.jar.set("refresh", "r1");
		const event = {
			cookies,
			fetch: vi.fn(),
			locals: {},
			url: new URL("https://example.com/private"),
		} as any;
		const resolve = vi.fn(async () => new Response("ok", { status: 200 }));

		const handle = createAuthHandle({
			baseUrl: "https://api.example.com",
			routes: {
				register: "/register",
				loginPassword: "/login/password",
				loginPasskey: "/login/passkey",
				logout: "/logout",
				refresh: "/refresh",
				session: "/session",
			},
			cookies: { accessTokenCookie: "access", refreshTokenCookie: "refresh" },
			shouldProtect: () => true,
			onUnauthenticated: async () => new Response("login", { status: 302 }),
		});

		const response = await handle({ event, resolve } as any);
		expect(response.status).toBe(302);
		expect(resolve).not.toHaveBeenCalled();
		expect(cookies.delete).toHaveBeenCalledWith("access", { path: "/" });
		expect(cookies.delete).toHaveBeenCalledWith("refresh", { path: "/" });
	});

	it("returns default 401 response when protected and unauthenticated without override", async () => {
		const { createAuthHandle } = await import("../../src/client/sveltekit");

		mocks.createHttpClient.mockReturnValue({});
		mocks.createAuthCommands.mockReturnValue({
			session: vi.fn().mockResolvedValue(null),
		});

		const event = {
			cookies: createCookiesMock(),
			fetch: vi.fn(),
			locals: {},
			url: new URL("https://example.com/private"),
		} as any;
		const resolve = vi.fn(async () => new Response("ok", { status: 200 }));

		const handle = createAuthHandle({
			baseUrl: "https://api.example.com",
			routes: {
				register: "/register",
				loginPassword: "/login/password",
				loginPasskey: "/login/passkey",
				logout: "/logout",
				refresh: "/refresh",
				session: "/session",
			},
			cookies: { accessTokenCookie: "access", refreshTokenCookie: "refresh" },
			shouldProtect: () => true,
		});

		const response = await handle({ event, resolve } as any);
		expect(response.status).toBe(401);
		expect(await response.text()).toBe("Unauthorized");
		expect(resolve).not.toHaveBeenCalled();
	});

	it("uses default refresh request through auth refresh adapter", async () => {
		const { createAuthHandle } = await import("../../src/client/sveltekit");

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
			routes: {
				register: "/register",
				loginPassword: "/login/password",
				loginPasskey: "/login/passkey",
				logout: "/logout",
				refresh: "/refresh",
				session: "/session",
			},
			cookies: { accessTokenCookie: "access", refreshTokenCookie: "refresh" },
		});

		await handle({ event, resolve } as any);
		const cfg = mocks.createHttpClient.mock.calls[1][0];
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
		const { createAuthHandle } = await import("../../src/client/sveltekit");

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
			routes: {
				register: "/register",
				loginPassword: "/login/password",
				loginPasskey: "/login/passkey",
				logout: "/logout",
				refresh: "/refresh",
				session: "/session",
			},
			cookies: { accessTokenCookie: "access", refreshTokenCookie: "refresh" },
			refreshRequest,
		});

		await handle({ event, resolve } as any);
		const cfg = mocks.createHttpClient.mock.calls[1][0];
		const refresh = cfg.auth.refresh as (ctx: {
			rawRequest: (req: any) => Promise<any>;
			getRefreshToken: () => Promise<string | null>;
		}) => Promise<any>;
		const rawRequest = vi.fn(async (req) => ({ ok: true, req }));

		await expect(
			refresh({
				rawRequest,
				getRefreshToken: vi.fn().mockResolvedValue("rt"),
			})
		).resolves.toEqual({ success: true, accessToken: "a3", refreshToken: "r3" });
		expect(rawRequest).toHaveBeenCalledTimes(8);
		expect(refreshRequest).toHaveBeenCalled();
	});

	it("locals auth getSession rethrows non-401 and clearTokens clears cookies", async () => {
		const { createAuthHandle } = await import("../../src/client/sveltekit");

		const commands = { session: vi.fn().mockRejectedValue(new UnderlayHttpError(500, "boom")) };
		mocks.createHttpClient.mockReturnValue({});
		mocks.createAuthCommands.mockReturnValue(commands);

		const cookies = createCookiesMock();
		const event = {
			cookies,
			fetch: vi.fn(),
			locals: {},
			url: new URL("https://example.com/app"),
		} as any;
		const resolve = vi.fn(async () => new Response("ok", { status: 200 }));

		const handle = createAuthHandle({
			baseUrl: "https://api.example.com",
			routes: {
				register: "/register",
				loginPassword: "/login/password",
				loginPasskey: "/login/passkey",
				logout: "/logout",
				refresh: "/refresh",
				session: "/session",
			},
			cookies: { accessTokenCookie: "access", refreshTokenCookie: "refresh" },
			shouldProtect: () => false,
		});

		await handle({ event, resolve } as any);
		const authLocals = (event.locals as any).auth;

		await expect(authLocals.getSession()).rejects.toMatchObject({ status: 500 });
		await authLocals.clearTokens();
		expect(cookies.delete).toHaveBeenCalledWith("access", { path: "/" });
		expect(cookies.delete).toHaveBeenCalledWith("refresh", { path: "/" });
	});
});
