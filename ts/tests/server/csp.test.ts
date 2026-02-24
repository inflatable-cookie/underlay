/**
 * Tests for Content Security Policy utilities.
 */

import { describe, it, expect } from "vitest";
import {
	generateNonce,
	createCspConfig,
	createSecurityHeadersConfig,
	buildCspHeader,
	getCspHeaderName,
	applyCspHeaders,
	createCspResolveOptions,
	type CspConfig,
	type ResolvedCspConfig,
} from "../../src/server/csp";

// ============================================================================
// generateNonce
// ============================================================================

describe("generateNonce", () => {
	it("generates a non-empty string", () => {
		const nonce = generateNonce();
		expect(typeof nonce).toBe("string");
		expect(nonce.length).toBeGreaterThan(0);
	});

	it("generates unique values", () => {
		const nonces = new Set<string>();
		for (let i = 0; i < 100; i++) {
			nonces.add(generateNonce());
		}
		expect(nonces.size).toBe(100);
	});

	it("generates base64-encoded values", () => {
		const nonce = generateNonce();
		// Base64 uses A-Z, a-z, 0-9, +, /, and = for padding
		expect(nonce).toMatch(/^[A-Za-z0-9+/]+=*$/);
	});

	it("generates 16 bytes (produces ~22-24 char base64)", () => {
		const nonce = generateNonce();
		// 16 bytes = 128 bits, base64 encodes 6 bits per char
		// ceil(128/6) = 22 chars, with padding up to 24
		expect(nonce.length).toBeGreaterThanOrEqual(22);
		expect(nonce.length).toBeLessThanOrEqual(24);
	});
});

// ============================================================================
// createCspConfig
// ============================================================================

describe("createCspConfig", () => {
	describe("default configuration", () => {
		it("provides sensible defaults", () => {
			const config = createCspConfig();
			expect(config.defaultSrc).toContain("'self'");
			expect(config.scriptSrc).toContain("'self'");
			expect(config.styleSrc).toContain("'self'");
			expect(config.styleSrc).toContain("'unsafe-inline'");
			expect(config.imgSrc).toContain("'self'");
			expect(config.imgSrc).toContain("data:");
			expect(config.imgSrc).toContain("https:");
			expect(config.objectSrc).toContain("'none'");
			expect(config.frameAncestors).toContain("'none'");
			expect(config.reportOnly).toBe(false);
		});
	});

	describe("merging custom sources", () => {
		it("adds custom sources to defaults", () => {
			const config = createCspConfig({
				connectSrc: ["https://api.example.com"],
			});
			expect(config.connectSrc).toContain("'self'");
			expect(config.connectSrc).toContain("https://api.example.com");
		});

		it("deduplicates sources", () => {
			const config = createCspConfig({
				scriptSrc: ["'self'", "https://cdn.example.com"],
			});
			const selfCount = config.scriptSrc.filter((s) => s === "'self'").length;
			expect(selfCount).toBe(1);
		});

		it("adds multiple custom sources", () => {
			const config = createCspConfig({
				frameSrc: ["https://www.youtube.com", "https://player.vimeo.com"],
			});
			expect(config.frameSrc).toContain("'self'");
			expect(config.frameSrc).toContain("https://www.youtube.com");
			expect(config.frameSrc).toContain("https://player.vimeo.com");
		});
	});

	describe("disabling directives", () => {
		it("disables directive when set to false", () => {
			const config = createCspConfig({
				objectSrc: false,
			});
			expect(config.objectSrc).toEqual([]);
		});

		it("can disable frame-ancestors", () => {
			const config = createCspConfig({
				frameAncestors: false,
			});
			expect(config.frameAncestors).toEqual([]);
		});
	});

	describe("report configuration", () => {
		it("sets reportOnly mode", () => {
			const config = createCspConfig({ reportOnly: true });
			expect(config.reportOnly).toBe(true);
		});

		it("sets reportUri", () => {
			const config = createCspConfig({ reportUri: "/csp-report" });
			expect(config.reportUri).toBe("/csp-report");
		});
	});
});

// ============================================================================
// createSecurityHeadersConfig
// ============================================================================

describe("createSecurityHeadersConfig", () => {
	it("provides sensible defaults", () => {
		const config = createSecurityHeadersConfig();
		expect(config.contentTypeOptions).toBe("nosniff");
		expect(config.frameOptions).toBe("DENY");
		expect(config.referrerPolicy).toBe("strict-origin-when-cross-origin");
		expect(config.xssProtection).toBe(false);
	});

	it("allows customization", () => {
		const config = createSecurityHeadersConfig({
			frameOptions: "SAMEORIGIN",
			referrerPolicy: "no-referrer",
		});
		expect(config.frameOptions).toBe("SAMEORIGIN");
		expect(config.referrerPolicy).toBe("no-referrer");
	});

	it("allows disabling headers", () => {
		const config = createSecurityHeadersConfig({
			frameOptions: false,
			contentTypeOptions: false,
		});
		expect(config.frameOptions).toBe(false);
		expect(config.contentTypeOptions).toBe(false);
	});

	it("supports permissions policy", () => {
		const config = createSecurityHeadersConfig({
			permissionsPolicy: "camera=(), microphone=()",
		});
		expect(config.permissionsPolicy).toBe("camera=(), microphone=()");
	});
});

// ============================================================================
// buildCspHeader
// ============================================================================

describe("buildCspHeader", () => {
	const baseConfig = createCspConfig();

	it("builds header with all directives", () => {
		const header = buildCspHeader(baseConfig);
		expect(header).toContain("default-src");
		expect(header).toContain("script-src");
		expect(header).toContain("style-src");
		expect(header).toContain("img-src");
		expect(header).toContain("connect-src");
		expect(header).toContain("frame-src");
		expect(header).toContain("object-src");
	});

	it("separates directives with semicolon and space", () => {
		const header = buildCspHeader(baseConfig);
		expect(header).toMatch(/default-src[^;]+; script-src/);
	});

	it("includes nonce in script-src when provided", () => {
		const nonce = "abc123xyz";
		const header = buildCspHeader(baseConfig, nonce);
		expect(header).toContain(`'nonce-${nonce}'`);
	});

	it("does not include nonce in style-src", () => {
		const nonce = "abc123xyz";
		const header = buildCspHeader(baseConfig, nonce);
		// style-src should NOT have nonce (breaks SvelteKit component styles)
		const styleSrcMatch = header.match(/style-src[^;]+/);
		expect(styleSrcMatch?.[0]).not.toContain("nonce");
	});

	it("includes report-uri when configured", () => {
		const config = createCspConfig({ reportUri: "/csp-violations" });
		const header = buildCspHeader(config);
		expect(header).toContain("report-uri /csp-violations");
	});

	it("omits empty directives", () => {
		const config = createCspConfig({ objectSrc: false });
		const header = buildCspHeader(config);
		expect(header).not.toContain("object-src");
	});
});

// ============================================================================
// getCspHeaderName
// ============================================================================

describe("getCspHeaderName", () => {
	it("returns enforcing header name by default", () => {
		const config = createCspConfig();
		expect(getCspHeaderName(config)).toBe("Content-Security-Policy");
	});

	it("returns report-only header name when configured", () => {
		const config = createCspConfig({ reportOnly: true });
		expect(getCspHeaderName(config)).toBe("Content-Security-Policy-Report-Only");
	});
});

// ============================================================================
// applyCspHeaders
// ============================================================================

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

// ============================================================================
// createCspResolveOptions
// ============================================================================

describe("createCspResolveOptions", () => {
	it("returns options object with transformPageChunk", () => {
		const nonce = "test-nonce";
		const options = createCspResolveOptions(nonce);
		expect(typeof options.transformPageChunk).toBe("function");
	});

	it("replaces nonce placeholder in HTML", async () => {
		const nonce = "my-secure-nonce";
		const options = createCspResolveOptions(nonce);
		const html = '<script nonce="%sveltekit.nonce%">console.log("hi")</script>';

		const result = await options.transformPageChunk({ html, done: true });
		expect(result).toBe(`<script nonce="${nonce}">console.log("hi")</script>`);
	});

	it("replaces multiple nonce placeholders", async () => {
		const nonce = "multi-nonce";
		const options = createCspResolveOptions(nonce);
		const html = `
			<script nonce="%sveltekit.nonce%">one</script>
			<script nonce="%sveltekit.nonce%">two</script>
		`;

		const result = await options.transformPageChunk({ html, done: true });
		expect(result).not.toContain("%sveltekit.nonce%");
		expect(result.match(new RegExp(nonce, "g"))?.length).toBe(2);
	});

	it("preserves existing options", () => {
		const nonce = "test";
		const filterFn = (name: string) => name === "content-type";
		const options = createCspResolveOptions(nonce, {
			filterSerializedResponseHeaders: filterFn,
		});

		expect(options.filterSerializedResponseHeaders).toBe(filterFn);
	});

	it("chains existing transformPageChunk", async () => {
		const nonce = "chained";
		const options = createCspResolveOptions(nonce, {
			transformPageChunk: async ({ html }) => html.toUpperCase(),
		});

		const html = '<script nonce="%sveltekit.nonce%">test</script>';
		const result = await options.transformPageChunk({ html, done: true });

		// Nonce should be replaced first, then uppercase applied
		expect(result).toContain(nonce.toUpperCase());
		expect(result).not.toContain("%sveltekit.nonce%");
	});
});
