import { describe, expect, it } from "vitest";
import {
	generic,
	isDomainAllowed,
	renderGenericEmbed,
	validateEmbedHtml,
} from "../../src/embed/providers/generic";

describe("embed/providers/generic", () => {
	it("parses supported URLs and rejects unsafe/non-url IDs", () => {
		const parsed = generic.parse("https://example.com/embed/123");
		expect(parsed?.provider).toBe("generic");
		expect(parsed?.fallbackUrl).toBe("https://example.com/embed/123");

		expect(generic.parse("javascript:alert(1)")).toBeNull();
		expect(generic.parse("data:text/html,hi")).toBeNull();
		expect(generic.parse("raw-id-without-html")).toBeNull();
		expect(generic.parse("   ")).toBeNull();
	});

	it("returns direct URL ids from getEmbedUrl and rejects non-url ids", () => {
		expect(generic.getEmbedUrl("https://example.com/embed/1")).toBe(
			"https://example.com/embed/1"
		);
		expect(generic.getEmbedUrl("http://example.com/embed/1")).toBe(
			"http://example.com/embed/1"
		);
		expect(generic.getEmbedUrl("abc123")).toBe("");
	});

	it("parses passthrough embed HTML", () => {
		const html = `<iframe src="https://player.example.com/x"></iframe>`;
		const parsed = generic.parse(html);
		expect(parsed?.isPassthrough).toBe(true);
		expect(parsed?.embedHtml).toBe(html);
		expect(renderGenericEmbed(parsed!)).toBe(html);
	});

	it("renders fallback iframe for URL-based generic embeds", () => {
		const parsed = generic.parse("https://video.example.com/abc")!;
		const out = renderGenericEmbed(parsed, {
			width: 800,
			height: 450,
			title: "External Media",
			loading: "eager",
			className: "embed-frame",
		});
		expect(out).toContain(`src="https://video.example.com/abc"`);
		expect(out).toContain(`width="800"`);
		expect(out).toContain(`height="450"`);
		expect(out).toContain(`title="External Media"`);
		expect(out).toContain(`loading="eager"`);
		expect(out).toContain(`class="embed-frame"`);
	});

	it("returns empty output when generic embed has neither passthrough HTML nor fallback URL", () => {
		const out = renderGenericEmbed({ provider: "generic", id: "x" });
		expect(out).toBe("");
	});

	it("checks domain allow-list logic", () => {
		expect(isDomainAllowed("https://media.example.com/video", [])).toBe(true);
		expect(
			isDomainAllowed("https://media.example.com/video", ["example.com"])
		).toBe(true);
		expect(isDomainAllowed("https://evil.com/video", ["example.com"])).toBe(
			false
		);
		expect(isDomainAllowed("not-a-url", ["example.com"])).toBe(false);
	});

	it("validates unsafe embed html patterns", () => {
		expect(validateEmbedHtml(`<iframe src="https://safe.example.com"></iframe>`)).toEqual(
			{ valid: true }
		);
		expect(validateEmbedHtml(`<img src=x onload="alert(1)" />`)).toEqual({
			valid: false,
			reason: 'Event handler "onload" not allowed',
		});
		expect(validateEmbedHtml(`<a href="javascript:alert(1)">x</a>`)).toEqual({
			valid: false,
			reason: "JavaScript URLs not allowed",
		});
		expect(validateEmbedHtml(`<iframe src="data:text/html;base64,AAAA"></iframe>`)).toEqual(
			{
				valid: false,
				reason: "Data URLs in src not allowed",
			}
		);
		expect(validateEmbedHtml(`<script>alert(1)</script>`)).toEqual({
			valid: false,
			reason: "Inline script content not allowed",
		});
		expect(
			validateEmbedHtml(`<script src="https://cdn.example.com/embed.js"></script>`)
		).toEqual({ valid: true });
	});
});
