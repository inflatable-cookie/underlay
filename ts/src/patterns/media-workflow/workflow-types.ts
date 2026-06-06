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

export type UploadMediaWithKnownHashInput<
  TCreated,
  TCreateRequest,
  TCreatedRecord,
  TInitiateRequest,
  TFinaliseRequest,
  TFinaliseResult,
> = {
  file: File;
  fileHash: string;
  maxFileSize: number;
  createRecord: (request: TCreateRequest) => Promise<TCreatedRecord>;
  buildCreateRequest: (file: File, fileHash: string) => TCreateRequest;
  initiateUpload: (
    createdRecord: TCreatedRecord,
    request: TInitiateRequest,
  ) => Promise<MediaUploadInitResult>;
  buildInitiateRequest: (file: File, fileHash: string) => TInitiateRequest;
  finaliseUpload: (
    createdRecord: TCreatedRecord,
    versionId: string,
    request: TFinaliseRequest,
  ) => Promise<TFinaliseResult>;
  buildFinaliseRequest: (file: File, fileHash: string) => TFinaliseRequest;
  toCreatedItem: (
    finaliseResult: TFinaliseResult,
    createdRecord: TCreatedRecord,
  ) => TCreated;
  onStep?: (step: "uploading" | "finalising") => void;
  onProgress?: (percent: number) => void;
};

export type RunMediaUploadWorkflowInput<
  TExisting,
  TCreated,
  TCreateRequest,
  TCreatedRecord,
  TInitiateRequest,
  TFinaliseRequest,
  TFinaliseResult,
> = UploadMediaWithKnownHashInput<
  TCreated,
  TCreateRequest,
  TCreatedRecord,
  TInitiateRequest,
  TFinaliseRequest,
  TFinaliseResult
> & {
  checkDuplicate: (
    sha256: string,
  ) => Promise<MediaDuplicateCheckResult<TExisting>>;
  onStep?: (step: "checking" | "uploading" | "finalising") => void;
};
