import { describe, expect, it } from "vitest";
import {
	detectMediaKindFromMimeType,
	getMediaDisplayName,
	getMediaKindAccent,
	getMediaKindLabel,
	getMediaVersionStateAccent,
	getMediaVersionStateLabel,
	getMediaVisibilityAccent,
	getMediaVisibilityLabel,
	isMediaDeleted,
} from "../../src/patterns/media-types/labels";
import { MediaKind, MediaVersionState, MediaVisibility } from "../../src/patterns/media-types/enums";

describe("patterns/media-types/labels", () => {
	it("returns labels for known and unknown media kinds", () => {
		expect(getMediaKindLabel(MediaKind.Image)).toBe("Image");
		expect(getMediaKindLabel(MediaKind.Video)).toBe("Video");
		expect(getMediaKindLabel(MediaKind.Audio)).toBe("Audio");
		expect(getMediaKindLabel(MediaKind.Document)).toBe("Document");
		expect(getMediaKindLabel(MediaKind.Pdf)).toBe("PDF");
		expect(getMediaKindLabel(MediaKind.Other)).toBe("Other");
		expect(getMediaKindLabel("weird" as unknown as typeof MediaKind[keyof typeof MediaKind])).toBe("weird");
	});

	it("returns accents for known and unknown media kinds", () => {
		expect(getMediaKindAccent(MediaKind.Image)).toBe("#22c55e");
		expect(getMediaKindAccent(MediaKind.Video)).toBe("#f59e0b");
		expect(getMediaKindAccent(MediaKind.Audio)).toBe("#8b5cf6");
		expect(getMediaKindAccent(MediaKind.Document)).toBe("#3b82f6");
		expect(getMediaKindAccent(MediaKind.Pdf)).toBe("#ef4444");
		expect(getMediaKindAccent(MediaKind.Other)).toBe("#94a3b8");
		expect(getMediaKindAccent("unknown" as unknown as typeof MediaKind[keyof typeof MediaKind])).toBe("#94a3b8");
	});

	it("returns visibility labels and accents with fallback", () => {
		expect(getMediaVisibilityLabel(MediaVisibility.Public)).toBe("Public");
		expect(getMediaVisibilityLabel(MediaVisibility.Restricted)).toBe("Restricted");
		expect(getMediaVisibilityLabel("secret" as unknown as typeof MediaVisibility[keyof typeof MediaVisibility])).toBe("secret");

		expect(getMediaVisibilityAccent(MediaVisibility.Public)).toBe("#3b82f6");
		expect(getMediaVisibilityAccent(MediaVisibility.Restricted)).toBe("#f59e0b");
		expect(getMediaVisibilityAccent("secret" as unknown as typeof MediaVisibility[keyof typeof MediaVisibility])).toBe("#94a3b8");
	});

	it("returns version-state labels and accents with fallback", () => {
		expect(getMediaVersionStateLabel(MediaVersionState.Uploading)).toBe("Uploading");
		expect(getMediaVersionStateLabel(MediaVersionState.Ready)).toBe("Ready");
		expect(getMediaVersionStateLabel(MediaVersionState.Failed)).toBe("Failed");
		expect(getMediaVersionStateLabel(MediaVersionState.Purging)).toBe("Purging");
		expect(getMediaVersionStateLabel("unknown" as unknown as typeof MediaVersionState[keyof typeof MediaVersionState])).toBe("unknown");

		expect(getMediaVersionStateAccent(MediaVersionState.Ready)).toBe("#22c55e");
		expect(getMediaVersionStateAccent(MediaVersionState.Uploading)).toBe("#3b82f6");
		expect(getMediaVersionStateAccent(MediaVersionState.Failed)).toBe("#ef4444");
		expect(getMediaVersionStateAccent(MediaVersionState.Purging)).toBe("#f59e0b");
		expect(getMediaVersionStateAccent("unknown" as unknown as typeof MediaVersionState[keyof typeof MediaVersionState])).toBe("#94a3b8");
	});

	it("detects media kind from mime type", () => {
		expect(detectMediaKindFromMimeType("image/png")).toBe(MediaKind.Image);
		expect(detectMediaKindFromMimeType("video/mp4")).toBe(MediaKind.Video);
		expect(detectMediaKindFromMimeType("audio/mpeg")).toBe(MediaKind.Audio);
		expect(detectMediaKindFromMimeType("application/pdf")).toBe(MediaKind.Pdf);
		expect(detectMediaKindFromMimeType("application/vnd.openxmlformats-officedocument.wordprocessingml.document")).toBe(MediaKind.Document);
		expect(detectMediaKindFromMimeType("application/octet-stream")).toBe(MediaKind.Other);
	});

	it("derives deletion and display metadata", () => {
		expect(isMediaDeleted({ deletedAt: null } as any)).toBe(false);
		expect(isMediaDeleted({ deletedAt: "2026-01-01T00:00:00Z" } as any)).toBe(true);

		expect(getMediaDisplayName({ title: "Banner", originalFilename: "banner.png" } as any)).toBe("Banner");
		expect(getMediaDisplayName({ title: null, originalFilename: "file.pdf" } as any)).toBe("file.pdf");
		expect(getMediaDisplayName({ title: null, originalFilename: null } as any)).toBe("Untitled");
	});
});
