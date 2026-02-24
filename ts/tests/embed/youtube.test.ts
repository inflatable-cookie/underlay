import { describe, expect, it, vi } from "vitest";
import { youtube } from "../../src/embed/providers/youtube";

describe("embed/providers/youtube", () => {
	it("parses raw ids and multiple youtube URL formats", () => {
		expect(youtube.parse("dQw4w9WgXcQ")).toEqual({ provider: "youtube", id: "dQw4w9WgXcQ" });
		expect(youtube.parse("https://www.youtube.com/watch?v=dQw4w9WgXcQ")).toEqual(
			expect.objectContaining({ provider: "youtube", id: "dQw4w9WgXcQ" })
		);
		expect(youtube.parse("https://youtu.be/dQw4w9WgXcQ?t=1m30s")).toEqual(
			expect.objectContaining({ provider: "youtube", id: "dQw4w9WgXcQ", queryParams: expect.objectContaining({ start: "90" }) })
		);
		expect(youtube.parse("https://www.youtube.com/embed/dQw4w9WgXcQ?end=2m")).toEqual(
			expect.objectContaining({ queryParams: expect.objectContaining({ end: "120" }) })
		);
		expect(youtube.parse("https://www.youtube.com/shorts/dQw4w9WgXcQ")).toEqual(
			expect.objectContaining({ id: "dQw4w9WgXcQ" })
		);
		expect(youtube.parse("https://example.com/video/1")).toBeNull();
	});

	it("builds embed URLs with privacy and player options", () => {
		expect(youtube.getEmbedUrl("abc123xyz00")).toBe(
			"https://www.youtube.com/embed/abc123xyz00?rel=0"
		);
		expect(
			youtube.getEmbedUrl("abc123xyz00", {
				consent: false,
				autoplay: true,
				startTime: 12,
				endTime: 30,
				enableApi: true,
				origin: "https://app.example",
			})
		).toBe(
			"https://www.youtube-nocookie.com/embed/abc123xyz00?autoplay=1&start=12&end=30&enablejsapi=1&origin=https%3A%2F%2Fapp.example&rel=0"
		);
	});

	it("builds expected thumbnail URLs and looks up oEmbed metadata", async () => {
		expect(youtube.getThumbnailUrl("id", "default")).toBe("https://img.youtube.com/vi/id/default.jpg");
		expect(youtube.getThumbnailUrl("id", "medium")).toBe("https://img.youtube.com/vi/id/mqdefault.jpg");
		expect(youtube.getThumbnailUrl("id", "high")).toBe("https://img.youtube.com/vi/id/hqdefault.jpg");
		expect(youtube.getThumbnailUrl("id", "max")).toBe("https://img.youtube.com/vi/id/maxresdefault.jpg");

		const ok = vi.fn().mockResolvedValue({
			ok: true,
			json: async () => ({
				title: "Video",
				author_name: "Author",
				author_url: "https://author",
				thumbnail_url: "https://img",
				thumbnail_width: 480,
				thumbnail_height: 360,
			}),
		});
		await expect(youtube.lookupMeta("id", ok as any)).resolves.toEqual({
			title: "Video",
			authorName: "Author",
			authorUrl: "https://author",
			thumbnailUrl: "https://img",
			thumbnailWidth: 480,
			thumbnailHeight: 360,
		});

		const bad = vi.fn().mockResolvedValue({ ok: false });
		await expect(youtube.lookupMeta("id", bad as any)).resolves.toBeNull();
		const throws = vi.fn().mockRejectedValue(new Error("x"));
		await expect(youtube.lookupMeta("id", throws as any)).resolves.toBeNull();
	});
});
