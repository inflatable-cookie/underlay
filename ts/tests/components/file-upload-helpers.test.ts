import { describe, expect, it } from "vitest";
import {
	formatFileSize,
	generateFileUploadId,
	validateUploadFile,
} from "../../src/components/file-upload/helpers";

describe("components/file-upload/helpers", () => {
	it("formats file sizes and generates ids", () => {
		expect(formatFileSize(0)).toBe("0 B");
		expect(formatFileSize(1024)).toBe("1 KB");
		expect(formatFileSize(1536)).toBe("1.5 KB");
		expect(formatFileSize(1024 * 1024)).toBe("1 MB");
		expect(generateFileUploadId()).toMatch(/^file-\d+-[a-z0-9]+$/);
	});

	it("validates file size/type and optional custom validator", () => {
		const pngFile = { size: 500, type: "image/png", name: "a.png" } as File;
		expect(validateUploadFile({ file: pngFile, maxSize: 1000, accept: "image/*,.pdf" })).toBeNull();

		const txtFile = { size: 500, type: "text/plain", name: "a.txt" } as File;
		expect(
			validateUploadFile({ file: txtFile, maxSize: 1000, accept: "image/*,.pdf" })
		).toContain("File type not accepted");

		const largeFile = { size: 5000, type: "image/png", name: "big.png" } as File;
		expect(
			validateUploadFile({ file: largeFile, maxSize: 1000, accept: "*" })
		).toContain("File too large");

		expect(
			validateUploadFile({
				file: pngFile,
				maxSize: 1000,
				accept: "*",
				validate: () => "custom error",
			})
		).toBe("custom error");

		const pdfFile = { size: 500, type: "application/pdf", name: "file.PDF" } as File;
		expect(
			validateUploadFile({ file: pdfFile, maxSize: 1000, accept: "application/pdf" })
		).toBeNull();
	});
});
