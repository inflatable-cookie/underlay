import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
	uploadToBlob,
	computeFileHash,
	validateFileType,
	isVideoFile,
	validateFileSize,
	formatFileSize,
	getFileTypeDescription,
	validateFile,
} from "../../src/patterns/blob-upload";
import { BlobUploadError } from "../../src/patterns/blob-types";

class MockXHR {
	static nextBehavior: "success" | "successNoLength" | "httpError" | "networkError" | "pending" = "success";
	status = 200;
	statusText = "OK";
	headers: Record<string, string> = {};
	uploadListeners: Record<string, (event: any) => void> = {};
	listeners: Record<string, () => void> = {};

	upload = {
		addEventListener: (event: string, cb: (event: any) => void) => {
			this.uploadListeners[event] = cb;
		},
	};

	open(_method: string, _url: string): void {}
	setRequestHeader(key: string, value: string): void {
		this.headers[key] = value;
	}
	addEventListener(event: string, cb: () => void): void {
		this.listeners[event] = cb;
	}
	send(_file: File): void {
		if (MockXHR.nextBehavior === "pending") return;
		if (MockXHR.nextBehavior === "networkError") {
			this.listeners.error?.();
			return;
		}
		this.uploadListeners.progress?.({
			lengthComputable: MockXHR.nextBehavior !== "successNoLength",
			loaded: 5,
			total: 10,
		});
		if (MockXHR.nextBehavior === "httpError") {
			this.status = 403;
			this.statusText = "Forbidden";
		}
		this.listeners.load?.();
	}
	abort(): void {
		this.listeners.abort?.();
	}
}

describe("patterns/blob-upload", () => {
	const originalXhr = globalThis.XMLHttpRequest;

	beforeEach(() => {
		(globalThis as any).XMLHttpRequest = MockXHR;
		MockXHR.nextBehavior = "success";
	});

	afterEach(() => {
		(globalThis as any).XMLHttpRequest = originalXhr;
	});

	it("validates upload constraints before sending", async () => {
		const file = new File(["hello"], "f.txt", { type: "text/plain" });
		const basePlan = {
			uploadUrl: "https://upload",
			method: "PUT",
			requiredHeaders: {},
			maxBytes: 2,
			allowedContentTypes: ["text/plain"],
			expiresAt: "2099-01-01T00:00:00.000Z",
			objectKey: "obj",
		};

		await expect(uploadToBlob(basePlan as any, file)).rejects.toMatchObject({ code: "FILE_TOO_LARGE" });
		await expect(uploadToBlob({ ...basePlan, maxBytes: 100, allowedContentTypes: ["image/png"] } as any, file)).rejects.toMatchObject({ code: "INVALID_CONTENT_TYPE" });
		await expect(uploadToBlob({ ...basePlan, maxBytes: 100, expiresAt: "2000-01-01T00:00:00.000Z" } as any, file)).rejects.toMatchObject({ code: "UPLOAD_EXPIRED" });
	});

	it("uploads successfully and reports progress", async () => {
		const file = new File(["hello"], "f.txt", { type: "text/plain" });
		const onProgress = vi.fn();
		const plan = {
			uploadUrl: "https://upload",
			method: "PUT",
			requiredHeaders: { "x-custom": "abc" },
			maxBytes: 100,
			allowedContentTypes: ["text/plain"],
			expiresAt: "2099-01-01T00:00:00.000Z",
			objectKey: "obj-1",
		};

		await expect(uploadToBlob(plan as any, file, { onProgress })).resolves.toEqual({
			objectKey: "obj-1",
			size: file.size,
			contentType: "text/plain",
		});
		expect(onProgress).toHaveBeenCalledWith({ loaded: 5, total: 10, percent: 50 });
	});

	it("skips progress callback when progress event is not length-computable", async () => {
		const file = new File(["hello"], "f.txt", { type: "text/plain" });
		const onProgress = vi.fn();
		const plan = {
			uploadUrl: "https://upload",
			method: "PUT",
			requiredHeaders: {},
			maxBytes: 100,
			allowedContentTypes: ["text/plain"],
			expiresAt: "2099-01-01T00:00:00.000Z",
			objectKey: "obj-1",
		};

		MockXHR.nextBehavior = "successNoLength";
		await expect(uploadToBlob(plan as any, file, { onProgress })).resolves.toEqual({
			objectKey: "obj-1",
			size: file.size,
			contentType: "text/plain",
		});
		expect(onProgress).not.toHaveBeenCalled();
	});

	it("maps xhr failures to BlobUploadError variants", async () => {
		const file = new File(["hello"], "f.txt", { type: "text/plain" });
		const plan = {
			uploadUrl: "https://upload",
			method: "PUT",
			requiredHeaders: {},
			maxBytes: 100,
			allowedContentTypes: ["text/plain"],
			expiresAt: "2099-01-01T00:00:00.000Z",
			objectKey: "obj",
		};

		MockXHR.nextBehavior = "httpError";
		await expect(uploadToBlob(plan as any, file)).rejects.toMatchObject({ code: "UPLOAD_FAILED", statusCode: 403 });

		MockXHR.nextBehavior = "networkError";
		await expect(uploadToBlob(plan as any, file)).rejects.toMatchObject({ code: "NETWORK_ERROR" });

		MockXHR.nextBehavior = "pending";
		const controller = new AbortController();
		const pending = uploadToBlob(plan as any, file, { signal: controller.signal });
		controller.abort();
		await expect(pending).rejects.toMatchObject({ code: "UPLOAD_ABORTED" });
	});

	it("computes file hash for small and large files", async () => {
		const small = new File([new Uint8Array([1, 2, 3])], "a.bin", { type: "application/octet-stream" });
		const large = {
			size: 70 * 1024 * 1024,
			arrayBuffer: async () => new Uint8Array([1, 2, 3]).buffer,
		} as unknown as File;

		const smallHash = await computeFileHash(small);
		const largeHash = await computeFileHash(large);
		expect(smallHash).toMatch(/^[a-f0-9]{64}$/);
		expect(largeHash).toBe(smallHash);
	});

	it("validates helper utilities for type, size, and messaging", () => {
		const image = new File(["x"], "a.png", { type: "image/png" });
		const video = new File(["x"], "a.mp4", { type: "video/mp4" });
		const unknown = new File(["x"], "a.bin", { type: "application/octet-stream" });

		expect(validateFileType(image)).toBe(true);
		expect(validateFileType(unknown)).toBe(false);
		expect(isVideoFile(video)).toBe(true);
		expect(isVideoFile(image)).toBe(false);
		expect(validateFileSize(image, 1)).toBe(true);
		expect(validateFileSize(image, 0)).toBe(false);

		expect(formatFileSize(undefined)).toBe("—");
		expect(formatFileSize(0)).toBe("0 B");
		expect(formatFileSize(1024)).toBe("1.0 KB");
		expect(getFileTypeDescription("image/png")).toBe("PNG image");
		expect(getFileTypeDescription("foo/bar")).toBe("foo/bar");

		expect(validateFile(video, 100)).toEqual({
			valid: false,
			error: "Video files are not supported. Please use Vimeo for video content.",
		});
			expect(validateFile(unknown, 100)).toEqual(
				expect.objectContaining({ valid: false, error: expect.stringContaining("not supported") })
			);
			expect(validateFile(new File(["x"], "unknown.bin", { type: "" }), 100)).toEqual(
				expect.objectContaining({ valid: false, error: expect.stringContaining('"unknown"') })
			);
			expect(validateFile(new File([new Uint8Array(5)], "a.png", { type: "image/png" }), 2)).toEqual(
				expect.objectContaining({ valid: false, error: expect.stringContaining("too large") })
			);
		expect(validateFile(image, 100)).toEqual({ valid: true });
	});

	it("BlobUploadError preserves code and status", () => {
		const err = new BlobUploadError("nope", "UPLOAD_FAILED", 500);
		expect(err).toBeInstanceOf(Error);
		expect(err.name).toBe("BlobUploadError");
		expect(err.message).toBe("nope");
		expect(err.code).toBe("UPLOAD_FAILED");
		expect(err.statusCode).toBe(500);
	});
});
