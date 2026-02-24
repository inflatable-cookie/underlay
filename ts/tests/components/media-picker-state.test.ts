import { describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
	validateFile: vi.fn(),
}));

vi.mock("../../src/patterns/index.js", () => ({
	validateFile: mocks.validateFile,
}));

import {
	createClearedUploadState,
	createResetBrowseState,
	validateMediaPickerFile,
} from "../../src/components/media-picker/state";

describe("components/media-picker/state", () => {
	it("creates cleared upload and browse states", () => {
		expect(createClearedUploadState()).toEqual({
			selectedFile: null,
			fileError: null,
			uploadStep: "select",
			uploadProgress: 0,
			uploadError: null,
			duplicateMedia: null,
			fileHash: null,
			createdMedia: null,
		});

		expect(createResetBrowseState()).toEqual({
			browseItems: [],
			browseNextCursor: null,
			browseHasMore: false,
		});
	});

	it("validates media picker files and returns typed result", () => {
		const file = { name: "x.png", size: 100, type: "image/png" } as File;
		mocks.validateFile.mockImplementationOnce(() => {});
		expect(validateMediaPickerFile(file, 1000)).toEqual({ selectedFile: file, fileError: null });

		mocks.validateFile.mockImplementationOnce(() => {
			throw new Error("too big");
		});
		expect(validateMediaPickerFile(file, 50)).toEqual({ selectedFile: null, fileError: "too big" });

		mocks.validateFile.mockImplementationOnce(() => {
			throw "bad";
		});
		expect(validateMediaPickerFile(file, 50)).toEqual({ selectedFile: null, fileError: "Invalid file" });
	});
});
