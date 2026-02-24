import { describe, expect, it, vi } from "vitest";
import { audioboom, getAudioboomPlaylistUrl, lookupPlaylistMeta } from "../../src/embed/providers/audioboom";

describe("embed/providers/audioboom", () => {
	it("parses raw ids and supported Audioboom URL formats", () => {
		expect(audioboom.parse("1234567")).toEqual({
			provider: "audioboom",
			id: "1234567",
			embedType: "single",
		});

		expect(audioboom.parse("https://audioboom.com/posts/12345-some-title")).toEqual(
			expect.objectContaining({ provider: "audioboom", id: "12345", embedType: "single" })
		);

		expect(audioboom.parse("https://embeds.audioboom.com/posts/456/embed/v4")).toEqual(
			expect.objectContaining({ provider: "audioboom", id: "456", embedType: "single" })
		);
		expect(audioboom.parse("https://audioboom.com/boos/abc-slug")).toEqual(
			expect.objectContaining({ provider: "audioboom", id: "abc-slug", embedType: "single" })
		);
		expect(audioboom.parse("https://embeds.audioboom.com/posts/slug-only/embed/v4")).toEqual(
			expect.objectContaining({ provider: "audioboom", id: "slug-only", embedType: "single" })
		);
		expect(audioboom.parse("https://embeds.audioboom.com/x/posts/777-episode/embed/v4")).toEqual(
			expect.objectContaining({ provider: "audioboom", id: "777", embedType: "single" })
		);
		expect(audioboom.parse("https://embeds.audioboom.com/not-posts/slug/embed/v4")).toBeNull();
		expect(audioboom.parse("https://audioboom.com/posts/")).toBeNull();

		expect(
			audioboom.parse("https://audioboom.com/publishing/playlist/v4?data_for_content_type=pl-1&channel_id=c1")
		).toEqual(
			expect.objectContaining({
				provider: "audioboom",
				id: "pl-1",
				embedType: "playlist",
				queryParams: expect.objectContaining({ channel_id: "c1" }),
			})
		);
		expect(
			audioboom.parse("https://audioboom.com/publishing/playlist/v4?playlist_id=pl-2")
		).toEqual(
			expect.objectContaining({
				provider: "audioboom",
				id: "pl-2",
				embedType: "playlist",
			})
		);
		expect(
			audioboom.parse("https://audioboom.com/publishing/playlist/v4?channel_id=channel-only")
		).toEqual(
			expect.objectContaining({
				provider: "audioboom",
				id: "channel-only",
				embedType: "playlist",
			})
		);
		expect(audioboom.parse("https://audioboom.com/publishing/playlist/v4")).toBeNull();
		expect(audioboom.parse("https://embeds.audioboom.com/x/posts/slug-only/embed/v4")).toEqual(
			expect.objectContaining({ provider: "audioboom", id: "slug-only", embedType: "single" })
		);

		expect(audioboom.parse("https://example.com/video/1")).toBeNull();
		expect(audioboom.parse("https://audioboom.com/unknown")).toBeNull();
	});

	it("builds embed and playlist URLs with options", () => {
		expect(audioboom.getEmbedUrl("123")).toBe("https://embeds.audioboom.com/posts/123/embed/v4");
		expect(audioboom.getEmbedUrl("123", { autoplay: true })).toBe(
			"https://embeds.audioboom.com/posts/123/embed/v4?autoplay=1"
		);

		expect(
			getAudioboomPlaylistUrl("pl-1", { autoplay: true, channelId: "c1", contentType: "episodes" })
		).toBe(
			"https://embeds.audioboom.com/publishing/playlist/v4?data_for_content_type=pl-1&channel_id=c1&content_type=episodes&autoplay=1"
		);
		expect(getAudioboomPlaylistUrl("pl-1")).toBe(
			"https://embeds.audioboom.com/publishing/playlist/v4?data_for_content_type=pl-1"
		);
	});

	it("looks up single-post metadata via oEmbed and handles failures", async () => {
		const okFetch = vi.fn().mockResolvedValue({
			ok: true,
			json: async () => ({
				title: "Episode",
				description: "Desc",
				duration: 42,
				author_name: "Author",
				author_url: "https://author",
				thumbnail_url: "https://img",
				thumbnail_width: 640,
				thumbnail_height: 360,
			}),
		});

		await expect(audioboom.lookupMeta("123", okFetch as any)).resolves.toEqual({
			title: "Episode",
			description: "Desc",
			duration: 42,
			authorName: "Author",
			authorUrl: "https://author",
			thumbnailUrl: "https://img",
			thumbnailWidth: 640,
			thumbnailHeight: 360,
		});
		expect(okFetch).toHaveBeenCalledWith(
			expect.stringContaining("https://audioboom.com/publishing/oembed.json?url=")
		);

		const badFetch = vi.fn().mockResolvedValue({ ok: false });
		await expect(audioboom.lookupMeta("123", badFetch as any)).resolves.toBeNull();

		const throwFetch = vi.fn().mockRejectedValue(new Error("network"));
		await expect(audioboom.lookupMeta("123", throwFetch as any)).resolves.toBeNull();
	});

	it("uses playlist lookup when parsed embed type is playlist", async () => {
		const fetchFn = vi.fn().mockResolvedValue({
			ok: true,
			json: async () => ({
				body: {
					playlist: {
						title: "Playlist",
						description: "Playlist Desc",
						clips: [{ duration: 10 }, { duration: 15 }],
						image_urls: { original: "https://img" },
						channel: { title: "Channel" },
					},
				},
			}),
		});

		await expect(
			audioboom.lookupMeta("pl-1", fetchFn as any, { provider: "audioboom", id: "pl-1", embedType: "playlist" } as any)
		).resolves.toEqual({
			title: "Playlist",
			description: "Playlist Desc",
			duration: 25,
			thumbnailUrl: "https://img",
			authorName: "Channel",
		});
	});

	it("playlist lookup handles missing data and fallback image url", async () => {
		const noPlaylist = vi.fn().mockResolvedValue({ ok: true, json: async () => ({ body: {} }) });
		await expect(lookupPlaylistMeta("pl-2", noPlaylist as any)).resolves.toBeNull();

		const fallbackImage = vi.fn().mockResolvedValue({
			ok: true,
			json: async () => ({
				body: {
					playlist: {
						title: "P",
						description: "D",
						clips: [{ duration: 0 }, {}],
						image_url: "https://fallback",
						channel: { title: "C" },
					},
				},
			}),
		});
		await expect(lookupPlaylistMeta("pl-3", fallbackImage as any)).resolves.toEqual({
			title: "P",
			description: "D",
			duration: undefined,
			thumbnailUrl: "https://fallback",
			authorName: "C",
		});

		const noClips = vi.fn().mockResolvedValue({
			ok: true,
			json: async () => ({
				body: {
					playlist: {
						title: "No Clips",
						description: "No clips field",
						image_url: "https://fallback-2",
					},
				},
			}),
		});
		await expect(lookupPlaylistMeta("pl-3b", noClips as any)).resolves.toEqual({
			title: "No Clips",
			description: "No clips field",
			duration: undefined,
			thumbnailUrl: "https://fallback-2",
			authorName: undefined,
		});

		const bad = vi.fn().mockResolvedValue({ ok: false });
		await expect(lookupPlaylistMeta("pl-4", bad as any)).resolves.toBeNull();
		const throws = vi.fn().mockRejectedValue(new Error("x"));
		await expect(lookupPlaylistMeta("pl-5", throws as any)).resolves.toBeNull();
	});
});
