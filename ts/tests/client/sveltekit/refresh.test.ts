import { beforeEach, describe, expect, it, vi } from "vitest";
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
});
