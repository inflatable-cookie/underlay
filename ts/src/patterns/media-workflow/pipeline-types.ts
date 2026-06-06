import type { UploadProgress } from "../blob-types";
import type { MediaUploadInitResult } from "./workflow-types";

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

export type MediaUploadPipelineContextOptions<TContext extends object> =
  TContext & {
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
  TContext extends object,
> = {
  detectKind: (mimeType: string) => TKind;
  createMedia: (
    request: {
      kind: TKind;
      visibility: MediaUploadPipelineVisibility;
      originalFilename: string;
      title: string | null;
    },
    context: TContext,
  ) => Promise<TMediaRecord>;
  initiateUpload: (
    mediaId: string,
    request: {
      contentType: string;
      contentLength: number;
      sha256?: string;
    },
    context: TContext,
  ) => Promise<MediaUploadInitResult>;
  finaliseUpload: (
    mediaId: string,
    versionId: string,
    request: {
      sha256: string;
      contentType: string;
    },
    context: TContext,
  ) => Promise<unknown>;
  checkDuplicate: (
    request: {
      sha256: string;
    },
    context: TContext,
  ) => Promise<{
    exists: boolean;
    media?: TExisting | null;
  }>;
  maxFileSize?: number;
  includeHashInInitiate?: boolean;
};

export type MediaCreateUploadOptions<
  TKind,
  TMediaRecord extends { id: string },
> = {
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
    },
  ) => Promise<MediaUploadInitResult>;
  finaliseUpload: (
    mediaId: string,
    versionId: string,
    request: {
      sha256: string;
      contentType: string;
    },
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
    },
  ) => Promise<MediaUploadInitResult>;
  finaliseUpload: (
    mediaId: string,
    versionId: string,
    request: {
      sha256: string;
      contentType: string;
    },
  ) => Promise<unknown>;
};

export type CheckMediaDuplicateFileOptions<TMedia> = {
  file: File;
  checkDuplicate: (request: { sha256: string }) => Promise<{
    exists: boolean;
    media?: TMedia | null;
  }>;
};
