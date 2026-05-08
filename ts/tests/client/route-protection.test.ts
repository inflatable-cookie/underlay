import { describe, expect, it } from "vitest";
import {
	createLoginRedirect,
	createRouteProtection,
	isPublicPath,
	shouldProtectRoute,
} from "../../src/client/route-protection";

describe("client/route-protection", () => {
	it("matches exact and wildcard public paths without widening plain paths", () => {
		const publicPaths = ["/login", "/auth/*", "/health"];

		expect(isPublicPath("/login", publicPaths)).toBe(true);
		expect(isPublicPath("/login-help", publicPaths)).toBe(false);
		expect(isPublicPath("/auth/callback", publicPaths)).toBe(true);
		expect(isPublicPath("/health", publicPaths)).toBe(true);
		expect(isPublicPath("/health/check", publicPaths)).toBe(false);
		expect(isPublicPath("/private", publicPaths)).toBe(false);
	});

	it("determines route protection from public path matching", () => {
		const publicPaths = ["/login", "/public/*"];
		expect(shouldProtectRoute("/login", publicPaths)).toBe(false);
		expect(shouldProtectRoute("/public/docs", publicPaths)).toBe(false);
		expect(shouldProtectRoute("/admin", publicPaths)).toBe(true);
	});

	it("creates login redirects with defaults and custom options", () => {
		const source = new URL("https://example.com/admin/users");

		const withDefault = createLoginRedirect(source);
		expect(withDefault.status).toBe(302);
		expect(withDefault.headers.get("location")).toBe(
			"https://example.com/login?redirectTo=%2Fadmin%2Fusers"
		);

		const withStringConfig = createLoginRedirect(source, "/sign-in");
		expect(withStringConfig.headers.get("location")).toBe(
			"https://example.com/sign-in?redirectTo=%2Fadmin%2Fusers"
		);

		const withCustomQuery = createLoginRedirect(source, {
			loginPath: "/sign-in",
			useRedirectTo: true,
			redirectToParam: "next",
		});
		expect(withCustomQuery.headers.get("location")).toBe(
			"https://example.com/sign-in?next=%2Fadmin%2Fusers"
		);

		const noRedirectTo = createLoginRedirect(source, { useRedirectTo: false });
		expect(noRedirectTo.headers.get("location")).toBe("https://example.com/login");

		const alreadyOnLogin = createLoginRedirect(new URL("https://example.com/login"));
		expect(alreadyOnLogin.headers.get("location")).toBe("https://example.com/login");
	});

	it("builds a route protection function", () => {
		const protectRoute = createRouteProtection({
			publicPaths: ["/login", "/auth/*"],
			loginPath: "/login",
			redirectToParam: "next",
		});

		expect(protectRoute(new URL("https://example.com/login"))).toBeNull();
		const redirect = protectRoute(new URL("https://example.com/admin"));
		expect(redirect?.status).toBe(302);
		expect(redirect?.headers.get("location")).toBe("https://example.com/login?next=%2Fadmin");
	});
});
