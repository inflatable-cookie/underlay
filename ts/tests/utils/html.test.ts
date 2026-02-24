import { beforeEach, describe, expect, it, vi } from "vitest";

const sanitizeMock = vi.hoisted(() => vi.fn((input: string) => `sanitized:${input}`));

vi.mock("isomorphic-dompurify", () => ({
	default: {
		sanitize: sanitizeMock,
	},
}));

import { sanitizeEmbedHtml, sanitizeHtml, sanitizeSvgHtml } from "../../src/utils/html";

describe("utils/html", () => {
	beforeEach(() => {
		sanitizeMock.mockClear();
	});

	it("sanitizes generic html with html profile and safe null fallback", () => {
		expect(sanitizeHtml("<p>Hello</p>")).toBe("sanitized:<p>Hello</p>");
		expect(sanitizeHtml(null)).toBe("sanitized:");

		expect(sanitizeMock).toHaveBeenNthCalledWith(
			1,
			"<p>Hello</p>",
			{ USE_PROFILES: { html: true } }
		);
		expect(sanitizeMock).toHaveBeenNthCalledWith(
			2,
			"",
			{ USE_PROFILES: { html: true } }
		);
	});

	it("sanitizes embed html with explicit media allowlist", () => {
		const result = sanitizeEmbedHtml("<iframe src=\"https://example.com\"></iframe>");
		expect(result).toContain("sanitized:");
		expect(sanitizeMock).toHaveBeenCalledWith(
			"<iframe src=\"https://example.com\"></iframe>",
			expect.objectContaining({
				ALLOWED_TAGS: ["iframe", "audio", "video", "source"],
				ALLOW_UNKNOWN_PROTOCOLS: false,
			})
		);
	});

	it("sanitizes svg html with svg profile and forbidden tags", () => {
		const result = sanitizeSvgHtml(undefined);
		expect(result).toBe("sanitized:");
		expect(sanitizeMock).toHaveBeenCalledWith(
			"",
			expect.objectContaining({
				USE_PROFILES: { svg: true, svgFilters: true },
				FORBID_TAGS: ["script", "foreignObject"],
			})
		);
	});
});
