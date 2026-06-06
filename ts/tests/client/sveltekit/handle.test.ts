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

describe("client/sveltekit auth handle", () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	it("creates auth handle that decorates locals and resolves when unprotected", async () => {
		const { createAuthHandle } = await import("../../../src/client/sveltekit");

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
			routes: authRoutes,
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
		const { createAuthHandle } = await import("../../../src/client/sveltekit");

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
			routes: authRoutes,
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
		const { createAuthHandle } = await import("../../../src/client/sveltekit");

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
			routes: authRoutes,
			cookies: { accessTokenCookie: "access", refreshTokenCookie: "refresh" },
			shouldProtect: () => true,
		});

		const response = await handle({ event, resolve } as any);
		expect(response.status).toBe(401);
		expect(await response.text()).toBe("Unauthorized");
		expect(resolve).not.toHaveBeenCalled();
	});

	it("resolves protected requests when a valid session exists", async () => {
		const { createAuthHandle } = await import("../../../src/client/sveltekit");

		mocks.createHttpClient.mockReturnValue({});
		mocks.createAuthCommands.mockReturnValue({
			session: vi.fn().mockResolvedValue({ id: "session-1" }),
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
			routes: authRoutes,
			cookies: { accessTokenCookie: "access", refreshTokenCookie: "refresh" },
			shouldProtect: () => true,
		});

		const response = await handle({ event, resolve } as any);
		expect(response.status).toBe(200);
		expect(resolve).toHaveBeenCalledWith(event);
	});
});
