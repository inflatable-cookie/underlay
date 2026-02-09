import { computeFileHash, type MediaSummary } from "../../patterns/index.js";
import { uploadNewMedia } from "./upload";
import type {
  CheckDuplicateResponse,
  CreateMediaRequest,
  FinaliseUploadRequest,
  FinaliseUploadResponse,
  InitiateUploadRequest,
  InitiateUploadResponse,
  MediaDetail
} from "../../patterns/index.js";

interface UploadFlowCallbacks {
  checkDuplicate: (sha256: string) => Promise<CheckDuplicateResponse>;
  createMedia: (request: CreateMediaRequest) => Promise<MediaDetail>;
  initiateUpload: (
    mediaId: string,
    request: InitiateUploadRequest
  ) => Promise<InitiateUploadResponse>;
  finaliseUpload: (
    mediaId: string,
    versionId: string,
    request: FinaliseUploadRequest
  ) => Promise<FinaliseUploadResponse>;
}

interface UploadProcessingCallbacks {
  createMedia: (request: CreateMediaRequest) => Promise<MediaDetail>;
  initiateUpload: (
    mediaId: string,
    request: InitiateUploadRequest
  ) => Promise<InitiateUploadResponse>;
  finaliseUpload: (
    mediaId: string,
    versionId: string,
    request: FinaliseUploadRequest
  ) => Promise<FinaliseUploadResponse>;
}

interface UploadMediaWithKnownHashInput extends UploadProcessingCallbacks {
  file: File;
  fileHash: string;
  maxFileSize: number;
  onStep: (stage: "uploading" | "finalising") => void;
  onProgress: (percent: number) => void;
}

interface RunUploadFlowInput extends UploadFlowCallbacks {
  file: File;
  maxFileSize: number;
  onStep: (stage: "checking" | "uploading" | "finalising") => void;
  onProgress: (percent: number) => void;
}

export interface UploadFlowDuplicateResult {
  kind: "duplicate";
  fileHash: string;
  duplicateMedia: MediaSummary;
}

export interface UploadFlowUploadedResult {
  kind: "uploaded";
  fileHash: string;
  createdMedia: MediaSummary;
}

export type UploadFlowResult = UploadFlowDuplicateResult | UploadFlowUploadedResult;

export async function uploadMediaWithKnownHash({
  file,
  fileHash,
  maxFileSize,
  createMedia,
  initiateUpload,
  finaliseUpload,
  onStep,
  onProgress
}: UploadMediaWithKnownHashInput): Promise<MediaSummary> {
  return uploadNewMedia({
    file,
    fileHash,
    maxFileSize,
    createMedia,
    initiateUpload,
    finaliseUpload,
    onStage: onStep,
    onProgress
  });
}

export async function runUploadFlow({
  file,
  maxFileSize,
  checkDuplicate,
  createMedia,
  initiateUpload,
  finaliseUpload,
  onStep,
  onProgress
}: RunUploadFlowInput): Promise<UploadFlowResult> {
  onStep("checking");

  const fileHash = await computeFileHash(file);
  const duplicateCheck = await checkDuplicate(fileHash);

  if (duplicateCheck.exists && duplicateCheck.media) {
    return {
      kind: "duplicate",
      fileHash,
      duplicateMedia: duplicateCheck.media
    };
  }

  const createdMedia = await uploadMediaWithKnownHash({
    file,
    fileHash,
    maxFileSize,
    createMedia,
    initiateUpload,
    finaliseUpload,
    onStep,
    onProgress
  });

  return {
    kind: "uploaded",
    fileHash,
    createdMedia
  };
}
