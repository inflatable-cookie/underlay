import { describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
	computeFileHash: vi.fn(),
	uploadNewMedia: vi.fn(),
}));

vi.mock("../../src/patterns/index.js", () => ({
	computeFileHash: mocks.computeFileHash,
}));

vi.mock("../../src/components/media-picker/upload", () => ({
	uploadNewMedia: mocks.uploadNewMedia,
}));

import { runUploadFlow, uploadMediaWithKnownHash } from "../../src/components/media-picker/upload-flow";

describe("components/media-picker/upload-flow", () => {
	it("returns duplicate result when duplicate media exists", async () => {
		const file = { name: "a.png" } as File;
		mocks.computeFileHash.mockResolvedValue("hash-1");

		await expect(
			runUploadFlow({
				file,
				maxFileSize: 123,
				checkDuplicate: async () => ({ exists: true, media: { id: "m1" } as any }),
				createMedia: vi.fn() as any,
				initiateUpload: vi.fn() as any,
				finaliseUpload: vi.fn() as any,
				onStep: vi.fn(),
				onProgress: vi.fn(),
			})
		).resolves.toEqual({
			kind: "duplicate",
			fileHash: "hash-1",
			duplicateMedia: { id: "m1" },
		});
	});

	it("uploads media with known hash and returns uploaded result", async () => {
		const file = { name: "a.png" } as File;
		const createdMedia = { id: "created-1" } as any;
		mocks.computeFileHash.mockResolvedValue("hash-2");
		mocks.uploadNewMedia.mockResolvedValue(createdMedia);

		const onStep = vi.fn();
		const onProgress = vi.fn();
		const createMedia = vi.fn();
		const initiateUpload = vi.fn();
		const finaliseUpload = vi.fn();

		await expect(
			runUploadFlow({
				file,
				maxFileSize: 999,
				checkDuplicate: async () => ({ exists: false, media: null }),
				createMedia: createMedia as any,
				initiateUpload: initiateUpload as any,
				finaliseUpload: finaliseUpload as any,
				onStep,
				onProgress,
			})
		).resolves.toEqual({ kind: "uploaded", fileHash: "hash-2", createdMedia });

		expect(onStep).toHaveBeenCalledWith("checking");
		expect(mocks.uploadNewMedia).toHaveBeenCalledWith({
			file,
			fileHash: "hash-2",
			maxFileSize: 999,
			createMedia,
			initiateUpload,
			finaliseUpload,
			onStage: onStep,
			onProgress,
		});

		await expect(
			uploadMediaWithKnownHash({
				file,
				fileHash: "hash-3",
				maxFileSize: 42,
				createMedia: createMedia as any,
				initiateUpload: initiateUpload as any,
				finaliseUpload: finaliseUpload as any,
				onStep,
				onProgress,
			})
		).resolves.toBe(createdMedia);
	});
});
