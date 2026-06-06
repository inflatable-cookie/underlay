import { computeFileHash, uploadToBlob } from "../blob-upload";
import { toUploadPlan } from "./plan";
import type {
  MediaUploadWorkflowResult,
  RunMediaUploadWorkflowInput,
  UploadMediaWithKnownHashInput,
} from "./types";

export async function uploadMediaWithKnownHash<
  TCreated,
  TCreateRequest,
  TCreatedRecord,
  TInitiateRequest,
  TFinaliseRequest,
  TFinaliseResult,
>(
  input: UploadMediaWithKnownHashInput<
    TCreated,
    TCreateRequest,
    TCreatedRecord,
    TInitiateRequest,
    TFinaliseRequest,
    TFinaliseResult
  >,
): Promise<TCreated> {
  const createdRecord = await input.createRecord(
    input.buildCreateRequest(input.file, input.fileHash),
  );

  const uploadStart = await input.initiateUpload(
    createdRecord,
    input.buildInitiateRequest(input.file, input.fileHash),
  );

  input.onStep?.("uploading");
  await uploadToBlob(
    toUploadPlan(uploadStart.uploadPlan, input.maxFileSize),
    input.file,
    {
      onProgress: input.onProgress
        ? (progress) => {
            input.onProgress?.(progress.percent);
          }
        : undefined,
    },
  );

  input.onStep?.("finalising");
  const finaliseResult = await input.finaliseUpload(
    createdRecord,
    uploadStart.versionId,
    input.buildFinaliseRequest(input.file, input.fileHash),
  );

  return input.toCreatedItem(finaliseResult, createdRecord);
}

export async function runMediaUploadWorkflow<
  TExisting,
  TCreated,
  TCreateRequest,
  TCreatedRecord,
  TInitiateRequest,
  TFinaliseRequest,
  TFinaliseResult,
>(
  input: RunMediaUploadWorkflowInput<
    TExisting,
    TCreated,
    TCreateRequest,
    TCreatedRecord,
    TInitiateRequest,
    TFinaliseRequest,
    TFinaliseResult
  >,
): Promise<MediaUploadWorkflowResult<TExisting, TCreated>> {
  input.onStep?.("checking");

  const fileHash = await computeFileHash(input.file);
  const duplicateCheck = await input.checkDuplicate(fileHash);

  if (duplicateCheck.exists && duplicateCheck.item) {
    return {
      kind: "duplicate",
      fileHash,
      existingItem: duplicateCheck.item,
    };
  }

  const createdItem = await uploadMediaWithKnownHash({
    ...input,
    fileHash,
  });

  return {
    kind: "uploaded",
    fileHash,
    createdItem,
  };
}
