import { describe, expect, it, vi } from "vitest";

const iconRefs = vi.hoisted(() => ({
	image: Symbol("image-icon"),
	video: Symbol("video-icon"),
	music: Symbol("music-icon"),
	fileText: Symbol("file-text-icon"),
	file: Symbol("file-icon"),
}));

vi.mock("lucide-svelte/icons/image", () => ({ default: iconRefs.image }));
vi.mock("lucide-svelte/icons/video", () => ({ default: iconRefs.video }));
vi.mock("lucide-svelte/icons/music", () => ({ default: iconRefs.music }));
vi.mock("lucide-svelte/icons/file-text", () => ({ default: iconRefs.fileText }));
vi.mock("lucide-svelte/icons/file", () => ({ default: iconRefs.file }));

import { MediaKind } from "../../src/patterns/media-types/enums";
import { getMediaKindIcon } from "../../src/patterns/media-types/icons";

describe("patterns/media-types/icons", () => {
	it("maps media kind to expected icon component", () => {
		expect(getMediaKindIcon(MediaKind.Image)).toBe(iconRefs.image);
		expect(getMediaKindIcon(MediaKind.Video)).toBe(iconRefs.video);
		expect(getMediaKindIcon(MediaKind.Audio)).toBe(iconRefs.music);
		expect(getMediaKindIcon(MediaKind.Pdf)).toBe(iconRefs.fileText);
		expect(getMediaKindIcon(MediaKind.Document)).toBe(iconRefs.fileText);
		expect(getMediaKindIcon(MediaKind.Other)).toBe(iconRefs.file);
	});

	it("falls back to generic file icon for unknown kinds", () => {
		expect(getMediaKindIcon("unknown" as unknown as typeof MediaKind[keyof typeof MediaKind])).toBe(iconRefs.file);
	});
});
