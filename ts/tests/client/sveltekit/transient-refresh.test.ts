import { describe, expect, it, vi } from "vitest";
import { UnderlayHttpError } from "../../../src/client/errors";
import { createAuthHandle } from "../../../src/client/sveltekit";
import { authRoutes, createCookiesMock } from "./fixtures";

describe("client/sveltekit createAuthHandle transient refresh preservation", () => {
	it("preserves session cookies when refresh fails with 5xx or network error", async () => {
		const fetchMock = vi
			.fn()
			.mockResolvedValueOnce(
				new Response(JSON.stringify({ error: { code: "unauthorized", message: "Expired" } }), {
					status: 401,
					headers: { "content-type": "application/json" },
				}),
			)
			.mockResolvedValueOnce(
				new Response(JSON.stringify({ error: { code: "unavailable", message: "Down" } }), {
					status: 503,
					headers: { "content-type": "application/json" },
				}),
			);

		const cookies = createCookiesMock();
		cookies.jar.set("access", "expired-access");
		cookies.jar.set("refresh", "valid-refresh");

		const event = {
			cookies,
			fetch: fetchMock,
			locals: {},
			url: new URL("https://example.com/private"),
		} as any;
		const resolve = vi.fn(async () => new Response("ok", { status: 200 }));

		const handle = createAuthHandle({
			baseUrl: "https://api.example.com",
			routes: authRoutes,
			cookies: {
				accessTokenCookie: "access",
				refreshTokenCookie: "refresh",
				cookie: { path: "/app", domain: ".example.com" },
			},
			shouldProtect: () => true,
		});

		await expect(handle({ event, resolve } as any)).rejects.toMatchObject({
			status: 503,
		});
		expect(cookies.delete).not.toHaveBeenCalled();
		expect(cookies.jar.get("access")).toBe("expired-access");
		expect(cookies.jar.get("refresh")).toBe("valid-refresh");

		// Network failure on refresh also preserves cookies
		const netFetch = vi
			.fn()
			.mockResolvedValueOnce(
				new Response(JSON.stringify({ error: { code: "unauthorized", message: "Expired" } }), {
					status: 401,
					headers: { "content-type": "application/json" },
				}),
			)
			.mockRejectedValueOnce(new TypeError("fetch failed"));

		const netCookies = createCookiesMock();
		netCookies.jar.set("access", "expired-access");
		netCookies.jar.set("refresh", "valid-refresh");
		const netEvent = {
			cookies: netCookies,
			fetch: netFetch,
			locals: {},
			url: new URL("https://example.com/private"),
		} as any;

		const netHandle = createAuthHandle({
			baseUrl: "https://api.example.com",
			routes: authRoutes,
			cookies: { accessTokenCookie: "access", refreshTokenCookie: "refresh" },
			shouldProtect: () => true,
		});

		await expect(netHandle({ event: netEvent, resolve } as any)).rejects.toBeInstanceOf(
			UnderlayHttpError,
		);
		expect(netCookies.delete).not.toHaveBeenCalled();
		expect(netCookies.jar.get("refresh")).toBe("valid-refresh");
	});

	it("wipes session cookies with full cookie options when refresh returns 401/403", async () => {
		for (const status of [401, 403] as const) {
			const fetchMock = vi
				.fn()
				.mockResolvedValueOnce(
					new Response(JSON.stringify({ error: { code: "unauthorized", message: "Expired" } }), {
						status: 401,
						headers: { "content-type": "application/json" },
					}),
				)
				.mockResolvedValueOnce(
					new Response(JSON.stringify({ error: { code: "rejected", message: "Revoked" } }), {
						status,
						headers: { "content-type": "application/json" },
					}),
				);

			const cookies = createCookiesMock();
			cookies.jar.set("access", "expired-access");
			cookies.jar.set("refresh", "revoked-refresh");

			const event = {
				cookies,
				fetch: fetchMock,
				locals: {},
				url: new URL("https://example.com/private"),
			} as any;
			const resolve = vi.fn(async () => new Response("ok", { status: 200 }));

			const handle = createAuthHandle({
				baseUrl: "https://api.example.com",
				routes: authRoutes,
				cookies: {
					accessTokenCookie: "access",
					refreshTokenCookie: "refresh",
					cookie: { path: "/app", domain: ".example.com", secure: true },
				},
				shouldProtect: () => true,
			});

			const response = await handle({ event, resolve } as any);
			expect(response.status).toBe(401);
			expect(cookies.delete).toHaveBeenCalledWith("access", {
				path: "/app",
				domain: ".example.com",
				secure: true,
			});
			expect(cookies.delete).toHaveBeenCalledWith("refresh", {
				path: "/app",
				domain: ".example.com",
				secure: true,
			});
			expect(cookies.jar.has("access")).toBe(false);
			expect(cookies.jar.has("refresh")).toBe(false);
		}
	});
});
