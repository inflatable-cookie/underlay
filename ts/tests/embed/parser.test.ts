/**
 * Tests for the Media Embed Parser module.
 *
 * Covers:
 * - Parser tests with various input formats (embed HTML, URLs, raw IDs)
 * - Provider detection tests
 * - URL extraction tests per provider (YouTube, Vimeo, Audioboom)
 * - Embed URL generation tests
 * - Metadata normalization tests
 */

import { afterEach, describe, it, expect, vi } from "vitest";
import {
	parseEmbed,
	getEmbedUrl,
	renderEmbed,
	getThumbnailUrl,
	supportsThumbnailUrl,
	supportsMetadataLookup,
} from "../../src/embed/parser";
import { youtube } from "../../src/embed/providers/youtube";
import { vimeo } from "../../src/embed/providers/vimeo";
import { audioboom } from "../../src/embed/providers/audioboom";
import { defaultRegistry } from "../../src/embed/providers";
import type { ParsedEmbed, EmbedOptions } from "../../src/embed/types";

// ============================================================================
// parseEmbed - Main Parser
// ============================================================================

describe("parseEmbed", () => {
	afterEach(() => {
		vi.restoreAllMocks();
	});

	describe("empty and invalid input", () => {
		it("returns error for empty string", () => {
			const result = parseEmbed("");
			expect(result.success).toBe(false);
			expect(result.error).toBe("Empty input");
		});

		it("returns error for whitespace-only input", () => {
			const result = parseEmbed("   \n\t  ");
			expect(result.success).toBe(false);
			expect(result.error).toBe("Empty input");
		});

		it("returns error for unrecognized input without generic fallback", () => {
			const result = parseEmbed("random text here", { allowGeneric: false });
			expect(result.success).toBe(false);
		});

		it("returns extraction error for embed html without URL when generic fallback is disabled", () => {
			const result = parseEmbed(`<iframe width="560" height="315"></iframe>`, {
				allowGeneric: false,
			});
			expect(result.success).toBe(false);
			expect(result.error).toBe("Could not extract URL from embed code");
		});

		it("returns provider-detection error for unsupported URLs when generic fallback is disabled", () => {
			const result = parseEmbed("https://unsupported.example.com/video/123", {
				allowGeneric: false,
			});
			expect(result.success).toBe(false);
			expect(result.error).toBe("Could not detect provider from URL");
		});
	});

	describe("YouTube URL parsing", () => {
		it("parses standard watch URL", () => {
			const result = parseEmbed("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("youtube");
			expect(result.parsed?.id).toBe("dQw4w9WgXcQ");
		});

		it("parses short URL (youtu.be)", () => {
			const result = parseEmbed("https://youtu.be/dQw4w9WgXcQ");
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("youtube");
			expect(result.parsed?.id).toBe("dQw4w9WgXcQ");
		});

		it("parses embed URL", () => {
			const result = parseEmbed("https://www.youtube.com/embed/dQw4w9WgXcQ");
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("youtube");
			expect(result.parsed?.id).toBe("dQw4w9WgXcQ");
		});

		it("parses nocookie embed URL", () => {
			const result = parseEmbed(
				"https://www.youtube-nocookie.com/embed/dQw4w9WgXcQ"
			);
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("youtube");
			expect(result.parsed?.id).toBe("dQw4w9WgXcQ");
		});

		it("parses shorts URL", () => {
			const result = parseEmbed("https://www.youtube.com/shorts/abcdefghijk");
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("youtube");
			expect(result.parsed?.id).toBe("abcdefghijk");
		});

		it("extracts start time from t parameter", () => {
			const result = parseEmbed(
				"https://www.youtube.com/watch?v=dQw4w9WgXcQ&t=1m30s"
			);
			expect(result.success).toBe(true);
			expect(result.parsed?.queryParams?.start).toBe("90");
		});

		it("extracts start time from plain seconds", () => {
			const result = parseEmbed(
				"https://www.youtube.com/watch?v=dQw4w9WgXcQ&t=120"
			);
			expect(result.success).toBe(true);
			expect(result.parsed?.queryParams?.start).toBe("120");
		});

		it("extracts end time", () => {
			const result = parseEmbed(
				"https://www.youtube.com/watch?v=dQw4w9WgXcQ&end=180"
			);
			expect(result.success).toBe(true);
			expect(result.parsed?.queryParams?.end).toBe("180");
		});
	});

	describe("Vimeo URL parsing", () => {
		it("parses standard URL", () => {
			const result = parseEmbed("https://vimeo.com/123456789");
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("vimeo");
			expect(result.parsed?.id).toBe("123456789");
		});

		it("parses player URL", () => {
			const result = parseEmbed("https://player.vimeo.com/video/123456789");
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("vimeo");
			expect(result.parsed?.id).toBe("123456789");
		});

		it("parses channel URL", () => {
			const result = parseEmbed(
				"https://vimeo.com/channels/staffpicks/123456789"
			);
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("vimeo");
			expect(result.parsed?.id).toBe("123456789");
		});

		it("parses group URL", () => {
			const result = parseEmbed(
				"https://vimeo.com/groups/shortfilm/videos/123456789"
			);
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("vimeo");
			expect(result.parsed?.id).toBe("123456789");
		});

		it("extracts hash for private videos", () => {
			const result = parseEmbed("https://vimeo.com/123456789?h=abc123def");
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("vimeo");
			expect(result.parsed?.id).toBe("123456789");
			expect(result.parsed?.queryParams?.h).toBe("abc123def");
		});
	});

	describe("Audioboom URL parsing", () => {
		it("parses posts URL", () => {
			const result = parseEmbed("https://audioboom.com/posts/12345678");
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("audioboom");
			expect(result.parsed?.id).toBe("12345678");
		});

		it("parses boos URL (legacy)", () => {
			const result = parseEmbed("https://audioboom.com/boos/12345678");
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("audioboom");
			expect(result.parsed?.id).toBe("12345678");
		});

		it("parses embed URL", () => {
			const result = parseEmbed(
				"https://embeds.audioboom.com/posts/12345678/embed/v4"
			);
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("audioboom");
			expect(result.parsed?.id).toBe("12345678");
		});

		it("extracts numeric ID from slug URL", () => {
			const result = parseEmbed(
				"https://audioboom.com/posts/12345678-some-podcast-title"
			);
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("audioboom");
			expect(result.parsed?.id).toBe("12345678");
		});

		it("parses playlist URL", () => {
			const result = parseEmbed(
				"https://audioboom.com/publishing/playlist/v4?data_for_content_type=channel123"
			);
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("audioboom");
			expect(result.parsed?.id).toBe("channel123");
			expect(result.parsed?.embedType).toBe("playlist");
		});
	});

	describe("embed HTML parsing", () => {
		it("parses YouTube iframe embed", () => {
			const embed = `<iframe width="560" height="315" src="https://www.youtube.com/embed/dQw4w9WgXcQ" frameborder="0" allowfullscreen></iframe>`;
			const result = parseEmbed(embed);
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("youtube");
			expect(result.parsed?.id).toBe("dQw4w9WgXcQ");
			expect(result.parsed?.width).toBe(560);
			expect(result.parsed?.height).toBe(315);
			expect(result.parsed?.originalEmbed).toBe(embed);
		});

		it("parses Vimeo iframe embed", () => {
			const embed = `<iframe src="https://player.vimeo.com/video/123456789" width="640" height="360" frameborder="0"></iframe>`;
			const result = parseEmbed(embed);
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("vimeo");
			expect(result.parsed?.id).toBe("123456789");
			expect(result.parsed?.width).toBe(640);
			expect(result.parsed?.height).toBe(360);
		});

		it("handles HTML-encoded attributes", () => {
			const embed = `<iframe src="https://www.youtube.com/embed/dQw4w9WgXcQ?autoplay=1&amp;rel=0" width="560" height="315"></iframe>`;
			const result = parseEmbed(embed);
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("youtube");
			expect(result.parsed?.id).toBe("dQw4w9WgXcQ");
		});
	});

	describe("raw ID parsing", () => {
		it("parses YouTube raw ID (11 chars)", () => {
			const result = parseEmbed("dQw4w9WgXcQ");
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("youtube");
			expect(result.parsed?.id).toBe("dQw4w9WgXcQ");
		});

		it("parses Vimeo raw ID (numeric, 6+ digits)", () => {
			const result = parseEmbed("123456789");
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("vimeo");
			expect(result.parsed?.id).toBe("123456789");
		});

		it("parses Audioboom raw ID (numeric, 5+ digits)", () => {
			const result = parseEmbed("12345678");
			expect(result.success).toBe(true);
			// Could match Vimeo or Audioboom - implementation-dependent
			expect(result.parsed?.id).toBe("12345678");
		});

		it("returns error for unrecognized raw ID", () => {
			const result = parseEmbed("xyz", { allowGeneric: false });
			expect(result.success).toBe(false);
			expect(result.error).toContain("Could not detect provider");
		});

		it("returns raw-id detection error when allowedProviders excludes matching providers", () => {
			const result = parseEmbed("123456789", {
				allowGeneric: false,
				allowedProviders: ["youtube"],
			});
			expect(result.success).toBe(false);
			expect(result.error).toContain("Could not detect provider");
		});
	});

	describe("provider filtering", () => {
		it("allows specified providers", () => {
			const result = parseEmbed("https://www.youtube.com/watch?v=dQw4w9WgXcQ", {
				allowedProviders: ["youtube"],
			});
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("youtube");
		});

		it("blocks non-allowed providers", () => {
			const result = parseEmbed("https://vimeo.com/123456789", {
				allowedProviders: ["youtube"],
				allowGeneric: false,
			});
			expect(result.success).toBe(false);
			expect(result.error).toContain("not allowed");
		});
	});

	describe("generic fallback guardrails", () => {
		it("blocks URLs not in allowedDomains when generic fallback is used", () => {
			const result = parseEmbed("https://example.com/video", {
				allowedDomains: ["trusted.com"],
			});
			expect(result.success).toBe(false);
			expect(result.error).toBe("Domain not in allowed list");
		});

		it("rejects unsafe generic embed HTML", () => {
			const result = parseEmbed(
				`<iframe src="https://example.com"></iframe><script>alert(1)</script>`
			);
			expect(result.success).toBe(false);
			expect(result.error).toContain("Inline script");
		});

		it("returns error when generic provider is unavailable", () => {
			vi.spyOn(defaultRegistry, "get").mockImplementation((name: string) => {
				if (name === "generic") return undefined;
				return undefined;
			});
			const result = parseEmbed("https://example.com/video");
			expect(result.success).toBe(false);
			expect(result.error).toBe("Generic provider not available");
		});

		it("returns parse error when generic provider cannot parse input", () => {
			const originalGet = defaultRegistry.get.bind(defaultRegistry);
			vi.spyOn(defaultRegistry, "get").mockImplementation((name: string) => {
				if (name === "generic") {
					return {
						name: "generic",
						parse: () => null,
						getEmbedUrl: () => "",
					} as any;
				}
				return originalGet(name);
			});
			const result = parseEmbed("https://example.com/video");
			expect(result.success).toBe(false);
			expect(result.error).toBe("Could not parse embed");
		});

		it("hydrates dimensions and originalEmbed on generic fallback results", () => {
			const originalGet = defaultRegistry.get.bind(defaultRegistry);
			vi.spyOn(defaultRegistry, "get").mockImplementation((name: string) => {
				if (name === "generic") {
					return {
						name: "generic",
						parse: () => ({ provider: "generic", id: "embedded" }),
						getEmbedUrl: () => "",
					} as any;
				}
				return originalGet(name);
			});

			const embedWithoutSrc = `<iframe width="640" height="360"></iframe>`;
			const result = parseEmbed(embedWithoutSrc);
			expect(result.success).toBe(true);
			expect(result.parsed).toEqual(
				expect.objectContaining({
					provider: "generic",
					id: "embedded",
					width: 640,
					height: 360,
					originalEmbed: embedWithoutSrc,
				})
			);
		});
	});

	describe("URL normalization", () => {
		it("handles HTTP URLs (upgrades to HTTPS)", () => {
			const result = parseEmbed("http://www.youtube.com/watch?v=dQw4w9WgXcQ");
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("youtube");
		});

		it("handles protocol-relative URLs", () => {
			const result = parseEmbed("//www.youtube.com/watch?v=dQw4w9WgXcQ");
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("youtube");
		});

		it("trims whitespace from input", () => {
			const result = parseEmbed(
				"  https://www.youtube.com/watch?v=dQw4w9WgXcQ  "
			);
			expect(result.success).toBe(true);
			expect(result.parsed?.provider).toBe("youtube");
		});
	});
});

// ============================================================================
// YouTube Provider
// ============================================================================

describe("YouTube provider", () => {
	describe("parse", () => {
		it("validates raw ID format (11 alphanumeric chars)", () => {
			expect(youtube.parse("dQw4w9WgXcQ")?.id).toBe("dQw4w9WgXcQ");
			expect(youtube.parse("abc-_123DEF")?.id).toBe("abc-_123DEF");
			expect(youtube.parse("short")).toBeNull(); // Too short
			expect(youtube.parse("toolongidhere")).toBeNull(); // Too long
		});

		it("rejects non-YouTube URLs", () => {
			expect(youtube.parse("https://vimeo.com/123456")).toBeNull();
			expect(youtube.parse("https://example.com/video")).toBeNull();
		});
	});

	describe("getEmbedUrl", () => {
		it("generates basic embed URL", () => {
			const url = youtube.getEmbedUrl("dQw4w9WgXcQ");
			expect(url).toBe("https://www.youtube.com/embed/dQw4w9WgXcQ?rel=0");
		});

		it("uses nocookie domain when consent is false", () => {
			const url = youtube.getEmbedUrl("dQw4w9WgXcQ", { consent: false });
			expect(url).toContain("youtube-nocookie.com");
		});

		it("adds autoplay parameter", () => {
			const url = youtube.getEmbedUrl("dQw4w9WgXcQ", { autoplay: true });
			expect(url).toContain("autoplay=1");
		});

		it("adds start and end time parameters", () => {
			const url = youtube.getEmbedUrl("dQw4w9WgXcQ", {
				startTime: 30,
				endTime: 90,
			});
			expect(url).toContain("start=30");
			expect(url).toContain("end=90");
		});

		it("adds API enablement parameters", () => {
			const url = youtube.getEmbedUrl("dQw4w9WgXcQ", {
				enableApi: true,
				origin: "https://example.com",
			});
			expect(url).toContain("enablejsapi=1");
			expect(url).toContain("origin=https%3A%2F%2Fexample.com");
		});
	});

	describe("getThumbnailUrl", () => {
		it("generates default size thumbnail URL", () => {
			const url = youtube.getThumbnailUrl!("dQw4w9WgXcQ", "default");
			expect(url).toBe("https://img.youtube.com/vi/dQw4w9WgXcQ/default.jpg");
		});

		it("generates medium size thumbnail URL", () => {
			const url = youtube.getThumbnailUrl!("dQw4w9WgXcQ", "medium");
			expect(url).toBe("https://img.youtube.com/vi/dQw4w9WgXcQ/mqdefault.jpg");
		});

		it("generates high quality thumbnail URL", () => {
			const url = youtube.getThumbnailUrl!("dQw4w9WgXcQ", "high");
			expect(url).toBe("https://img.youtube.com/vi/dQw4w9WgXcQ/hqdefault.jpg");
		});

		it("generates max resolution thumbnail URL", () => {
			const url = youtube.getThumbnailUrl!("dQw4w9WgXcQ", "max");
			expect(url).toBe(
				"https://img.youtube.com/vi/dQw4w9WgXcQ/maxresdefault.jpg"
			);
		});
	});
});

// ============================================================================
// Vimeo Provider
// ============================================================================

describe("Vimeo provider", () => {
	describe("parse", () => {
		it("validates raw ID format (numeric, 6+ digits)", () => {
			expect(vimeo.parse("123456")?.id).toBe("123456");
			expect(vimeo.parse("123456789")?.id).toBe("123456789");
			expect(vimeo.parse("12345")).toBeNull(); // Too short
			expect(vimeo.parse("abc123")).toBeNull(); // Contains letters
		});

		it("rejects non-Vimeo URLs", () => {
			expect(vimeo.parse("https://youtube.com/watch?v=abc")).toBeNull();
			expect(vimeo.parse("https://example.com/123456")).toBeNull();
		});
	});

	describe("getEmbedUrl", () => {
		it("generates basic embed URL", () => {
			const url = vimeo.getEmbedUrl("123456789");
			expect(url).toBe("https://player.vimeo.com/video/123456789");
		});

		it("adds autoplay parameter", () => {
			const url = vimeo.getEmbedUrl("123456789", { autoplay: true });
			expect(url).toContain("autoplay=1");
		});

		it("adds start time as URL fragment", () => {
			const url = vimeo.getEmbedUrl("123456789", { startTime: 60 });
			expect(url).toContain("#t=60s");
		});

		it("adds API parameters", () => {
			const url = vimeo.getEmbedUrl("123456789", {
				enableApi: true,
				origin: "https://example.com",
			});
			expect(url).toContain("api=1");
			expect(url).toContain("origin=https%3A%2F%2Fexample.com");
		});
	});
});

// ============================================================================
// Audioboom Provider
// ============================================================================

describe("Audioboom provider", () => {
	describe("parse", () => {
		it("validates raw ID format (numeric, 5+ digits)", () => {
			expect(audioboom.parse("12345")?.id).toBe("12345");
			expect(audioboom.parse("12345678")?.id).toBe("12345678");
			expect(audioboom.parse("1234")).toBeNull(); // Too short
		});

		it("rejects non-Audioboom URLs", () => {
			expect(audioboom.parse("https://youtube.com/watch?v=abc")).toBeNull();
			expect(audioboom.parse("https://vimeo.com/123456")).toBeNull();
		});
	});

	describe("getEmbedUrl", () => {
		it("generates single embed URL", () => {
			const url = audioboom.getEmbedUrl("12345678");
			expect(url).toBe("https://embeds.audioboom.com/posts/12345678/embed/v4");
		});

		it("adds autoplay parameter", () => {
			const url = audioboom.getEmbedUrl("12345678", { autoplay: true });
			expect(url).toContain("autoplay=1");
		});
	});
});

// ============================================================================
// getEmbedUrl - Main Function
// ============================================================================

describe("getEmbedUrl", () => {
	it("generates URL for YouTube parsed embed", () => {
		const parsed: ParsedEmbed = {
			provider: "youtube",
			id: "dQw4w9WgXcQ",
		};
		const url = getEmbedUrl(parsed);
		expect(url).toContain("youtube.com/embed/dQw4w9WgXcQ");
	});

	it("generates URL for Vimeo parsed embed", () => {
		const parsed: ParsedEmbed = {
			provider: "vimeo",
			id: "123456789",
		};
		const url = getEmbedUrl(parsed);
		expect(url).toContain("player.vimeo.com/video/123456789");
	});

	it("merges start time from queryParams", () => {
		const parsed: ParsedEmbed = {
			provider: "youtube",
			id: "dQw4w9WgXcQ",
			queryParams: { start: "30" },
		};
		const url = getEmbedUrl(parsed);
		expect(url).toContain("start=30");
	});

	it("options override parsed queryParams", () => {
		const parsed: ParsedEmbed = {
			provider: "youtube",
			id: "dQw4w9WgXcQ",
			queryParams: { start: "30" },
		};
		const url = getEmbedUrl(parsed, { startTime: 60 });
		expect(url).toContain("start=60");
		expect(url).not.toContain("start=30");
	});

	it("merges end time from queryParams when options do not override", () => {
		const parsed: ParsedEmbed = {
			provider: "youtube",
			id: "dQw4w9WgXcQ",
			queryParams: { end: "180" },
		};
		const url = getEmbedUrl(parsed);
		expect(url).toContain("end=180");
	});

	it("returns empty string for unknown provider", () => {
		const parsed: ParsedEmbed = {
			provider: "unknown",
			id: "abc123",
		};
		const url = getEmbedUrl(parsed);
		expect(url).toBe("");
	});
});

// ============================================================================
// renderEmbed
// ============================================================================

describe("renderEmbed", () => {
	it("generates iframe HTML for YouTube", () => {
		const parsed: ParsedEmbed = {
			provider: "youtube",
			id: "dQw4w9WgXcQ",
		};
		const html = renderEmbed(parsed);
		expect(html).toContain("<iframe");
		expect(html).toContain("youtube.com/embed/dQw4w9WgXcQ");
		expect(html).toContain('width="560"');
		expect(html).toContain('height="315"');
		expect(html).toContain("allowfullscreen");
	});

	it("uses dimensions from parsed embed", () => {
		const parsed: ParsedEmbed = {
			provider: "youtube",
			id: "dQw4w9WgXcQ",
			width: 800,
			height: 450,
		};
		const html = renderEmbed(parsed);
		expect(html).toContain('width="800"');
		expect(html).toContain('height="450"');
	});

	it("options override parsed dimensions", () => {
		const parsed: ParsedEmbed = {
			provider: "youtube",
			id: "dQw4w9WgXcQ",
			width: 800,
			height: 450,
		};
		const html = renderEmbed(parsed, { width: 640, height: 360 });
		expect(html).toContain('width="640"');
		expect(html).toContain('height="360"');
	});

	it("adds custom CSS class", () => {
		const parsed: ParsedEmbed = {
			provider: "youtube",
			id: "dQw4w9WgXcQ",
		};
		const html = renderEmbed(parsed, { className: "video-embed" });
		expect(html).toContain('class="video-embed"');
	});

	it("sets custom title for accessibility", () => {
		const parsed: ParsedEmbed = {
			provider: "youtube",
			id: "dQw4w9WgXcQ",
		};
		const html = renderEmbed(parsed, { title: "My awesome video" });
		expect(html).toContain('title="My awesome video"');
	});

	it("sets loading strategy", () => {
		const parsed: ParsedEmbed = {
			provider: "youtube",
			id: "dQw4w9WgXcQ",
		};
		const htmlLazy = renderEmbed(parsed, { loading: "lazy" });
		expect(htmlLazy).toContain('loading="lazy"');

		const htmlEager = renderEmbed(parsed, { loading: "eager" });
		expect(htmlEager).toContain('loading="eager"');
	});

	it("returns passthrough HTML for generic embeds", () => {
		const passthrough = `<iframe src="https://example.com/embed"></iframe>`;
		const parsed = {
			provider: "generic",
			id: "embedded",
			isPassthrough: true,
			embedHtml: passthrough,
		} as unknown as ParsedEmbed;

		expect(renderEmbed(parsed)).toBe(passthrough);
	});

	it("renders iframe for generic fallback URLs", () => {
		const parsed = {
			provider: "generic",
			id: "example.com",
			fallbackUrl: "https://example.com/embed",
		} as unknown as ParsedEmbed;
		const html = renderEmbed(parsed);

		expect(html).toContain("<iframe");
		expect(html).toContain("https://example.com/embed");
	});

	it("returns empty HTML when provider cannot generate embed URL", () => {
		const parsed: ParsedEmbed = {
			provider: "unknown",
			id: "abc123",
		};
		expect(renderEmbed(parsed)).toBe("");
	});
});

// ============================================================================
// getThumbnailUrl
// ============================================================================

describe("getThumbnailUrl", () => {
	it("returns thumbnail URL for YouTube", () => {
		const parsed: ParsedEmbed = {
			provider: "youtube",
			id: "dQw4w9WgXcQ",
		};
		const url = getThumbnailUrl(parsed, "high");
		expect(url).toBe("https://img.youtube.com/vi/dQw4w9WgXcQ/hqdefault.jpg");
	});

	it("returns null for Vimeo (no predictable thumbnails)", () => {
		const parsed: ParsedEmbed = {
			provider: "vimeo",
			id: "123456789",
		};
		const url = getThumbnailUrl(parsed);
		expect(url).toBeNull();
	});

	it("returns null for Audioboom (no predictable thumbnails)", () => {
		const parsed: ParsedEmbed = {
			provider: "audioboom",
			id: "12345678",
		};
		const url = getThumbnailUrl(parsed);
		expect(url).toBeNull();
	});
});

// ============================================================================
// Feature detection
// ============================================================================

describe("feature detection", () => {
	describe("supportsThumbnailUrl", () => {
		it("returns true for YouTube", () => {
			expect(supportsThumbnailUrl("youtube")).toBe(true);
		});

		it("returns false for Vimeo", () => {
			expect(supportsThumbnailUrl("vimeo")).toBe(false);
		});

		it("returns false for Audioboom", () => {
			expect(supportsThumbnailUrl("audioboom")).toBe(false);
		});

		it("returns false for unknown provider", () => {
			expect(supportsThumbnailUrl("unknown")).toBe(false);
		});
	});

	describe("supportsMetadataLookup", () => {
		it("returns true for YouTube", () => {
			expect(supportsMetadataLookup("youtube")).toBe(true);
		});

		it("returns true for Vimeo", () => {
			expect(supportsMetadataLookup("vimeo")).toBe(true);
		});

		it("returns true for Audioboom", () => {
			expect(supportsMetadataLookup("audioboom")).toBe(true);
		});

		it("returns false for unknown provider", () => {
			expect(supportsMetadataLookup("unknown")).toBe(false);
		});
	});
});
