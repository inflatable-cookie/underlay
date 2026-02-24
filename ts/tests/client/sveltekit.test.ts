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

		await store.setRefreshToken("r1");
		expect(await store.getRefreshToken()).toBe("r1");
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
});
