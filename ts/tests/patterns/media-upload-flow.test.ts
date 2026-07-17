import { describe, expect, it, vi } from "vitest";

type BlobUploadModule = typeof import("../../src/patterns/blob-upload.js");

async function loadMediaUploadModule(mocks?: {
	validateImpl?: (file: File, max: number) => { valid: boolean; error?: string };
	hashImpl?: (file: File) => Promise<string>;
	uploadImpl?: BlobUploadModule["uploadToBlob"];
}) {
	vi.resetModules();
	(globalThis as any).$state = <T>(initial: T) => initial;
	(globalThis as any).$derived = <T>(value: T) => value;

	vi.doMock("../../src/patterns/blob-upload.js", () => ({
		validateFile: vi.fn(mocks?.validateImpl ?? (() => ({ valid: true }))),
		computeFileHash: vi.fn(mocks?.hashImpl ?? (async () => "hash-1")),
		uploadToBlob: vi.fn(
			mocks?.uploadImpl ??
				(async (_plan, _file, callbacks) => {
					callbacks?.onProgress?.({ loaded: 1, total: 1, percent: 100 });
				})
		),
	}));
	vi.doMock("../../src/patterns/media-types.js", () => ({
		MediaKind: { Image: "Image", Pdf: "Pdf" },
		MediaVisibility: { Public: "Public" },
	}));

	const mod = await import("../../src/patterns/media-upload-flow.svelte");
	const blob = await import("../../src/patterns/blob-upload.js");
	return { mod, blob };
}

function fakeFile(overrides?: Partial<File>): File {
	return {
		name: "photo.jpg",
		type: "image/jpeg",
		size: 1234,
		...overrides,
	} as File;
}

describe("patterns/media-upload-flow.svelte.ts", () => {
	it("handles duplicate checks, useDuplicate, and reset/clear", async () => {
		const { mod, blob } = await loadMediaUploadModule();
		const checkDuplicate = vi.fn(async () => ({
			exists: true,
			media: { id: "m-1", title: "Existing" },
		}));
		const onError = vi.fn();

		const flow = mod.createMediaUploadFlow({
			checkDuplicate,
			createMedia: vi.fn() as any,
			initiateUpload: vi.fn() as any,
			finaliseUpload: vi.fn() as any,
			onError,
		});

		const file = fakeFile();
		flow.setFile(file);
		expect((blob.validateFile as any).mock.calls.length).toBe(1);
		expect(flow.file).toBe(file);

		await flow.startUpload();
		expect(flow.step).toBe("duplicate");
		expect(flow.duplicateMedia?.id).toBe("m-1");
		expect(checkDuplicate).toHaveBeenCalledWith("hash-1");

		flow.useDuplicate();
		expect(flow.step).toBe("select");

		flow.clearFile();
		expect(flow.file).toBeNull();
		expect(flow.fileHash).toBeNull();
		expect(flow.progress).toBe(0);
		expect(flow.error).toBeNull();
		expect(flow.duplicateMedia).toBeNull();

		flow.reset();
		expect(flow.createdMedia).toBeNull();

		expect(onError).not.toHaveBeenCalled();
	});

	it("completes new-upload and replace-upload flows and handles errors", async () => {
		const { mod } = await loadMediaUploadModule();
		const onComplete = vi.fn();
		const onError = vi.fn();
		const createMedia = vi.fn(async () => ({ id: "media-1", title: "Created" }));
		const initiateUpload = vi.fn(async () => ({
			versionId: "v-1",
			uploadPlan: {
				uploadUrl: "https://upload",
				method: "PUT",
				headers: { "x-test": "1" },
				expiresAt: "2099-01-01T00:00:00Z",
			},
		}));
		const finaliseUpload = vi.fn(async () => ({
			media: { id: "media-1", title: "Final" },
		}));

		const flow = mod.createMediaUploadFlow({
			checkDuplicate: vi.fn(async () => ({ exists: false })),
			createMedia: createMedia as any,
			initiateUpload: initiateUpload as any,
			finaliseUpload: finaliseUpload as any,
			onComplete,
			onError,
		});

		flow.setFile(fakeFile({ name: "invoice.pdf", type: "application/pdf", size: 4567 }));
		await flow.startUpload({ title: "Doc" });
		expect(flow.step).toBe("complete");
		expect(flow.progress).toBe(100);
		expect(flow.createdMedia?.id).toBe("media-1");
		expect(createMedia).toHaveBeenCalled();
		expect(initiateUpload).toHaveBeenCalledWith("media-1", {
			contentType: "application/pdf",
			contentLength: 4567,
			sha256: "hash-1",
		});
		expect(finaliseUpload).toHaveBeenCalledWith("media-1", "v-1", {
			sha256: "hash-1",
			contentType: "application/pdf",
		});
		expect(onComplete).toHaveBeenCalledTimes(1);
		expect(onError).not.toHaveBeenCalled();

		const replaceFlow = mod.createMediaUploadFlow({
			checkDuplicate: vi.fn(async () => ({ exists: false })),
			createMedia: vi.fn() as any,
			initiateUpload: initiateUpload as any,
			finaliseUpload: finaliseUpload as any,
			existingMediaId: "existing-media",
		});
		replaceFlow.setFile(fakeFile());
		await replaceFlow.startUpload();
		expect(replaceFlow.step).toBe("complete");
		expect(replaceFlow.createdMedia?.id).toBe("media-1");

		const existingHashFlow = mod.createMediaUploadFlow({
			checkDuplicate: vi.fn(async () => ({ exists: false })),
			createMedia: vi.fn() as any,
			initiateUpload: vi.fn() as any,
			finaliseUpload: vi.fn() as any,
			existingVersionHashes: ["hash-1"],
		});
		existingHashFlow.setFile(fakeFile());
		await existingHashFlow.startUpload();
		expect(existingHashFlow.step).toBe("error");
		expect(existingHashFlow.error).toMatch(/already been uploaded/);

		const failingFlow = mod.createMediaUploadFlow({
			checkDuplicate: vi.fn(async () => ({ exists: false })),
			createMedia: vi.fn(async () => {
				throw new Error("create failed");
			}) as any,
			initiateUpload: vi.fn() as any,
			finaliseUpload: vi.fn() as any,
			onError,
		});
		const errorSpy = vi.spyOn(console, "error").mockImplementation(() => undefined);
		failingFlow.setFile(fakeFile());
		await failingFlow.startUpload();
		expect(failingFlow.step).toBe("error");
		expect(failingFlow.error).toBe("create failed");
		expect(onError).toHaveBeenCalled();
		errorSpy.mockRestore();
	});

	it("handles validation/hash errors and no-op guards", async () => {
		const { mod } = await loadMediaUploadModule({
			validateImpl: () => ({ valid: false, error: "invalid file" }),
		});

		const onError = vi.fn();
		const errorSpy = vi.spyOn(console, "error").mockImplementation(() => undefined);
		const flow = mod.createMediaUploadFlow({
			checkDuplicate: vi.fn(async () => ({ exists: false })),
			createMedia: vi.fn() as any,
			initiateUpload: vi.fn() as any,
			finaliseUpload: vi.fn() as any,
			onError,
		});

		flow.setFile(fakeFile());
		expect(flow.file).toBeNull();
		expect(flow.fileError).toBe("invalid file");
		await flow.startUpload();
		expect(flow.step).toBe("select");

		flow.proceedWithUpload();
		expect(flow.step).toBe("select");

		errorSpy.mockRestore();

		const { mod: hashMod } = await loadMediaUploadModule({
			hashImpl: async () => {
				throw new Error("hash failed");
			},
		});
		const hashErrorSpy = vi.spyOn(console, "error").mockImplementation(() => undefined);
		const hashFlow = hashMod.createMediaUploadFlow({
			checkDuplicate: vi.fn(async () => ({ exists: false })),
			createMedia: vi.fn() as any,
			initiateUpload: vi.fn() as any,
			finaliseUpload: vi.fn() as any,
			onError,
		});
		hashFlow.setFile(fakeFile({ name: "ok.jpg", type: "image/jpeg" }));
		await hashFlow.startUpload();
		expect(hashFlow.step).toBe("error");
		expect(hashFlow.error).toBe("hash failed");
		expect(onError).toHaveBeenCalled();
		hashErrorSpy.mockRestore();
	});

	it("rejects oversized files via the validateFile result (no throw expected)", async () => {
		// Use the real validateFile contract shape: a result object.
		const { mod, blob } = await loadMediaUploadModule({
			validateImpl: (file, max) =>
				file.size > max
					? { valid: false, error: "File is too large" }
					: { valid: true },
		});

		const flow = mod.createMediaUploadFlow({
			checkDuplicate: vi.fn(async () => ({ exists: false })),
			createMedia: vi.fn() as any,
			initiateUpload: vi.fn() as any,
			finaliseUpload: vi.fn() as any,
			maxFileSize: 1000,
		});

		flow.setFile(fakeFile({ size: 5000 }));
		expect(flow.file).toBeNull();
		expect(flow.fileError).toBe("File is too large");
		expect(flow.canUpload).toBe(false);
		expect((blob.validateFile as any).mock.calls[0][1]).toBe(1000);

		flow.setFile(fakeFile({ size: 500 }));
		expect(flow.file).not.toBeNull();
		expect(flow.fileError).toBeNull();
	});

	it("cancels an in-progress upload back to select without an error state", async () => {
		const { BlobUploadError } = await import("../../src/patterns/blob-types.js");
		const { mod } = await loadMediaUploadModule({
			uploadImpl: async (_plan, _file, options) =>
				new Promise((_resolve, reject) => {
					options?.signal?.addEventListener("abort", () => {
						reject(new BlobUploadError("Upload was aborted", "UPLOAD_ABORTED"));
					});
				}) as any,
		});

		const onError = vi.fn();
		const flow = mod.createMediaUploadFlow({
			checkDuplicate: vi.fn(async () => ({ exists: false })),
			createMedia: vi.fn(async () => ({ id: "media-1" })) as any,
			initiateUpload: vi.fn(async () => ({
				versionId: "v-1",
				uploadPlan: {
					uploadUrl: "https://upload",
					method: "PUT",
					headers: {},
					expiresAt: "2099-01-01T00:00:00Z",
				},
			})) as any,
			finaliseUpload: vi.fn() as any,
			onError,
		});

		flow.setFile(fakeFile());
		const uploadPromise = flow.startUpload();

		await vi.waitFor(() => {
			expect(flow.step).toBe("uploading");
		});

		flow.cancelUpload();
		await uploadPromise;

		expect(flow.step).toBe("select");
		expect(flow.error).toBeNull();
		expect(flow.file).not.toBeNull();
		expect(onError).not.toHaveBeenCalled();
	});
});
