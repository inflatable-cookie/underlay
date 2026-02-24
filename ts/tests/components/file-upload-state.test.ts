import { describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
	compressImage: vi.fn(),
	generateFileUploadId: vi.fn(),
	validateUploadFile: vi.fn(),
	createObjectURL: vi.fn(),
	revokeObjectURL: vi.fn(),
}));

vi.mock("../../src/components/file-upload/compression", () => ({
	compressImage: mocks.compressImage,
}));

vi.mock("../../src/components/file-upload/helpers", () => ({
	generateFileUploadId: mocks.generateFileUploadId,
	validateUploadFile: mocks.validateUploadFile,
}));

import {
	processUploadFiles,
	removeUploadItem,
	retryUploadItem,
	revokePreviewUrls,
	setUploadError,
	updateUploadProgress,
} from "../../src/components/file-upload/state";

describe("components/file-upload/state", () => {
	it("handles empty file input safely", async () => {
		await expect(
			processUploadFiles({
				fileList: null,
				currentFiles: [],
				accept: "*",
				maxSize: 100,
				multiple: true,
				maxFiles: 3,
				showPreview: true,
				compress: false,
				compressionOptions: {} as any,
			})
		).resolves.toEqual({ nextFiles: [], filesToUpload: [], replacedPreviewUrls: [] });
	});

	it("processes valid files, previews, compression, and replacement", async () => {
		(globalThis as any).URL = {
			createObjectURL: mocks.createObjectURL,
			revokeObjectURL: mocks.revokeObjectURL,
		};
		mocks.generateFileUploadId.mockReturnValue("id-1");
		mocks.validateUploadFile.mockReturnValue(null);

		const file = { name: "a.png", type: "image/png", size: 50 } as File;
		const compressed = { name: "a.png", type: "image/png", size: 20 } as File;
		mocks.compressImage.mockResolvedValue(compressed);
		mocks.createObjectURL.mockReturnValue("blob:new");

		const fileList = [file] as unknown as FileList;
		const current = [{ id: "old", previewUrl: "blob:old" }] as any[];

		const result = await processUploadFiles({
			fileList,
			currentFiles: current,
			accept: "image/*",
			maxSize: 100,
			multiple: false,
			maxFiles: 1,
			showPreview: true,
			compress: true,
			compressionOptions: {} as any,
		});

		expect(result.filesToUpload).toEqual([compressed]);
		expect(result.replacedPreviewUrls).toEqual(["blob:old"]);
		expect(result.nextFiles[0]).toEqual(expect.objectContaining({ id: "id-1", file: compressed, originalFile: file, previewUrl: "blob:new" }));
	});

	it("skips invalid files and emits validation errors", async () => {
		mocks.validateUploadFile.mockReturnValue("nope");
		const onValidationError = vi.fn();
		const file = { name: "bad.txt", type: "text/plain", size: 1 } as File;
		const fileList = [file] as unknown as FileList;

		const result = await processUploadFiles({
			fileList,
			currentFiles: [],
			accept: "image/*",
			maxSize: 100,
			multiple: true,
			maxFiles: 3,
			showPreview: false,
			compress: false,
			compressionOptions: {} as any,
			onValidationError,
		});

		expect(result).toEqual({ nextFiles: [], filesToUpload: [], replacedPreviewUrls: [] });
		expect(onValidationError).toHaveBeenCalledWith({ file, message: "nope" });
	});

	it("updates and manages upload items", () => {
		(globalThis as any).URL = {
			revokeObjectURL: mocks.revokeObjectURL,
		};
		const item = { id: "i1", file: { name: "a" } as any, previewUrl: "blob:i1", progress: 0, status: "pending" } as any;
		const current = [item, { id: "i2", file: { name: "b" } as any, progress: 0, status: "pending" } as any];

		revokePreviewUrls(current);
		expect(mocks.revokeObjectURL).toHaveBeenCalledWith("blob:i1");

		expect(removeUploadItem(current, item)).toHaveLength(1);
		expect(retryUploadItem(current, item)).toEqual({
			nextFiles: [
				expect.objectContaining({ id: "i1", status: "pending", progress: 0, error: undefined }),
				expect.objectContaining({ id: "i2" }),
			],
			retryFile: item.file,
		});

		expect(updateUploadProgress(current, "i2", 50)[1]).toEqual(expect.objectContaining({ status: "uploading", progress: 50 }));
		expect(updateUploadProgress(current, "i2", 100)[1]).toEqual(expect.objectContaining({ status: "complete", progress: 100 }));
		expect(setUploadError(current, "i2", "failed")[1]).toEqual(expect.objectContaining({ status: "error", error: "failed" }));
	});
});
