import { computeFileHash, uploadToBlob } from "../blob-upload";
import { DEFAULT_MEDIA_UPLOAD_MAX_FILE_SIZE, toUploadPlan } from "./plan";
import type {
  CheckMediaDuplicateFileOptions,
  CreateMediaUploadPipelineConfig,
  MediaCreateAndUploadPipelineOptions,
  MediaCreateUploadOptions,
  MediaReplaceUploadOptions,
  MediaReplaceUploadPipelineOptions,
  MediaUploadPipelineDuplicate,
  MediaUploadPipelineResult,
  MediaUploadPipelineVisibility,
} from "./types";

export async function createMediaAndUpload<
  TKind,
  TMediaRecord extends { id: string },
>(
  input: MediaCreateUploadOptions<TKind, TMediaRecord>,
): Promise<MediaUploadPipelineResult> {
  const hash = await computeFileHash(input.file);
  const mediaRecord = await input.createMedia({
    kind: input.detectKind(input.file.type),
    visibility: normalizeUploadVisibility(input.visibility),
    originalFilename: input.file.name,
    title: input.title ?? null,
  });

  await uploadMediaVersion({
    file: input.file,
    mediaId: mediaRecord.id,
    hash,
    maxFileSize: input.maxFileSize,
    includeHashInInitiate: input.includeHashInInitiate,
    onProgress: input.onProgress,
    initiateUpload: input.initiateUpload,
    finaliseUpload: input.finaliseUpload,
  });

  return {
    mediaId: mediaRecord.id,
    hash,
  };
}

export async function replaceMediaUpload(
  input: MediaReplaceUploadOptions,
): Promise<MediaUploadPipelineResult> {
  const hash = await computeFileHash(input.file);

  await uploadMediaVersion({
    file: input.file,
    mediaId: input.mediaId,
    hash,
    maxFileSize: input.maxFileSize,
    includeHashInInitiate: input.includeHashInInitiate,
    onProgress: input.onProgress,
    initiateUpload: input.initiateUpload,
    finaliseUpload: input.finaliseUpload,
  });

  return {
    mediaId: input.mediaId,
    hash,
  };
}

export async function checkMediaDuplicateFile<TMedia>(
  input: CheckMediaDuplicateFileOptions<TMedia>,
): Promise<MediaUploadPipelineDuplicate<TMedia>> {
  const hash = await computeFileHash(input.file);
  const result = await input.checkDuplicate({ sha256: hash });

  return {
    hash,
    exists: result.exists,
    media: result.media,
  };
}

export function createMediaUploadPipeline<
  TKind,
  TMediaRecord extends { id: string },
  TExisting = unknown,
  TContext extends object = {},
>(
  config: CreateMediaUploadPipelineConfig<
    TKind,
    TMediaRecord,
    TExisting,
    TContext
  >,
) {
  const maxFileSize = config.maxFileSize ?? DEFAULT_MEDIA_UPLOAD_MAX_FILE_SIZE;
  const includeHashInInitiate = config.includeHashInInitiate ?? false;

  async function createAndUpload(
    options: MediaCreateAndUploadPipelineOptions<TContext>,
  ): Promise<MediaUploadPipelineResult> {
    const { file, onProgress, title, visibility, ...context } = options;

    return createMediaAndUpload({
      file,
      title,
      visibility,
      maxFileSize,
      includeHashInInitiate,
      onProgress,
      detectKind: config.detectKind,
      createMedia: (request) =>
        config.createMedia(request, context as TContext),
      initiateUpload: (mediaId, request) =>
        config.initiateUpload(mediaId, request, context as TContext),
      finaliseUpload: (mediaId, versionId, request) =>
        config.finaliseUpload(mediaId, versionId, request, context as TContext),
    });
  }

  async function replaceUpload(
    options: MediaReplaceUploadPipelineOptions<TContext>,
  ): Promise<MediaUploadPipelineResult> {
    const { file, mediaId, onProgress, ...context } = options;

    return replaceMediaUpload({
      file,
      mediaId,
      maxFileSize,
      includeHashInInitiate,
      onProgress,
      initiateUpload: (id, request) =>
        config.initiateUpload(id, request, context as TContext),
      finaliseUpload: (id, versionId, request) =>
        config.finaliseUpload(id, versionId, request, context as TContext),
    });
  }

  async function checkDuplicate(
    file: File,
    context: TContext,
  ): Promise<MediaUploadPipelineDuplicate<TExisting>> {
    return checkMediaDuplicateFile({
      file,
      checkDuplicate: (request) => config.checkDuplicate(request, context),
    });
  }

  return {
    maxFileSize,
    createAndUpload,
    replaceUpload,
    checkDuplicate,
  };
}

async function uploadMediaVersion(input: {
  file: File;
  mediaId: string;
  hash: string;
  maxFileSize?: number;
  includeHashInInitiate?: boolean;
  onProgress?: MediaReplaceUploadOptions["onProgress"];
  initiateUpload: MediaReplaceUploadOptions["initiateUpload"];
  finaliseUpload: MediaReplaceUploadOptions["finaliseUpload"];
}): Promise<void> {
  const uploadInfo = await input.initiateUpload(input.mediaId, {
    contentType: input.file.type,
    contentLength: input.file.size,
    ...(input.includeHashInInitiate ? { sha256: input.hash } : {}),
  });

  await uploadToBlob(
    toUploadPlan(
      uploadInfo.uploadPlan,
      input.maxFileSize ?? DEFAULT_MEDIA_UPLOAD_MAX_FILE_SIZE,
    ),
    input.file,
    {
      onProgress: input.onProgress,
    },
  );

  await input.finaliseUpload(input.mediaId, uploadInfo.versionId, {
    sha256: input.hash,
    contentType: input.file.type,
  });
}

function normalizeUploadVisibility(
  visibility?: MediaUploadPipelineVisibility | string,
): MediaUploadPipelineVisibility {
  return visibility === "restricted" ? "restricted" : "public";
}
