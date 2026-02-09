export function generateFileUploadId(): string {
	return `file-${Date.now()}-${Math.random().toString(36).slice(2, 11)}`;
}

export function formatFileSize(bytes: number): string {
	if (bytes === 0) return "0 B";
	const k = 1024;
	const sizes = ["B", "KB", "MB", "GB"];
	const i = Math.floor(Math.log(bytes) / Math.log(k));
	return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`;
}

function isAcceptedFileType(file: File, accept: string): boolean {
	const acceptedTypes = accept.split(",").map((type) => type.trim());
	const fileType = file.type;
	const fileExtension = `.${file.name.split(".").pop()?.toLowerCase()}`;

	return acceptedTypes.some((acceptedType) => {
		if (acceptedType.startsWith(".")) {
			return fileExtension === acceptedType.toLowerCase();
		}
		if (acceptedType.endsWith("/*")) {
			return fileType.startsWith(acceptedType.slice(0, -1));
		}
		return fileType === acceptedType;
	});
}

interface ValidateFileInput {
	file: File;
	maxSize: number;
	accept: string;
	validate?: (file: File) => string | null;
}

export function validateUploadFile({ file, maxSize, accept, validate }: ValidateFileInput): string | null {
	if (file.size > maxSize) {
		return `File too large. Maximum size is ${formatFileSize(maxSize)}`;
	}

	if (accept !== "*" && !isAcceptedFileType(file, accept)) {
		return `File type not accepted. Accepted types: ${accept}`;
	}

	return validate ? validate(file) : null;
}
