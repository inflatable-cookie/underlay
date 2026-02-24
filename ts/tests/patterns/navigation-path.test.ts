import { describe, it, expect, beforeEach, afterEach } from "vitest";
import { matchesCurrentPath } from "../../src/patterns/navigation-path";

describe("matchesCurrentPath", () => {
	const originalWindow = (globalThis as { window?: unknown }).window;

	afterEach(() => {
		(globalThis as { window?: unknown }).window = originalWindow;
	});

	it("returns true during SSR when window is unavailable", () => {
		(globalThis as { window?: unknown }).window = undefined;
		expect(matchesCurrentPath("/any/path")).toBe(true);
	});

	it("matches pathnames while ignoring query strings and hashes", () => {
		(globalThis as { window?: unknown }).window = {
			location: {
				origin: "https://example.com",
				pathname: "/articles/123"
			}
		};

		expect(matchesCurrentPath("https://example.com/articles/123?tab=meta#section")).toBe(true);
		expect(matchesCurrentPath("/articles/123?x=1")).toBe(true);
		expect(matchesCurrentPath("/articles/456")).toBe(false);
	});

	it("falls back to raw string comparison when URL parsing fails", () => {
		const invalidHref = "http://[::1";
		(globalThis as { window?: unknown }).window = {
			location: {
				origin: "https://example.com",
				pathname: invalidHref
			}
		};

		expect(matchesCurrentPath(invalidHref)).toBe(true);
		expect(matchesCurrentPath("/other")).toBe(false);
	});
});
