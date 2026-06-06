import { describe, expect, it } from "vitest";
import { createCspConfig, createSecurityHeadersConfig } from "../../../src/server/csp";

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
