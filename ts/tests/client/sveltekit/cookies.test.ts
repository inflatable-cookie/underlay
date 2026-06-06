import { describe, expect, it } from "vitest";
import { createAuthCookieHelpers, createCookieTokenStore } from "../../../src/client/sveltekit";
import { createCookiesMock } from "./fixtures";

describe("client/sveltekit cookies", () => {
	it("reads/writes/clears auth cookies via helpers", () => {
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
});
