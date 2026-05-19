import { computeFileHash, uploadToBlob } from "./blob-upload";
import type { UploadPlan, UploadProgress } from "./blob-types";

export const DEFAULT_MEDIA_UPLOAD_MAX_FILE_SIZE = 50 * 1024 * 1024;

export type MediaWorkflowPaginationParams = {
  cursor?: string;
  limit?: number;
};

export type MediaWorkflowPageResponse<TItem> = {
  data: TItem[];
  nextCursor?: string | null;
  hasMore?: boolean;
};

export type MediaBrowseState<TItem> = {
  items: TItem[];
  nextCursor: string | null;
  hasMore: boolean;
};

export type LoadedMediaBrowsePage<TItem> = {
  items: TItem[];
  nextCursor: string | null;
  hasMore: boolean;
};

export type MediaUploadWorkflowStep =
  | "checking"
  | "duplicate"
  | "uploading"
  | "finalising"
  | "complete"
  | "error";

export type MediaUploadDisplayStep = "select" | MediaUploadWorkflowStep;

export type MediaDuplicateCheckResult<TExisting> = {
  exists: boolean;
  item?: TExisting | null;
};

export type MediaWorkflowUploadPlan = {
  uploadUrl: string;
  method: string;
  headers?: Record<string, string> | null;
  expiresAt: string;
  maxBytes?: number | null;
  allowedContentTypes?: string[] | null;
  objectKey?: string;
};

export type MediaUploadInitResult = {
  versionId: string;
  uploadPlan: MediaWorkflowUploadPlan;
};

export type MediaUploadDuplicateResult<TExisting> = {
  kind: "duplicate";
  fileHash: string;
  existingItem: TExisting;
};

export type MediaUploadCompleteResult<TCreated> = {
  kind: "uploaded";
  fileHash: string;
  createdItem: TCreated;
};

export type MediaUploadWorkflowResult<TExisting, TCreated> =
  | MediaUploadDuplicateResult<TExisting>
  | MediaUploadCompleteResult<TCreated>;

export type MediaUploadPipelineVisibility = "public" | "restricted";

export type MediaUploadPipelineResult = {
  mediaId: string;
  hash: string;
};

export type MediaUploadPipelineDuplicate<TMedia> = {
  hash: string;
  exists: boolean;
  media?: TMedia | null;
};

export type MediaUploadPipelineContextOptions<TContext extends object> = TContext & {
  file: File;
  onProgress?: (progress: UploadProgress) => void;
};

export type MediaCreateAndUploadPipelineOptions<TContext extends object> =
  MediaUploadPipelineContextOptions<TContext> & {
    title?: string | null;
    visibility?: string;
  };

export type MediaReplaceUploadPipelineOptions<TContext extends object> =
  MediaUploadPipelineContextOptions<TContext> & {
    mediaId: string;
  };

export type CreateMediaUploadPipelineConfig<
  TKind,
  TMediaRecord extends { id: string },
  TExisting,
  TContext extends object
> = {
  detectKind: (mimeType: string) => TKind;
  createMedia: (
    request: {
      kind: TKind;
      visibility: MediaUploadPipelineVisibility;
      originalFilename: string;
      title: string | null;
    },
    context: TContext
  ) => Promise<TMediaRecord>;
  initiateUpload: (
    mediaId: string,
    request: {
      contentType: string;
      contentLength: number;
      sha256?: string;
    },
    context: TContext
  ) => Promise<MediaUploadInitResult>;
  finaliseUpload: (
    mediaId: string,
    versionId: string,
    request: {
      sha256: string;
      contentType: string;
    },
    context: TContext
  ) => Promise<unknown>;
  checkDuplicate: (
    request: {
      sha256: string;
    },
    context: TContext
  ) => Promise<{
    exists: boolean;
    media?: TExisting | null;
  }>;
  maxFileSize?: number;
  includeHashInInitiate?: boolean;
};

export type MediaCreateUploadOptions<TKind, TMediaRecord extends { id: string }> = {
  file: File;
  title?: string | null;
  visibility?: MediaUploadPipelineVisibility | string;
  maxFileSize?: number;
  includeHashInInitiate?: boolean;
  onProgress?: (progress: UploadProgress) => void;
  detectKind: (mimeType: string) => TKind;
  createMedia: (request: {
    kind: TKind;
    visibility: MediaUploadPipelineVisibility;
    originalFilename: string;
    title: string | null;
  }) => Promise<TMediaRecord>;
  initiateUpload: (
    mediaId: string,
    request: {
      contentType: string;
      contentLength: number;
      sha256?: string;
    }
  ) => Promise<MediaUploadInitResult>;
  finaliseUpload: (
    mediaId: string,
    versionId: string,
    request: {
      sha256: string;
      contentType: string;
    }
  ) => Promise<unknown>;
};

export type MediaReplaceUploadOptions = {
  file: File;
  mediaId: string;
  maxFileSize?: number;
  includeHashInInitiate?: boolean;
  onProgress?: (progress: UploadProgress) => void;
  initiateUpload: (
    mediaId: string,
    request: {
      contentType: string;
      contentLength: number;
      sha256?: string;
    }
  ) => Promise<MediaUploadInitResult>;
  finaliseUpload: (
    mediaId: string,
    versionId: string,
    request: {
      sha256: string;
      contentType: string;
    }
  ) => Promise<unknown>;
};

export type CheckMediaDuplicateFileOptions<TMedia> = {
  file: File;
  checkDuplicate: (
    request: {
      sha256: string;
    }
  ) => Promise<{
    exists: boolean;
    media?: TMedia | null;
  }>;
};

export type LoadMediaBrowsePageInput<TItem> = {
  listPage: (
    params?: MediaWorkflowPaginationParams
  ) => Promise<MediaWorkflowPageResponse<TItem>>;
  cursor?: string;
  limit?: number;
};

export async function loadMediaBrowsePage<TItem>({
  listPage,
  cursor,
  limit = 12
}: LoadMediaBrowsePageInput<TItem>): Promise<LoadedMediaBrowsePage<TItem>> {
  const response = await listPage({
    cursor: cursor ?? undefined,
    limit
  });

  return {
    items: response.data,
    nextCursor: response.nextCursor ?? null,
    hasMore: response.hasMore ?? false
  };
}

export function mergeMediaBrowseItems<TItem>(
  existingItems: TItem[],
  nextItems: TItem[],
  cursor?: string
): TItem[] {
  return cursor ? [...existingItems, ...nextItems] : nextItems;
}

export function createResetMediaBrowseState<TItem>(): MediaBrowseState<TItem> {
  return {
    items: [],
    nextCursor: null,
    hasMore: false
  };
}

export type UploadMediaWithKnownHashInput<
  TCreated,
  TCreateRequest,
  TCreatedRecord,
  TInitiateRequest,
  TFinaliseRequest,
  TFinaliseResult
> = {
  file: File;
  fileHash: string;
  maxFileSize: number;
  createRecord: (request: TCreateRequest) => Promise<TCreatedRecord>;
  buildCreateRequest: (file: File, fileHash: string) => TCreateRequest;
  initiateUpload: (
    createdRecord: TCreatedRecord,
    request: TInitiateRequest
  ) => Promise<MediaUploadInitResult>;
  buildInitiateRequest: (file: File, fileHash: string) => TInitiateRequest;
  finaliseUpload: (
    createdRecord: TCreatedRecord,
    versionId: string,
    request: TFinaliseRequest
  ) => Promise<TFinaliseResult>;
  buildFinaliseRequest: (file: File, fileHash: string) => TFinaliseRequest;
  toCreatedItem: (
    finaliseResult: TFinaliseResult,
    createdRecord: TCreatedRecord
  ) => TCreated;
  onStep?: (step: "uploading" | "finalising") => void;
  onProgress?: (percent: number) => void;
};

export async function uploadMediaWithKnownHash<
  TCreated,
  TCreateRequest,
  TCreatedRecord,
  TInitiateRequest,
  TFinaliseRequest,
  TFinaliseResult
>(
  input: UploadMediaWithKnownHashInput<
    TCreated,
    TCreateRequest,
    TCreatedRecord,
    TInitiateRequest,
    TFinaliseRequest,
    TFinaliseResult
  >
): Promise<TCreated> {
  const createdRecord = await input.createRecord(
    input.buildCreateRequest(input.file, input.fileHash)
  );

  const uploadStart = await input.initiateUpload(
    createdRecord,
    input.buildInitiateRequest(input.file, input.fileHash)
  );

  input.onStep?.("uploading");
  await uploadToBlob(toUploadPlan(uploadStart.uploadPlan, input.maxFileSize), input.file, {
    onProgress: input.onProgress
      ? (progress) => {
          input.onProgress?.(progress.percent);
        }
      : undefined
  });

  input.onStep?.("finalising");
  const finaliseResult = await input.finaliseUpload(
    createdRecord,
    uploadStart.versionId,
    input.buildFinaliseRequest(input.file, input.fileHash)
  );

  return input.toCreatedItem(finaliseResult, createdRecord);
}

export type RunMediaUploadWorkflowInput<
  TExisting,
  TCreated,
  TCreateRequest,
  TCreatedRecord,
  TInitiateRequest,
  TFinaliseRequest,
  TFinaliseResult
> = UploadMediaWithKnownHashInput<
  TCreated,
  TCreateRequest,
  TCreatedRecord,
  TInitiateRequest,
  TFinaliseRequest,
  TFinaliseResult
> & {
  checkDuplicate: (sha256: string) => Promise<MediaDuplicateCheckResult<TExisting>>;
  onStep?: (step: "checking" | "uploading" | "finalising") => void;
};

export async function runMediaUploadWorkflow<
  TExisting,
  TCreated,
  TCreateRequest,
  TCreatedRecord,
  TInitiateRequest,
  TFinaliseRequest,
  TFinaliseResult
>(
  input: RunMediaUploadWorkflowInput<
    TExisting,
    TCreated,
    TCreateRequest,
    TCreatedRecord,
    TInitiateRequest,
    TFinaliseRequest,
    TFinaliseResult
  >
): Promise<MediaUploadWorkflowResult<TExisting, TCreated>> {
  input.onStep?.("checking");

  const fileHash = await computeFileHash(input.file);
  const duplicateCheck = await input.checkDuplicate(fileHash);

  if (duplicateCheck.exists && duplicateCheck.item) {
    return {
      kind: "duplicate",
      fileHash,
      existingItem: duplicateCheck.item
    };
  }

  const createdItem = await uploadMediaWithKnownHash({
    ...input,
    fileHash
  });

  return {
    kind: "uploaded",
    fileHash,
    createdItem
  };
}

export async function createMediaAndUpload<
  TKind,
  TMediaRecord extends { id: string }
>(
  input: MediaCreateUploadOptions<TKind, TMediaRecord>
): Promise<MediaUploadPipelineResult> {
  const hash = await computeFileHash(input.file);
  const mediaRecord = await input.createMedia({
    kind: input.detectKind(input.file.type),
    visibility: normalizeUploadVisibility(input.visibility),
    originalFilename: input.file.name,
    title: input.title ?? null
  });

  await uploadMediaVersion({
    file: input.file,
    mediaId: mediaRecord.id,
    hash,
    maxFileSize: input.maxFileSize,
    includeHashInInitiate: input.includeHashInInitiate,
    onProgress: input.onProgress,
    initiateUpload: input.initiateUpload,
    finaliseUpload: input.finaliseUpload
  });

  return {
    mediaId: mediaRecord.id,
    hash
  };
}

export async function replaceMediaUpload(
  input: MediaReplaceUploadOptions
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
    finaliseUpload: input.finaliseUpload
  });

  return {
    mediaId: input.mediaId,
    hash
  };
}

export async function checkMediaDuplicateFile<TMedia>(
  input: CheckMediaDuplicateFileOptions<TMedia>
): Promise<MediaUploadPipelineDuplicate<TMedia>> {
  const hash = await computeFileHash(input.file);
  const result = await input.checkDuplicate({ sha256: hash });

  return {
    hash,
    exists: result.exists,
    media: result.media
  };
}

export function createMediaUploadPipeline<
  TKind,
  TMediaRecord extends { id: string },
  TExisting = unknown,
  TContext extends object = {}
>(
  config: CreateMediaUploadPipelineConfig<TKind, TMediaRecord, TExisting, TContext>
) {
  const maxFileSize = config.maxFileSize ?? DEFAULT_MEDIA_UPLOAD_MAX_FILE_SIZE;
  const includeHashInInitiate = config.includeHashInInitiate ?? false;

  async function createAndUpload(
    options: MediaCreateAndUploadPipelineOptions<TContext>
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
      createMedia: (request) => config.createMedia(request, context as TContext),
      initiateUpload: (mediaId, request) =>
        config.initiateUpload(mediaId, request, context as TContext),
      finaliseUpload: (mediaId, versionId, request) =>
        config.finaliseUpload(mediaId, versionId, request, context as TContext)
    });
  }

  async function replaceUpload(
    options: MediaReplaceUploadPipelineOptions<TContext>
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
        config.finaliseUpload(id, versionId, request, context as TContext)
    });
  }

  async function checkDuplicate(
    file: File,
    context: TContext
  ): Promise<MediaUploadPipelineDuplicate<TExisting>> {
    return checkMediaDuplicateFile({
      file,
      checkDuplicate: (request) => config.checkDuplicate(request, context)
    });
  }

  return {
    maxFileSize,
    createAndUpload,
    replaceUpload,
    checkDuplicate
  };
}

async function uploadMediaVersion(input: {
  file: File;
  mediaId: string;
  hash: string;
  maxFileSize?: number;
  includeHashInInitiate?: boolean;
  onProgress?: (progress: UploadProgress) => void;
  initiateUpload: MediaReplaceUploadOptions["initiateUpload"];
  finaliseUpload: MediaReplaceUploadOptions["finaliseUpload"];
}): Promise<void> {
  const uploadInfo = await input.initiateUpload(input.mediaId, {
    contentType: input.file.type,
    contentLength: input.file.size,
    ...(input.includeHashInInitiate ? { sha256: input.hash } : {})
  });

  await uploadToBlob(
    toUploadPlan(
      uploadInfo.uploadPlan,
      input.maxFileSize ?? DEFAULT_MEDIA_UPLOAD_MAX_FILE_SIZE
    ),
    input.file,
    {
      onProgress: input.onProgress
    }
  );

  await input.finaliseUpload(input.mediaId, uploadInfo.versionId, {
    sha256: input.hash,
    contentType: input.file.type
  });
}

function normalizeUploadVisibility(
  visibility?: MediaUploadPipelineVisibility | string
): MediaUploadPipelineVisibility {
  return visibility === "restricted" ? "restricted" : "public";
}

function toUploadPlan(plan: MediaWorkflowUploadPlan, maxFileSize: number): UploadPlan {
  return {
    uploadUrl: plan.uploadUrl,
    method: plan.method,
    requiredHeaders: plan.headers ?? {},
    expiresAt: plan.expiresAt,
    maxBytes: plan.maxBytes ?? maxFileSize,
    allowedContentTypes: plan.allowedContentTypes ?? [],
    objectKey: plan.objectKey ?? ""
  };
}
