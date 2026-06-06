import { describe, expect, it } from "vitest";
import {
	applyCspHeaders,
	createCspConfig,
	createSecurityHeadersConfig
} from "../../../src/server/csp";

describe("applyCspHeaders", () => {
	it("sets CSP header on response", () => {
		const response = new Response();
		const config = createCspConfig();
		applyCspHeaders(response, config);

		expect(response.headers.has("Content-Security-Policy")).toBe(true);
	});

	it("sets CSP header with nonce", () => {
		const response = new Response();
		const config = createCspConfig();
		const nonce = "test-nonce-123";
		applyCspHeaders(response, config, nonce);

		const cspHeader = response.headers.get("Content-Security-Policy");
		expect(cspHeader).toContain(`'nonce-${nonce}'`);
	});

	it("sets report-only header when configured", () => {
		const response = new Response();
		const config = createCspConfig({ reportOnly: true });
		applyCspHeaders(response, config);

		expect(response.headers.has("Content-Security-Policy-Report-Only")).toBe(true);
		expect(response.headers.has("Content-Security-Policy")).toBe(false);
	});

	it("sets security headers", () => {
		const response = new Response();
		const config = createCspConfig();
		applyCspHeaders(response, config);

		expect(response.headers.get("X-Content-Type-Options")).toBe("nosniff");
		expect(response.headers.get("X-Frame-Options")).toBe("DENY");
		expect(response.headers.get("Referrer-Policy")).toBe("strict-origin-when-cross-origin");
	});

	it("respects custom security headers config", () => {
		const response = new Response();
		const cspConfig = createCspConfig();
		const securityConfig = createSecurityHeadersConfig({
			frameOptions: "SAMEORIGIN",
		});
		applyCspHeaders(response, cspConfig, undefined, securityConfig);

		expect(response.headers.get("X-Frame-Options")).toBe("SAMEORIGIN");
	});

	it("sets xss protection header when configured", () => {
		const response = new Response();
		const cspConfig = createCspConfig();
		const securityConfig = createSecurityHeadersConfig({
			xssProtection: "1; mode=block",
		});
		applyCspHeaders(response, cspConfig, undefined, securityConfig);

		expect(response.headers.get("X-XSS-Protection")).toBe("1; mode=block");
	});

	it("omits disabled security headers", () => {
		const response = new Response();
		const cspConfig = createCspConfig();
		const securityConfig = createSecurityHeadersConfig({
			frameOptions: false,
			contentTypeOptions: false,
			referrerPolicy: false,
		});
		applyCspHeaders(response, cspConfig, undefined, securityConfig);

		expect(response.headers.has("X-Frame-Options")).toBe(false);
		expect(response.headers.has("X-Content-Type-Options")).toBe(false);
		expect(response.headers.has("Referrer-Policy")).toBe(false);
	});

	it("sets permissions policy when configured", () => {
		const response = new Response();
		const cspConfig = createCspConfig();
		const securityConfig = createSecurityHeadersConfig({
			permissionsPolicy: "geolocation=()",
		});
		applyCspHeaders(response, cspConfig, undefined, securityConfig);

		expect(response.headers.get("Permissions-Policy")).toBe("geolocation=()");
	});
});
