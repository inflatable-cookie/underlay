/**
 * Media Upload Flow - Reusable state machine for media upload operations.
 *
 * This pattern encapsulates the common upload flow:
 * 1. select - File selection
 * 2. checking - Computing hash, checking for duplicates
 * 3. duplicate - Duplicate found, user chooses
 * 4. uploading - Uploading to blob storage with progress
 * 5. finalising - Finalising upload
 * 6. complete - Success
 * 7. error - Error state
 *
 * Usage:
 * ```typescript
 * const uploadFlow = createMediaUploadFlow({
 *   checkDuplicate: async (sha256) => { ... },
 *   createMedia: async (request) => { ... },
 *   initiateUpload: async (mediaId, request) => { ... },
 *   finaliseUpload: async (mediaId, versionId, request) => { ... },
 *   onComplete: (media) => { ... },
 * });
 *
 * // In template:
 * // {#if uploadFlow.step === 'select'}...{/if}
 * // <button onclick={() => uploadFlow.startUpload()}>Upload</button>
 * ```
 */

import { uploadToBlob, computeFileHash, validateFile } from "./blob-upload.js";
import { type UploadPlan } from "./blob-types.js";
import { DEFAULT_MEDIA_UPLOAD_MAX_FILE_SIZE } from "./media-workflow/plan.js";
import {
  type MediaSummary,
  type MediaDetail,
  type CreateMediaRequest,
  type CheckDuplicateResponse,
  type InitiateUploadRequest,
  type InitiateUploadResponse,
  type FinaliseUploadRequest,
  type FinaliseUploadResponse,
} from "./media-types.js";
import { MediaVisibility } from "./media-types/enums.js";
import { detectMediaKindFromMimeType } from "./media-types/labels.js";

/**
 * Upload step in the state machine
 */
export type MediaUploadStep =
  | "select"
  | "checking"
  | "duplicate"
  | "uploading"
  | "finalising"
  | "complete"
  | "error";

/**
 * Options for creating a media upload flow
 */
export interface MediaUploadFlowOptions {
  /** Check for duplicate files by hash */
  checkDuplicate: (sha256: string) => Promise<CheckDuplicateResponse>;

  /** Create a new media item (metadata only) */
  createMedia: (request: CreateMediaRequest) => Promise<MediaDetail>;

  /** Initiate upload for a media item */
  initiateUpload: (
    mediaId: string,
    request: InitiateUploadRequest
  ) => Promise<InitiateUploadResponse>;

  /** Finalise upload after blob is stored */
  finaliseUpload: (
    mediaId: string,
    versionId: string,
    request: FinaliseUploadRequest
  ) => Promise<FinaliseUploadResponse>;

  /** Called when upload completes successfully */
  onComplete?: (media: MediaDetail) => void;

  /** Called when an error occurs */
  onError?: (error: Error) => void;

  /** Maximum file size in bytes (default DEFAULT_MEDIA_UPLOAD_MAX_FILE_SIZE, 50MB) */
  maxFileSize?: number;

  /** Default visibility for new media (default: Public) */
  defaultVisibility?: MediaVisibility;

  // For "replace file" flow (adding version to existing media)
  /** Existing media ID (for replace flow) */
  existingMediaId?: string;

  /** Existing version hashes to check against (for replace flow) */
  existingVersionHashes?: string[];
}

/**
 * Media upload flow controller
 */
export interface MediaUploadFlowController {
  // Reactive state (read-only from outside)
  readonly step: MediaUploadStep;
  readonly file: File | null;
  readonly fileError: string | null;
  readonly fileHash: string | null;
  readonly progress: number;
  readonly error: string | null;
  readonly duplicateMedia: MediaSummary | null;
  readonly createdMedia: MediaDetail | null;

  // Computed
  readonly canUpload: boolean;
  readonly isUploading: boolean;

  // Actions
  setFile: (file: File) => void;
  clearFile: () => void;
  startUpload: (metadata?: Partial<CreateMediaRequest>) => Promise<void>;
  proceedWithUpload: (metadata?: Partial<CreateMediaRequest>) => Promise<void>;
  cancelUpload: () => void;
  useDuplicate: () => void;
  reset: () => void;
}

/**
 * Create a media upload flow controller with Svelte 5 runes.
 */
export function createMediaUploadFlow(
  options: MediaUploadFlowOptions
): MediaUploadFlowController {
  const {
    checkDuplicate,
    createMedia,
    initiateUpload,
    finaliseUpload,
    onComplete,
    onError,
    maxFileSize = DEFAULT_MEDIA_UPLOAD_MAX_FILE_SIZE,
    defaultVisibility = MediaVisibility.Public,
    existingMediaId,
    existingVersionHashes = [],
  } = options;

  // State
  let abortController: AbortController | null = null;
  let runGeneration = 0;
  let step = $state<MediaUploadStep>("select");
  let file = $state<File | null>(null);
  let fileError = $state<string | null>(null);
  let fileHash = $state<string | null>(null);
  let progress = $state(0);
  let error = $state<string | null>(null);
  let duplicateMedia = $state<MediaSummary | null>(null);
  let createdMedia = $state<MediaDetail | null>(null);

  // Computed
  const canUpload = $derived(file !== null && fileError === null);
  const isUploading = $derived(
    step === "checking" || step === "uploading" || step === "finalising"
  );

  // Actions
  function setFile(f: File) {
    runGeneration += 1;
    abortController?.abort();
    abortController = null;
    fileError = null;
    file = null;
    fileHash = null;
    step = "select";
    progress = 0;
    error = null;
    duplicateMedia = null;
    createdMedia = null;

    // validateFile returns a result object; it never throws.
    const result = validateFile(f, maxFileSize);
    if (result.valid) {
      file = f;
    } else {
      fileError = result.error ?? "Invalid file";
    }
  }

  function clearFile() {
    runGeneration += 1;
    abortController?.abort();
    abortController = null;
    file = null;
    fileError = null;
    fileHash = null;
    step = "select";
    progress = 0;
    error = null;
    duplicateMedia = null;
  }

  function reset() {
    clearFile();
    createdMedia = null;
  }

  function isUploadActive(): boolean {
    return step === "checking" || step === "uploading" || step === "finalising";
  }

  async function startUpload(metadata?: Partial<CreateMediaRequest>) {
    if (!file || isUploadActive()) return;

    const generation = ++runGeneration;
    const targetFile = file;
    step = "checking";
    error = null;

    try {
      // Step 1: Compute file hash
      const computedHash = await computeFileHash(targetFile);
      if (generation !== runGeneration) return;
      fileHash = computedHash;

      // Step 2: Check for duplicates
      // For replace flow, first check against existing versions
      if (existingVersionHashes.includes(fileHash)) {
        throw new Error("This file has already been uploaded as a version of this media item.");
      }

      // Check global duplicates (for new upload flow)
      if (!existingMediaId) {
        const duplicateCheck = await checkDuplicate(fileHash);
        if (generation !== runGeneration) return;

        if (duplicateCheck.exists && duplicateCheck.media) {
          duplicateMedia = duplicateCheck.media;
          step = "duplicate";
          return;
        }
      }

      // No duplicate - proceed with upload
      await proceedWithUploadInternal(metadata, generation);
    } catch (e) {
      if (generation !== runGeneration) return;
      console.error("Upload failed", e);
      error = e instanceof Error ? e.message : "Upload failed";
      step = "error";
      onError?.(e instanceof Error ? e : new Error(String(e)));
    }
  }

  async function proceedWithUpload(metadata?: Partial<CreateMediaRequest>) {
    if (!file || !fileHash || isUploadActive()) return;
    const generation = ++runGeneration;
    await proceedWithUploadInternal(metadata, generation);
  }

  async function proceedWithUploadInternal(
    metadata: Partial<CreateMediaRequest> | undefined,
    generation: number
  ) {
    if (!file || !fileHash) return;
    const targetFile = file;
    const targetHash = fileHash;

    try {
      step = "uploading";
      progress = 0;

      let mediaId: string;

      if (existingMediaId) {
        // Replace flow - use existing media ID
        mediaId = existingMediaId;
      } else {
        // New upload flow - create media item first
        const media = await createMedia({
          kind: detectMediaKindFromMimeType(targetFile.type),
          visibility: defaultVisibility,
          title: metadata?.title ?? targetFile.name.replace(/\.[^/.]+$/, ""),
          originalFilename: metadata?.originalFilename ?? targetFile.name,
          ...metadata,
        });
        if (generation !== runGeneration) return;
        mediaId = media.id;
      }

      // Initiate upload
      const uploadResponse = await initiateUpload(mediaId, {
        contentType: targetFile.type,
        contentLength: targetFile.size,
        sha256: targetHash,
      });
      if (generation !== runGeneration) return;

      // Upload file to blob storage
      const plan: UploadPlan = {
        uploadUrl: uploadResponse.uploadPlan.uploadUrl,
        method: uploadResponse.uploadPlan.method,
        requiredHeaders: uploadResponse.uploadPlan.headers,
        expiresAt: uploadResponse.uploadPlan.expiresAt,
        maxBytes: uploadResponse.uploadPlan.maxBytes || maxFileSize,
        allowedContentTypes: uploadResponse.uploadPlan.allowedContentTypes || [],
        objectKey: "",
      };

      const controller = new AbortController();
      abortController = controller;
      try {
        await uploadToBlob(plan, targetFile, {
          onProgress: (p) => {
            if (generation === runGeneration) {
              progress = p.percent;
            }
          },
          signal: controller.signal,
        });
      } finally {
        if (abortController === controller) {
          abortController = null;
        }
      }
      if (generation !== runGeneration) return;

      // Finalise upload
      step = "finalising";
      const finaliseResponse = await finaliseUpload(mediaId, uploadResponse.versionId, {
        sha256: targetHash,
        contentType: targetFile.type,
      });
      if (generation !== runGeneration) return;

      // Done!
      createdMedia = finaliseResponse.media;
      step = "complete";
      onComplete?.(finaliseResponse.media);
    } catch (e) {
      if (generation !== runGeneration) return;

      console.error("Upload failed", e);
      error = e instanceof Error ? e.message : "Upload failed";
      step = "error";
      onError?.(e instanceof Error ? e : new Error(String(e)));
    }
  }

  function cancelUpload() {
    runGeneration += 1;
    abortController?.abort();
    abortController = null;
    if (isUploadActive()) {
      step = "select";
      progress = 0;
    }
  }

  function useDuplicate() {
    // This is a signal to the UI that the user wants to use the duplicate
    // The UI should handle navigation to the duplicate media
    // We just reset the flow state
    if (duplicateMedia) {
      // Keep duplicateMedia available for the UI to use
      step = "select";
    }
  }

  return {
    get step() {
      return step;
    },
    get file() {
      return file;
    },
    get fileError() {
      return fileError;
    },
    get fileHash() {
      return fileHash;
    },
    get progress() {
      return progress;
    },
    get error() {
      return error;
    },
    get duplicateMedia() {
      return duplicateMedia;
    },
    get createdMedia() {
      return createdMedia;
    },
    get canUpload() {
      return canUpload;
    },
    get isUploading() {
      return isUploading;
    },
    setFile,
    clearFile,
    startUpload,
    proceedWithUpload,
    cancelUpload,
    useDuplicate,
    reset,
  };
}
