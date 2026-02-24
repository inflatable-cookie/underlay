import { describe, expect, it, vi } from "vitest";
import { vimeo } from "../../src/embed/providers/vimeo";

describe("embed/providers/vimeo", () => {
	it("parses raw ids and supported vimeo URL formats", () => {
		expect(vimeo.parse("12345678")).toEqual({ provider: "vimeo", id: "12345678" });
		expect(vimeo.parse("https://vimeo.com/12345678")).toEqual(
			expect.objectContaining({ provider: "vimeo", id: "12345678" })
		);
		expect(vimeo.parse("https://player.vimeo.com/video/987654")).toEqual(
			expect.objectContaining({ provider: "vimeo", id: "987654" })
		);
		expect(vimeo.parse("https://vimeo.com/channels/staffpicks/13579")).toEqual(
			expect.objectContaining({ provider: "vimeo", id: "13579" })
		);
		expect(vimeo.parse("https://vimeo.com/groups/name/videos/24680")).toEqual(
			expect.objectContaining({ provider: "vimeo", id: "24680" })
		);
		expect(vimeo.parse("https://vimeo.com/123456?h=secret")).toEqual(
			expect.objectContaining({ provider: "vimeo", id: "123456", queryParams: expect.objectContaining({ h: "secret" }) })
		);
		expect(vimeo.parse("https://player.vimeo.com/video/not-numeric")).toBeNull();
		expect(vimeo.parse("https://vimeo.com/channels/staffpicks/not-numeric")).toBeNull();
		expect(vimeo.parse("https://vimeo.com/groups/name/videos/not-numeric")).toBeNull();
		expect(vimeo.parse("https://vimeo.com/not-numeric")).toBeNull();
		expect(vimeo.parse("https://vimeo.com/123456")).toEqual(
			expect.objectContaining({ provider: "vimeo", id: "123456", queryParams: expect.not.objectContaining({ h: expect.anything() }) })
		);
		expect(vimeo.parse("https://example.com/video/1")).toBeNull();
		expect(vimeo.parse("not-vimeo")).toBeNull();
	});

	it("builds embed URL with options", () => {
		expect(vimeo.getEmbedUrl("123")).toBe("https://player.vimeo.com/video/123");
		expect(
			vimeo.getEmbedUrl("123", { autoplay: true, enableApi: true, origin: "https://app.example", startTime: 15 })
		).toBe("https://player.vimeo.com/video/123?autoplay=1&api=1&origin=https%3A%2F%2Fapp.example#t=15s");
	});

	it("looks up metadata via oEmbed and handles failures", async () => {
		const ok = vi.fn().mockResolvedValue({
			ok: true,
			json: async () => ({
				title: "Video",
				description: "Desc",
				duration: 12,
				author_name: "Author",
				author_url: "https://author",
				thumbnail_url: "https://img",
				thumbnail_width: 640,
				thumbnail_height: 360,
				upload_date: "2026-01-01",
			}),
		});
		await expect(vimeo.lookupMeta("123", ok as any)).resolves.toEqual({
			title: "Video",
			description: "Desc",
			duration: 12,
			authorName: "Author",
			authorUrl: "https://author",
			thumbnailUrl: "https://img",
			thumbnailWidth: 640,
			thumbnailHeight: 360,
			uploadDate: "2026-01-01",
		});

		const bad = vi.fn().mockResolvedValue({ ok: false });
		await expect(vimeo.lookupMeta("123", bad as any)).resolves.toBeNull();
		const throws = vi.fn().mockRejectedValue(new Error("x"));
		await expect(vimeo.lookupMeta("123", throws as any)).resolves.toBeNull();
	});
});
