/**
 * Tests for the Media Embed Parser utility functions.
 *
 * Covers:
 * - URL extraction from embed HTML
 * - Dimension extraction
 * - HTML entity decoding
 * - URL detection and normalization
 * - Query string parsing and building
 * - Timestamp conversion
 * - HTML generation
 */

import { describe, it, expect } from "vitest";
import {
	extractUrlFromEmbed,
	extractAttribute,
	extractDimensions,
	decodeHtmlEntities,
	isEmbedHtml,
	isUrl,
	normalizeUrl,
	parseQueryString,
	buildQueryString,
	getHostname,
	hostnameMatches,
	isSafeEmbedUrl,
	generateIframeHtml,
	escapeHtmlAttribute,
	secondsToTimestamp,
	timestampToSeconds,
} from "../../src/embed/utils";

// ============================================================================
// extractUrlFromEmbed
// ============================================================================

describe("extractUrlFromEmbed", () => {
	it("extracts src from iframe", () => {
		const html = `<iframe src="https://www.youtube.com/embed/abc123"></iframe>`;
		expect(extractUrlFromEmbed(html)).toBe(
			"https://www.youtube.com/embed/abc123"
		);
	});

	it("extracts src from embed tag", () => {
		const html = `<embed src="https://player.vimeo.com/video/123456" type="video/mp4">`;
		expect(extractUrlFromEmbed(html)).toBe(
			"https://player.vimeo.com/video/123456"
		);
	});

	it("extracts data from object tag", () => {
		const html = `<object data="https://example.com/video.swf" type="application/x-shockwave-flash"></object>`;
		expect(extractUrlFromEmbed(html)).toBe("https://example.com/video.swf");
	});

	it("extracts src from script tag", () => {
		const html = `<script src="https://embed.example.com/player.js"></script>`;
		expect(extractUrlFromEmbed(html)).toBe("https://embed.example.com/player.js");
	});

	it("handles HTML entities in URLs", () => {
		const html = `<iframe src="https://www.youtube.com/embed/abc123?autoplay=1&amp;rel=0"></iframe>`;
		expect(extractUrlFromEmbed(html)).toBe(
			"https://www.youtube.com/embed/abc123?autoplay=1&rel=0"
		);
	});

	it("returns null for HTML without extractable URL", () => {
		const html = `<div class="video-container"></div>`;
		expect(extractUrlFromEmbed(html)).toBeNull();
	});
});

// ============================================================================
// extractAttribute
// ============================================================================

describe("extractAttribute", () => {
	it("extracts attribute value with double quotes", () => {
		const html = `<iframe src="https://example.com" width="560"></iframe>`;
		expect(extractAttribute(html, "iframe", "src")).toBe("https://example.com");
		expect(extractAttribute(html, "iframe", "width")).toBe("560");
	});

	it("extracts attribute value with single quotes", () => {
		const html = `<iframe src='https://example.com'></iframe>`;
		expect(extractAttribute(html, "iframe", "src")).toBe("https://example.com");
	});

	it("handles multiple attributes", () => {
		const html = `<iframe width="560" height="315" src="https://example.com" frameborder="0"></iframe>`;
		expect(extractAttribute(html, "iframe", "src")).toBe("https://example.com");
	});

	it("returns null for missing attribute", () => {
		const html = `<iframe src="https://example.com"></iframe>`;
		expect(extractAttribute(html, "iframe", "width")).toBeNull();
	});

	it("returns null for missing tag", () => {
		const html = `<div src="https://example.com"></div>`;
		expect(extractAttribute(html, "iframe", "src")).toBeNull();
	});

	it("is case-insensitive for tag names", () => {
		const html = `<IFRAME SRC="https://example.com"></IFRAME>`;
		expect(extractAttribute(html, "iframe", "src")).toBe("https://example.com");
	});
});

// ============================================================================
// extractDimensions
// ============================================================================

describe("extractDimensions", () => {
	it("extracts width and height", () => {
		const html = `<iframe width="560" height="315"></iframe>`;
		const dims = extractDimensions(html);
		expect(dims.width).toBe(560);
		expect(dims.height).toBe(315);
	});

	it("handles quotes around values", () => {
		const html = `<iframe width='640' height='360'></iframe>`;
		const dims = extractDimensions(html);
		expect(dims.width).toBe(640);
		expect(dims.height).toBe(360);
	});

	it("handles unquoted values", () => {
		const html = `<iframe width=560 height=315></iframe>`;
		const dims = extractDimensions(html);
		expect(dims.width).toBe(560);
		expect(dims.height).toBe(315);
	});

	it("returns empty object for missing dimensions", () => {
		const html = `<iframe src="https://example.com"></iframe>`;
		const dims = extractDimensions(html);
		expect(dims.width).toBeUndefined();
		expect(dims.height).toBeUndefined();
	});

	it("returns partial dimensions", () => {
		const html = `<iframe width="560"></iframe>`;
		const dims = extractDimensions(html);
		expect(dims.width).toBe(560);
		expect(dims.height).toBeUndefined();
	});
});

// ============================================================================
// decodeHtmlEntities
// ============================================================================

describe("decodeHtmlEntities", () => {
	it("decodes &amp;", () => {
		expect(decodeHtmlEntities("a &amp; b")).toBe("a & b");
	});

	it("decodes &lt; and &gt;", () => {
		expect(decodeHtmlEntities("&lt;div&gt;")).toBe("<div>");
	});

	it("decodes &quot;", () => {
		expect(decodeHtmlEntities('say &quot;hello&quot;')).toBe('say "hello"');
	});

	it("decodes &#39; and &#x27;", () => {
		expect(decodeHtmlEntities("it&#39;s")).toBe("it's");
		expect(decodeHtmlEntities("it&#x27;s")).toBe("it's");
	});

	it("decodes &#x2F;", () => {
		expect(decodeHtmlEntities("a&#x2F;b")).toBe("a/b");
	});

	it("handles multiple entities", () => {
		expect(decodeHtmlEntities("&lt;a href=&quot;/&quot;&gt;")).toBe(
			'<a href="/">'
		);
	});
});

// ============================================================================
// isEmbedHtml
// ============================================================================

describe("isEmbedHtml", () => {
	it("returns true for iframe tags", () => {
		expect(isEmbedHtml("<iframe src='...'></iframe>")).toBe(true);
		expect(isEmbedHtml("<IFRAME src='...'></IFRAME>")).toBe(true);
	});

	it("returns true for embed tags", () => {
		expect(isEmbedHtml("<embed src='...' />")).toBe(true);
	});

	it("returns true for object tags", () => {
		expect(isEmbedHtml("<object data='...'></object>")).toBe(true);
	});

	it("returns true for script tags with src", () => {
		expect(isEmbedHtml("<script src='...'></script>")).toBe(true);
	});

	it("returns false for script tags without src", () => {
		expect(isEmbedHtml("<script>console.log('hi')</script>")).toBe(false);
	});

	it("returns false for plain text", () => {
		expect(isEmbedHtml("hello world")).toBe(false);
	});

	it("returns false for URLs", () => {
		expect(isEmbedHtml("https://www.youtube.com/watch?v=abc")).toBe(false);
	});

	it("returns false for div and other non-embed tags", () => {
		expect(isEmbedHtml("<div>content</div>")).toBe(false);
		expect(isEmbedHtml("<video src='...'></video>")).toBe(false);
	});
});

// ============================================================================
// isUrl
// ============================================================================

describe("isUrl", () => {
	it("returns true for http URLs", () => {
		expect(isUrl("http://example.com")).toBe(true);
	});

	it("returns true for https URLs", () => {
		expect(isUrl("https://example.com")).toBe(true);
	});

	it("returns true for protocol-relative URLs", () => {
		expect(isUrl("//example.com")).toBe(true);
	});

	it("returns false for plain text", () => {
		expect(isUrl("hello world")).toBe(false);
	});

	it("returns false for raw IDs", () => {
		expect(isUrl("dQw4w9WgXcQ")).toBe(false);
	});

	it("handles whitespace", () => {
		expect(isUrl("  https://example.com  ")).toBe(true);
	});
});

// ============================================================================
// normalizeUrl
// ============================================================================

describe("normalizeUrl", () => {
	it("upgrades http to https", () => {
		expect(normalizeUrl("http://example.com")).toBe("https://example.com");
	});

	it("adds https to protocol-relative URLs", () => {
		expect(normalizeUrl("//example.com")).toBe("https://example.com");
	});

	it("preserves https URLs", () => {
		expect(normalizeUrl("https://example.com")).toBe("https://example.com");
	});

	it("trims whitespace", () => {
		expect(normalizeUrl("  https://example.com  ")).toBe("https://example.com");
	});
});

// ============================================================================
// parseQueryString
// ============================================================================

describe("parseQueryString", () => {
	it("parses simple query string", () => {
		const result = parseQueryString("?foo=bar");
		expect(result).toEqual({ foo: "bar" });
	});

	it("parses multiple parameters", () => {
		const result = parseQueryString("?foo=bar&baz=qux");
		expect(result).toEqual({ foo: "bar", baz: "qux" });
	});

	it("handles missing leading ?", () => {
		const result = parseQueryString("foo=bar&baz=qux");
		expect(result).toEqual({ foo: "bar", baz: "qux" });
	});

	it("decodes URI components", () => {
		const result = parseQueryString("?name=hello%20world");
		expect(result).toEqual({ name: "hello world" });
	});

	it("handles empty values", () => {
		const result = parseQueryString("?foo=&bar=baz");
		expect(result).toEqual({ foo: "", bar: "baz" });
	});

	it("handles keys without values", () => {
		const result = parseQueryString("?foo&bar=baz");
		expect(result).toEqual({ foo: "", bar: "baz" });
	});

	it("returns empty object for empty string", () => {
		expect(parseQueryString("")).toEqual({});
		expect(parseQueryString("?")).toEqual({});
	});
});

// ============================================================================
// buildQueryString
// ============================================================================

describe("buildQueryString", () => {
	it("builds simple query string", () => {
		const result = buildQueryString({ foo: "bar" });
		expect(result).toBe("?foo=bar");
	});

	it("builds multiple parameters", () => {
		const result = buildQueryString({ foo: "bar", baz: "qux" });
		expect(result).toContain("foo=bar");
		expect(result).toContain("baz=qux");
		expect(result).toMatch(/^\?/);
	});

	it("handles numeric values", () => {
		const result = buildQueryString({ count: 5 });
		expect(result).toBe("?count=5");
	});

	it("handles boolean values", () => {
		const result = buildQueryString({ enabled: true });
		expect(result).toBe("?enabled=true");
	});

	it("encodes special characters", () => {
		const result = buildQueryString({ name: "hello world" });
		expect(result).toBe("?name=hello%20world");
	});

	it("omits undefined values", () => {
		const result = buildQueryString({ foo: "bar", baz: undefined });
		expect(result).toBe("?foo=bar");
	});

	it("omits null values", () => {
		const result = buildQueryString({ foo: "bar", baz: null as any });
		expect(result).toBe("?foo=bar");
	});

	it("omits empty string values", () => {
		const result = buildQueryString({ foo: "bar", baz: "" });
		expect(result).toBe("?foo=bar");
	});

	it("returns empty string for empty object", () => {
		expect(buildQueryString({})).toBe("");
	});
});

// ============================================================================
// getHostname
// ============================================================================

describe("getHostname", () => {
	it("extracts hostname from URL", () => {
		expect(getHostname("https://www.youtube.com/watch?v=abc")).toBe(
			"www.youtube.com"
		);
	});

	it("returns lowercase hostname", () => {
		expect(getHostname("https://WWW.YouTube.COM/watch?v=abc")).toBe(
			"www.youtube.com"
		);
	});

	it("handles protocol-relative URLs", () => {
		expect(getHostname("//vimeo.com/123456")).toBe("vimeo.com");
	});

	it("returns null for invalid URLs", () => {
		expect(getHostname("not a url")).toBeNull();
	});
});

// ============================================================================
// hostnameMatches
// ============================================================================

describe("hostnameMatches", () => {
	it("matches exact hostname", () => {
		expect(hostnameMatches("https://vimeo.com/123", ["vimeo.com"])).toBe(true);
	});

	it("matches subdomain", () => {
		expect(
			hostnameMatches("https://player.vimeo.com/video/123", ["vimeo.com"])
		).toBe(true);
	});

	it("matches any pattern in array", () => {
		expect(
			hostnameMatches("https://youtube.com/watch?v=abc", [
				"vimeo.com",
				"youtube.com",
			])
		).toBe(true);
	});

	it("returns false for non-matching hostname", () => {
		expect(hostnameMatches("https://example.com/video", ["vimeo.com"])).toBe(
			false
		);
	});

	it("is case-insensitive", () => {
		expect(hostnameMatches("https://VIMEO.COM/123", ["vimeo.com"])).toBe(true);
	});

	it("returns false for invalid URL", () => {
		expect(hostnameMatches("not a url", ["vimeo.com"])).toBe(false);
	});
});

// ============================================================================
// isSafeEmbedUrl
// ============================================================================

describe("isSafeEmbedUrl", () => {
	it("allows https URLs", () => {
		expect(isSafeEmbedUrl("https://example.com/video")).toBe(true);
	});

	it("allows http URLs", () => {
		expect(isSafeEmbedUrl("http://example.com/video")).toBe(true);
	});

	it("allows protocol-relative URLs", () => {
		expect(isSafeEmbedUrl("//example.com/video")).toBe(true);
	});

	it("blocks javascript: URLs", () => {
		expect(isSafeEmbedUrl("javascript:alert(1)")).toBe(false);
	});

	it("blocks data: URLs", () => {
		expect(isSafeEmbedUrl("data:text/html,<script>alert(1)</script>")).toBe(
			false
		);
	});

	it("blocks vbscript: URLs", () => {
		expect(isSafeEmbedUrl("vbscript:msgbox(1)")).toBe(false);
	});

	it("blocks file: URLs", () => {
		expect(isSafeEmbedUrl("file:///etc/passwd")).toBe(false);
	});

	it("is case-insensitive for protocol check", () => {
		expect(isSafeEmbedUrl("JAVASCRIPT:alert(1)")).toBe(false);
		expect(isSafeEmbedUrl("JavaScript:alert(1)")).toBe(false);
	});
});

// ============================================================================
// generateIframeHtml
// ============================================================================

describe("generateIframeHtml", () => {
	it("generates basic iframe", () => {
		const html = generateIframeHtml("https://example.com/embed");
		expect(html).toContain("<iframe");
		expect(html).toContain('src="https://example.com/embed"');
		expect(html).toContain('width="560"');
		expect(html).toContain('height="315"');
		expect(html).toContain("</iframe>");
	});

	it("sets custom dimensions", () => {
		const html = generateIframeHtml("https://example.com/embed", {
			width: 800,
			height: 450,
		});
		expect(html).toContain('width="800"');
		expect(html).toContain('height="450"');
	});

	it("sets title attribute", () => {
		const html = generateIframeHtml("https://example.com/embed", {
			title: "My Video",
		});
		expect(html).toContain('title="My Video"');
	});

	it("escapes title attribute", () => {
		const html = generateIframeHtml("https://example.com/embed", {
			title: 'Video "test" & more',
		});
		expect(html).toContain('title="Video &quot;test&quot; &amp; more"');
	});

	it("includes allowfullscreen by default", () => {
		const html = generateIframeHtml("https://example.com/embed");
		expect(html).toContain("allowfullscreen");
	});

	it("excludes allowfullscreen when disabled", () => {
		const html = generateIframeHtml("https://example.com/embed", {
			allowFullscreen: false,
		});
		expect(html).not.toContain("allowfullscreen");
	});

	it("sets allow attribute", () => {
		const html = generateIframeHtml("https://example.com/embed", {
			allow: "autoplay; fullscreen",
		});
		expect(html).toContain('allow="autoplay; fullscreen"');
	});

	it("sets loading attribute", () => {
		const htmlLazy = generateIframeHtml("https://example.com/embed", {
			loading: "lazy",
		});
		expect(htmlLazy).toContain('loading="lazy"');

		const htmlEager = generateIframeHtml("https://example.com/embed", {
			loading: "eager",
		});
		expect(htmlEager).toContain('loading="eager"');
	});

	it("sets sandbox attribute", () => {
		const html = generateIframeHtml("https://example.com/embed", {
			sandbox: "allow-scripts allow-same-origin",
		});
		expect(html).toContain('sandbox="allow-scripts allow-same-origin"');
	});

	it("sets class attribute", () => {
		const html = generateIframeHtml("https://example.com/embed", {
			className: "video-embed",
		});
		expect(html).toContain('class="video-embed"');
	});
});

// ============================================================================
// escapeHtmlAttribute
// ============================================================================

describe("escapeHtmlAttribute", () => {
	it("escapes ampersand", () => {
		expect(escapeHtmlAttribute("a & b")).toBe("a &amp; b");
	});

	it("escapes double quotes", () => {
		expect(escapeHtmlAttribute('say "hello"')).toBe("say &quot;hello&quot;");
	});

	it("escapes single quotes", () => {
		expect(escapeHtmlAttribute("it's")).toBe("it&#39;s");
	});

	it("escapes angle brackets", () => {
		expect(escapeHtmlAttribute("<script>")).toBe("&lt;script&gt;");
	});

	it("handles multiple characters", () => {
		expect(escapeHtmlAttribute('<a href="test?a=1&b=2">')).toBe(
			"&lt;a href=&quot;test?a=1&amp;b=2&quot;&gt;"
		);
	});
});

// ============================================================================
// secondsToTimestamp
// ============================================================================

describe("secondsToTimestamp", () => {
	it("converts seconds only", () => {
		expect(secondsToTimestamp(45)).toBe("45s");
	});

	it("converts minutes and seconds", () => {
		expect(secondsToTimestamp(90)).toBe("1m30s");
	});

	it("converts hours, minutes, and seconds", () => {
		expect(secondsToTimestamp(3723)).toBe("1h2m3s");
	});

	it("handles zero", () => {
		expect(secondsToTimestamp(0)).toBe("0s");
	});

	it("handles exact minutes", () => {
		expect(secondsToTimestamp(120)).toBe("2m");
	});

	it("handles exact hours", () => {
		expect(secondsToTimestamp(3600)).toBe("1h");
	});

	it("omits zero components", () => {
		expect(secondsToTimestamp(3660)).toBe("1h1m"); // 1 hour, 1 minute, 0 seconds
	});
});

// ============================================================================
// timestampToSeconds
// ============================================================================

describe("timestampToSeconds", () => {
	describe("YouTube format (1h2m3s)", () => {
		it("parses seconds only", () => {
			expect(timestampToSeconds("45s")).toBe(45);
		});

		it("parses minutes and seconds", () => {
			expect(timestampToSeconds("1m30s")).toBe(90);
		});

		it("parses hours, minutes, and seconds", () => {
			expect(timestampToSeconds("1h2m3s")).toBe(3723);
		});

		it("parses hours only", () => {
			expect(timestampToSeconds("2h")).toBe(7200);
		});

		it("parses minutes only", () => {
			expect(timestampToSeconds("5m")).toBe(300);
		});

		it("parses hours and minutes without seconds", () => {
			expect(timestampToSeconds("1h30m")).toBe(5400);
		});

		it("parses hours and seconds without minutes", () => {
			expect(timestampToSeconds("1h30s")).toBe(3630);
		});

		it("is case-insensitive", () => {
			expect(timestampToSeconds("1H2M3S")).toBe(3723);
		});
	});

	describe("colon format (h:mm:ss or m:ss)", () => {
		it("parses m:ss format", () => {
			expect(timestampToSeconds("1:30")).toBe(90);
		});

		it("parses h:mm:ss format", () => {
			expect(timestampToSeconds("1:02:03")).toBe(3723);
		});

		it("handles leading zeros", () => {
			expect(timestampToSeconds("01:30")).toBe(90);
			expect(timestampToSeconds("1:05:00")).toBe(3900);
		});
	});

	describe("plain seconds", () => {
		it("parses plain number", () => {
			expect(timestampToSeconds("120")).toBe(120);
		});

		it("parses zero", () => {
			expect(timestampToSeconds("0")).toBe(0);
		});
	});

	describe("invalid input", () => {
		it("returns null for empty string", () => {
			expect(timestampToSeconds("")).toBeNull();
		});

		it("returns null for non-numeric text", () => {
			expect(timestampToSeconds("hello")).toBeNull();
		});

		it("returns null for malformed timestamp", () => {
			expect(timestampToSeconds("1:2:3:4")).toBeNull();
			expect(timestampToSeconds("abc123")).toBeNull();
		});
	});
});
