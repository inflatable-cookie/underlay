import {
	uploadToBlob,
	type UploadPlan,
	type UploadProgress,
	MediaKind,
	MediaVisibility,
	type CreateMediaRequest,
	type FinaliseUploadRequest,
	type FinaliseUploadResponse,
	type InitiateUploadRequest,
	type InitiateUploadResponse,
	type MediaDetail,
	type MediaSummary
} from "../../patterns/index.js";

function getMediaKindFromFile(file: File): MediaKind {
	if (file.type.startsWith("image/")) return MediaKind.Image;
	if (file.type === "application/pdf") return MediaKind.Pdf;
	return MediaKind.Image;
}

function toCreatedMediaSummary(media: MediaDetail): MediaSummary {
	const version = media.currentVersion;
	return {
		id: media.id,
		kind: media.kind,
		visibility: media.visibility,
		title: media.title,
		originalFilename: media.originalFilename,
		currentVersionId: media.currentVersionId,
		createdAt: media.createdAt,
		updatedAt: media.updatedAt,
		deletedAt: null,
		byteSize: version?.byteSize ?? null,
		mimeType: version?.mimeType ?? null,
		thumbnailUrl: null
	};
}

interface UploadNewMediaInput {
	file: File;
	fileHash: string;
	maxFileSize: number;
	createMedia: (input: CreateMediaRequest) => Promise<MediaDetail>;
	initiateUpload: (
		mediaId: string,
		input: InitiateUploadRequest
	) => Promise<InitiateUploadResponse>;
	finaliseUpload: (
		mediaId: string,
		versionId: string,
		input: FinaliseUploadRequest
	) => Promise<FinaliseUploadResponse>;
	onStage?: (stage: "uploading" | "finalising") => void;
	onProgress: (percent: number) => void;
}

export async function uploadNewMedia(input: UploadNewMediaInput): Promise<MediaSummary> {
	const media = await input.createMedia({
		kind: getMediaKindFromFile(input.file),
		visibility: MediaVisibility.Public,
		title: input.file.name.replace(/\.[^/.]+$/, ""),
		originalFilename: input.file.name
	});

	const uploadResponse = await input.initiateUpload(media.id, {
		contentType: input.file.type,
		contentLength: input.file.size,
		sha256: input.fileHash
	});

	const plan: UploadPlan = {
		uploadUrl: uploadResponse.uploadPlan.uploadUrl,
		method: uploadResponse.uploadPlan.method,
		requiredHeaders: uploadResponse.uploadPlan.headers,
		expiresAt: uploadResponse.uploadPlan.expiresAt,
		maxBytes: uploadResponse.uploadPlan.maxBytes || input.maxFileSize,
		allowedContentTypes: uploadResponse.uploadPlan.allowedContentTypes || [],
		objectKey: ""
	};

	input.onStage?.("uploading");
	await uploadToBlob(plan, input.file, {
		onProgress: (progress: UploadProgress) => {
			input.onProgress(progress.percent);
		}
	});

	input.onStage?.("finalising");
	const finaliseResponse = await input.finaliseUpload(media.id, uploadResponse.versionId, {
		sha256: input.fileHash,
		contentType: input.file.type
	});

	// Use the media detail from the finalise response — it has the
	// up-to-date version info including renditions / thumbnail URLs.
	return toCreatedMediaSummary(finaliseResponse.media);
}
