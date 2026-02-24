import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { compressImage } from "../../src/components/file-upload/compression";

const state = {
	mode: "load" as "load" | "error",
	width: 4000,
	height: 2000,
	blob: new Blob([new Uint8Array(10)], { type: "image/jpeg" }),
	ctx: { drawImage: vi.fn() } as { drawImage: ReturnType<typeof vi.fn> } | null,
	lastToBlobType: "" as string | undefined,
	lastToBlobQuality: 0 as number | undefined,
};

class MockImage {
	onload: (() => void) | null = null;
	onerror: (() => void) | null = null;
	width = state.width;
	height = state.height;
	private _src = "";

	get src() {
		return this._src;
	}

	set src(value: string) {
		this._src = value;
		queueMicrotask(() => {
			this.width = state.width;
			this.height = state.height;
			if (state.mode === "error") {
				this.onerror?.();
				return;
			}
			this.onload?.();
		});
	}
}

describe("components/file-upload/compression", () => {
	const originalImage = (globalThis as any).Image;
	const originalDocument = (globalThis as any).document;
	const originalCreate = URL.createObjectURL;
	const originalRevoke = URL.revokeObjectURL;

	beforeEach(() => {
		state.mode = "load";
		state.width = 4000;
		state.height = 2000;
		state.blob = new Blob([new Uint8Array(10)], { type: "image/jpeg" });
		state.ctx = { drawImage: vi.fn() };
		state.lastToBlobType = undefined;
		state.lastToBlobQuality = undefined;

		(globalThis as any).Image = MockImage;
		(globalThis as any).document = {
			createElement: vi.fn(() => {
				const canvas = {
					width: 0,
					height: 0,
					getContext: vi.fn(() => state.ctx),
					toBlob: vi.fn((cb: (blob: Blob | null) => void, type?: string, quality?: number) => {
						state.lastToBlobType = type;
						state.lastToBlobQuality = quality;
						cb(state.blob);
					}),
				};
				return canvas;
			}),
		};
		URL.createObjectURL = vi.fn(() => "blob:mock-url");
		URL.revokeObjectURL = vi.fn();
	});

	afterEach(() => {
		(globalThis as any).Image = originalImage;
		(globalThis as any).document = originalDocument;
		URL.createObjectURL = originalCreate;
		URL.revokeObjectURL = originalRevoke;
		vi.restoreAllMocks();
	});

	it("returns original file for non-image and excluded image formats", async () => {
		const textFile = new File(["abc"], "a.txt", { type: "text/plain" });
		const svgFile = new File(["<svg/>"], "a.svg", { type: "image/svg+xml" });
		const gifFile = new File(["gif"], "a.gif", { type: "image/gif" });

		await expect(compressImage(textFile)).resolves.toBe(textFile);
		await expect(compressImage(svgFile)).resolves.toBe(svgFile);
		await expect(compressImage(gifFile)).resolves.toBe(gifFile);
	});

	it("compresses oversized images and returns smaller output file", async () => {
		const file = new File([new Uint8Array(100)], "photo.jpg", { type: "image/jpeg" });
		state.width = 4000;
		state.height = 2000;
		state.blob = new Blob([new Uint8Array(10)], { type: "image/jpeg" });

		const result = await compressImage(file, { maxWidth: 1000, maxHeight: 1000, quality: 0.5 });

		expect(result).not.toBe(file);
		expect(result.type).toBe("image/jpeg");
		expect(state.ctx?.drawImage).toHaveBeenCalledWith(expect.any(MockImage), 0, 0, 1000, 500);
		expect(state.lastToBlobType).toBe("image/jpeg");
		expect(state.lastToBlobQuality).toBe(0.5);
		expect(URL.createObjectURL).toHaveBeenCalledWith(file);
		expect(URL.revokeObjectURL).toHaveBeenCalledWith("blob:mock-url");
	});

	it("keeps original dimensions when image is within limits", async () => {
		const file = new File([new Uint8Array(100)], "photo.jpg", { type: "image/jpeg" });
		state.width = 800;
		state.height = 600;
		state.blob = new Blob([new Uint8Array(10)], { type: "image/jpeg" });

		await compressImage(file, { maxWidth: 1000, maxHeight: 1000, quality: 0.7 });
		expect(state.ctx?.drawImage).toHaveBeenCalledWith(expect.any(MockImage), 0, 0, 800, 600);
	});

	it("uses PNG output by default for PNG input", async () => {
		const file = new File([new Uint8Array(100)], "photo.png", { type: "image/png" });
		await compressImage(file);
		expect(state.lastToBlobType).toBe("image/png");
	});

	it("returns original file when canvas context is unavailable or blob is not smaller", async () => {
		const file = new File([new Uint8Array(50)], "photo.jpg", { type: "image/jpeg" });

		state.ctx = null;
		await expect(compressImage(file)).resolves.toBe(file);

		state.ctx = { drawImage: vi.fn() };
		state.blob = new Blob([new Uint8Array(100)], { type: "image/jpeg" });
		await expect(compressImage(file)).resolves.toBe(file);

		state.blob = null as unknown as Blob;
		await expect(compressImage(file)).resolves.toBe(file);
	});

	it("returns original file on image load error", async () => {
		const file = new File([new Uint8Array(50)], "photo.jpg", { type: "image/jpeg" });
		state.mode = "error";
		await expect(compressImage(file)).resolves.toBe(file);
		expect(URL.revokeObjectURL).toHaveBeenCalledWith("blob:mock-url");
	});
});
