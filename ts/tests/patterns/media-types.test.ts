import { describe, expect, it, vi } from "vitest";

const iconRefs = vi.hoisted(() => ({
	image: Symbol("image"),
	video: Symbol("video"),
	music: Symbol("music"),
	fileText: Symbol("fileText"),
	file: Symbol("file"),
}));

vi.mock("lucide-svelte/icons/image", () => ({ default: iconRefs.image }));
vi.mock("lucide-svelte/icons/video", () => ({ default: iconRefs.video }));
vi.mock("lucide-svelte/icons/music", () => ({ default: iconRefs.music }));
vi.mock("lucide-svelte/icons/file-text", () => ({ default: iconRefs.fileText }));
vi.mock("lucide-svelte/icons/file", () => ({ default: iconRefs.file }));

import {
	MediaKind,
	MediaVisibility,
	MediaVersionState,
	detectMediaKindFromMimeType,
	getMediaKindIcon,
	getMediaKindLabel,
} from "../../src/patterns/media-types";

describe("patterns/media-types (barrel)", () => {
	it("re-exports enums and helpers", () => {
		expect(MediaKind.Pdf).toBe("pdf");
		expect(MediaVisibility.Public).toBe("public");
		expect(MediaVersionState.Ready).toBe("ready");
		expect(getMediaKindLabel(MediaKind.Image)).toBe("Image");
		expect(detectMediaKindFromMimeType("application/pdf")).toBe(MediaKind.Pdf);
		expect(getMediaKindIcon(MediaKind.Image)).toBe(iconRefs.image);
	});
});
