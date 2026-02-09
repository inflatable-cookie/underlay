export interface ImageCompressionOptions {
	/** Maximum width in pixels */
	maxWidth?: number;
	/** Maximum height in pixels */
	maxHeight?: number;
	/** JPEG/WebP quality (0-1) */
	quality?: number;
	/** Output format (defaults to original or 'image/jpeg' for non-web formats) */
	format?: "image/jpeg" | "image/png" | "image/webp";
}

export const DEFAULT_COMPRESSION: ImageCompressionOptions = {
	maxWidth: 1920,
	maxHeight: 1080,
	quality: 0.85
};

export async function compressImage(
	file: File,
	options: ImageCompressionOptions = DEFAULT_COMPRESSION
): Promise<File> {
	if (!file.type.startsWith("image/")) {
		return file;
	}

	if (file.type === "image/svg+xml" || file.type === "image/gif") {
		return file;
	}

	const { maxWidth = 1920, maxHeight = 1080, quality = 0.85, format } = options;

	return new Promise((resolve) => {
		const img = new Image();
		const canvas = document.createElement("canvas");
		const ctx = canvas.getContext("2d");

		img.onload = () => {
			let { width, height } = img;

			if (width > maxWidth || height > maxHeight) {
				const ratio = Math.min(maxWidth / width, maxHeight / height);
				width = Math.round(width * ratio);
				height = Math.round(height * ratio);
			}

			canvas.width = width;
			canvas.height = height;

			if (ctx) {
				ctx.drawImage(img, 0, 0, width, height);

				const outputFormat = format || (file.type === "image/png" ? "image/png" : "image/jpeg");

				canvas.toBlob(
					(blob) => {
						if (blob && blob.size < file.size) {
							resolve(
								new File([blob], file.name, {
									type: outputFormat,
									lastModified: Date.now()
								})
							);
						} else {
							resolve(file);
						}
					},
					outputFormat,
					quality
				);
			} else {
				resolve(file);
			}

			URL.revokeObjectURL(img.src);
		};

		img.onerror = () => {
			URL.revokeObjectURL(img.src);
			resolve(file);
		};

		img.src = URL.createObjectURL(file);
	});
}
