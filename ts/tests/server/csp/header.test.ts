import { describe, expect, it } from "vitest";
import { buildCspHeader, createCspConfig, getCspHeaderName } from "../../../src/server/csp";

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
