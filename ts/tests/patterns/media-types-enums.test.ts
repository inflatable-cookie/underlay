import { describe, expect, it } from "vitest";
import {
	MediaKind,
	MediaVersionState,
	MediaVisibility,
} from "../../src/patterns/media-types/enums";

describe("patterns/media-types/enums", () => {
	it("defines expected media kinds", () => {
		expect(MediaKind).toEqual({
			Image: "image",
			Video: "video",
			Audio: "audio",
			Document: "document",
			Pdf: "pdf",
			Other: "other",
		});
	});

	it("defines expected media visibility values", () => {
		expect(MediaVisibility).toEqual({
			Public: "public",
			Restricted: "restricted",
		});
	});

	it("defines expected media version states", () => {
		expect(MediaVersionState).toEqual({
			Uploading: "uploading",
			Ready: "ready",
			Failed: "failed",
			Purging: "purging",
		});
	});
});
