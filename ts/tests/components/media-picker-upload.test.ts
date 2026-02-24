import { describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
	uploadToBlob: vi.fn(),
}));

vi.mock("../../src/patterns/index.js", () => ({
	uploadToBlob: mocks.uploadToBlob,
	MediaKind: { Image: "image", Pdf: "pdf" },
	MediaVisibility: { Public: "public" },
}));

import { uploadNewMedia } from "../../src/components/media-picker/upload";

describe("components/media-picker/upload", () => {
	it("creates, uploads, finalises media and maps summary", async () => {
		const file = { name: "photo.png", type: "image/png", size: 100 } as File;
		const createMedia = vi.fn().mockResolvedValue({ id: "m1" });
		const initiateUpload = vi.fn().mockResolvedValue({
			versionId: "v1",
			uploadPlan: {
				uploadUrl: "https://upload",
				method: "PUT",
				headers: { a: "b" },
				expiresAt: "2026-01-01T00:00:00Z",
				maxBytes: null,
				allowedContentTypes: null,
			},
		});
		const finaliseUpload = vi.fn().mockResolvedValue({
			media: {
				id: "m1",
				kind: "image",
				visibility: "public",
				title: "photo",
				originalFilename: "photo.png",
				currentVersionId: "v1",
				createdAt: "2026-01-01",
				updatedAt: "2026-01-01",
				currentVersion: { byteSize: 100, mimeType: "image/png" },
			},
		});
		mocks.uploadToBlob.mockImplementation(async (_plan, _file, opts) => {
			opts.onProgress({ percent: 75 });
		});

		const onStage = vi.fn();
		const onProgress = vi.fn();
		await expect(
			uploadNewMedia({
				file,
				fileHash: "hash",
				maxFileSize: 500,
				createMedia,
				initiateUpload,
				finaliseUpload,
				onStage,
				onProgress,
			})
		).resolves.toEqual({
			id: "m1",
			kind: "image",
			visibility: "public",
			title: "photo",
			originalFilename: "photo.png",
			currentVersionId: "v1",
			createdAt: "2026-01-01",
			updatedAt: "2026-01-01",
			deletedAt: null,
			byteSize: 100,
			mimeType: "image/png",
			thumbnailUrl: null,
		});

		expect(createMedia).toHaveBeenCalledWith(expect.objectContaining({ kind: "image", title: "photo" }));
		expect(onStage).toHaveBeenNthCalledWith(1, "uploading");
		expect(onStage).toHaveBeenNthCalledWith(2, "finalising");
		expect(onProgress).toHaveBeenCalledWith(75);
	});

	it("maps pdf files to pdf media kind", async () => {
		const file = { name: "doc.pdf", type: "application/pdf", size: 100 } as File;
		const createMedia = vi.fn().mockResolvedValue({ id: "m2" });
		const initiateUpload = vi.fn().mockResolvedValue({
			versionId: "v2",
			uploadPlan: { uploadUrl: "u", method: "PUT", headers: {}, expiresAt: "x", maxBytes: 1, allowedContentTypes: [] },
		});
		const finaliseUpload = vi.fn().mockResolvedValue({ media: { id: "m2", kind: "pdf", visibility: "public", title: "doc", originalFilename: "doc.pdf", currentVersionId: "v2", createdAt: "x", updatedAt: "x", currentVersion: null } });
		mocks.uploadToBlob.mockResolvedValue(undefined);

		await uploadNewMedia({ file, fileHash: "h", maxFileSize: 1, createMedia, initiateUpload, finaliseUpload, onProgress: vi.fn() });
		expect(createMedia).toHaveBeenCalledWith(expect.objectContaining({ kind: "pdf" }));
	});

	it("falls back unknown mime types to image media kind", async () => {
		const file = { name: "data.bin", type: "application/octet-stream", size: 100 } as File;
		const createMedia = vi.fn().mockResolvedValue({ id: "m3" });
		const initiateUpload = vi.fn().mockResolvedValue({
			versionId: "v3",
			uploadPlan: { uploadUrl: "u", method: "PUT", headers: {}, expiresAt: "x", maxBytes: 1, allowedContentTypes: [] },
		});
		const finaliseUpload = vi.fn().mockResolvedValue({ media: { id: "m3", kind: "image", visibility: "public", title: "data", originalFilename: "data.bin", currentVersionId: "v3", createdAt: "x", updatedAt: "x", currentVersion: null } });
		mocks.uploadToBlob.mockResolvedValue(undefined);

		await uploadNewMedia({ file, fileHash: "h", maxFileSize: 1, createMedia, initiateUpload, finaliseUpload, onProgress: vi.fn() });
		expect(createMedia).toHaveBeenCalledWith(expect.objectContaining({ kind: "image" }));
	});
});
