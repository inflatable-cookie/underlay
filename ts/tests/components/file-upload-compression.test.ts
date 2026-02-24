import { describe, expect, it } from "vitest";
import { compressImage } from "../../src/components/file-upload/compression";

describe("components/file-upload/compression", () => {
	it("returns original file for non-image and excluded image formats", async () => {
		const textFile = new File(["abc"], "a.txt", { type: "text/plain" });
		const svgFile = new File(["<svg/>"], "a.svg", { type: "image/svg+xml" });
		const gifFile = new File(["gif"], "a.gif", { type: "image/gif" });

		await expect(compressImage(textFile)).resolves.toBe(textFile);
		await expect(compressImage(svgFile)).resolves.toBe(svgFile);
		await expect(compressImage(gifFile)).resolves.toBe(gifFile);
	});
});
