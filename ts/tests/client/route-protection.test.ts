import { describe, expect, it } from "vitest";
import {
	createLoginRedirect,
	createRouteProtection,
	isPublicPath,
	normalizePath,
	resolveRedirectTo,
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

	it("resolveRedirectTo accepts legitimate same-origin paths", () => {
		expect(resolveRedirectTo("/admin/users")).toBe("/admin/users");
		expect(resolveRedirectTo("/admin/users?tab=2")).toBe("/admin/users?tab=2");
		expect(resolveRedirectTo("/")).toBe("/");
		expect(resolveRedirectTo("/a/b/c#frag")).toBe("/a/b/c#frag");
	});

	it("resolveRedirectTo rejects off-origin and malformed targets", () => {
		expect(resolveRedirectTo("//evil.com")).toBe("/");
		expect(resolveRedirectTo("//evil.com/path")).toBe("/");
		expect(resolveRedirectTo("\\evil")).toBe("/");
		expect(resolveRedirectTo("/\\evil.com")).toBe("/");
		expect(resolveRedirectTo("https://evil.com")).toBe("/");
		expect(resolveRedirectTo("javascript:alert(1)")).toBe("/");
		expect(resolveRedirectTo("%2F%2Fevil.com")).toBe("/");
		expect(resolveRedirectTo("/%2e%2e/admin")).toBe("/");
		expect(resolveRedirectTo("/%252e%252e/admin")).toBe("/");
		expect(resolveRedirectTo("/a/../b")).toBe("/");
		expect(resolveRedirectTo("/line%0d%0abreak")).toBe("/");
		expect(resolveRedirectTo("")).toBe("/");
		expect(resolveRedirectTo(null)).toBe("/");
		expect(resolveRedirectTo(undefined)).toBe("/");
		expect(resolveRedirectTo("%zz")).toBe("/");
	});

	it("resolveRedirectTo honours a custom fallback", () => {
		expect(resolveRedirectTo("//evil.com", "/home")).toBe("/home");
	});

	it("normalizePath decodes and collapses traversal", () => {
		expect(normalizePath("/docs/../admin")).toBe("/admin");
		expect(normalizePath("/docs/%2e%2e/admin")).toBe("/admin");
		expect(normalizePath("/%70ublic/page")).toBe("/public/page");
		expect(normalizePath("/docs/")).toBe("/docs/");
		expect(normalizePath("/")).toBe("/");
	});

	it("isPublicPath is not bypassed by encoded traversal", () => {
		const publicPaths = ["/docs/*"];

		expect(isPublicPath("/docs/guide", publicPaths)).toBe(true);
		expect(isPublicPath("/docs/../admin", publicPaths)).toBe(false);
		expect(isPublicPath("/docs/%2e%2e/admin", publicPaths)).toBe(false);
		expect(shouldProtectRoute("/docs/%2e%2e/admin", publicPaths)).toBe(true);
	});

	it("does not write protocol-relative pathnames into the redirect param", () => {
		const redirect = createLoginRedirect(new URL("https://example.com//evil.com"));
		expect(redirect.headers.get("location")).toBe("https://example.com/login");
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
