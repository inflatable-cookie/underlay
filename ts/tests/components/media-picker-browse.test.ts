import { describe, expect, it, vi } from "vitest";
import { loadMediaBrowsePage, mergeMediaBrowseItems } from "../../src/components/media-picker/browse";

describe("components/media-picker/browse", () => {
	it("loads browse pages with expected params and shape", async () => {
		const listMediaPaginated = vi.fn().mockResolvedValue({
			data: [{ id: "1" }],
			nextCursor: "cursor-2",
			hasMore: true,
		});

		await expect(loadMediaBrowsePage({ listMediaPaginated })).resolves.toEqual({
			items: [{ id: "1" }],
			nextCursor: "cursor-2",
			hasMore: true,
		});
		expect(listMediaPaginated).toHaveBeenCalledWith({ cursor: undefined, limit: 12 });
	});

	it("merges browse items only when cursor is present", () => {
		const existing = [{ id: "1" }] as any[];
		const next = [{ id: "2" }] as any[];
		expect(mergeMediaBrowseItems(existing, next)).toEqual(next);
		expect(mergeMediaBrowseItems(existing, next, "cursor-2")).toEqual([{ id: "1" }, { id: "2" }]);
	});
});
