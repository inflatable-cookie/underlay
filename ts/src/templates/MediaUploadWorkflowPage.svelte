<script lang="ts">
  import {
    ALLOWED_MEDIA_TYPES,
    REJECTED_VIDEO_TYPES,
    validateFileType
  } from "../runtime/media";
  import { Button, FileUpload, Icon, IconButton, Progress, formatFileSize } from "@poodle/svelte";
  import type { FileUploadItem } from "@poodle/svelte";
  import { default as MediaUploadPage } from "./MediaUploadPage.svelte";

  type UploadStatus =
    | "pending"
    | "hashing"
    | "checking"
    | "creating"
    | "uploading"
    | "finalising"
    | "done"
    | "error"
    | "duplicate";

  interface DuplicateMediaSummary {
    id: string;
    title?: string | null;
    originalFilename?: string | null;
  }

  interface DuplicateCheckResult {
    hash: string;
    exists: boolean;
    media?: DuplicateMediaSummary | null;
  }

  interface UploadQueueItem {
    id: string;
    file: File;
    status: UploadStatus;
    progress: number;
    title: string;
    hash?: string;
    mediaId?: string;
    error?: string;
    duplicateOf?: DuplicateMediaSummary;
  }

  interface UploadHandlerInput {
    file: File;
    title?: string | null;
    onProgress?: (progress: { percent: number }) => void;
    signal?: AbortSignal;
  }

  interface ReplaceHandlerInput {
    file: File;
    mediaId: string;
    onProgress?: (progress: { percent: number }) => void;
    signal?: AbortSignal;
  }

  interface UploadResult {
    mediaId: string;
    hash?: string;
  }

  interface Props {
    replaceMediaId?: string | null;
    maxFileSize?: number;
    accept?: string;
    title?: string;
    replaceTitle?: string;
    backHref?: string;
    replaceBackHref?: string;
    backLabel?: string;
    replaceBackLabel?: string;
    createUpload: (input: UploadHandlerInput) => Promise<UploadResult>;
    replaceUpload: (input: ReplaceHandlerInput) => Promise<UploadResult | void>;
    checkDuplicate?: (file: File) => Promise<DuplicateCheckResult>;
    navigate?: (href: string) => unknown;
    onToast?: (variant: "success" | "info" | "error", message: string) => void;
  }

  let {
    replaceMediaId = null,
    maxFileSize = 50 * 1024 * 1024,
    accept = ALLOWED_MEDIA_TYPES.join(","),
    title = "Upload Media",
    replaceTitle = "Replace File",
    backHref = "/media",
    replaceBackHref = undefined,
    backLabel = "Back to library",
    replaceBackLabel = "Back to media",
    createUpload,
    replaceUpload,
    checkDuplicate,
    navigate = (href) => {
      globalThis.location.assign(href);
    },
    onToast
  }: Props = $props();

  let files = $state<FileUploadItem[]>([]);
  let uploadQueue = $state<UploadQueueItem[]>([]);
  let uploading = $state(false);
  let uploadAbort: AbortController | null = null;
  let error = $state<string | null>(null);
  let singleUploadStage = $state<UploadStatus>("pending");
  let singleUploadProgress = $state(0);

  const isBulkMode = $derived(!replaceMediaId);
  const pendingCount = $derived(uploadQueue.filter((item) => item.status === "pending").length);
  const completedCount = $derived(uploadQueue.filter((item) => item.status === "done").length);
  const errorCount = $derived(uploadQueue.filter((item) => item.status === "error").length);
  const allDone = $derived(
    uploadQueue.length > 0 &&
      uploadQueue.every((item) => item.status === "done" || item.status === "error" || item.status === "duplicate")
  );

  $effect(() => {
    if (isBulkMode && files.length > 0) {
      addFilesToQueue(files);
      files = [];
    }
  });

  $effect(() => {
    if (!isBulkMode && files.length > 1) {
      files = files.slice(0, 1);
    }
  });

  function addFilesToQueue(selectedFiles: FileUploadItem[]): void {
    const validationErrors: string[] = [];
    const nextQueue = [...uploadQueue];

    for (const fileItem of selectedFiles) {
      const file = fileItem.file;
      const validationError = validateMediaFile(file);
      if (validationError) {
        validationErrors.push(`${file.name}: ${validationError}`);
        continue;
      }

      const alreadyQueued = nextQueue.some(
        (item) => item.file.name === file.name && item.file.size === file.size
      );
      if (alreadyQueued) {
        continue;
      }

      nextQueue.push({
        id: crypto.randomUUID(),
        file,
        status: "pending",
        progress: 0,
        title: file.name.replace(/\.[^/.]+$/, "").replace(/[-_]/g, " ")
      });
    }

    uploadQueue = nextQueue;
    error = validationErrors.length > 0 ? validationErrors.join("\n") : null;
  }

  function validateMediaFile(file: File): string | null {
    if (REJECTED_VIDEO_TYPES.includes(file.type as (typeof REJECTED_VIDEO_TYPES)[number])) {
      return "Video uploads are not supported";
    }

    if (!validateFileType(file, ALLOWED_MEDIA_TYPES)) {
      return "Unsupported file type";
    }

    if (file.size > maxFileSize) {
      return `File too large (${formatFileSize(file.size)})`;
    }

    return null;
  }

  function updateQueueItem(itemId: string, updates: Partial<UploadQueueItem>): void {
    uploadQueue = uploadQueue.map((item) =>
      item.id === itemId ? { ...item, ...updates } : item
    );
  }

  function removeFromQueue(itemId: string): void {
    uploadQueue = uploadQueue.filter((item) => item.id !== itemId);
  }

  function clearQueue(): void {
    uploadQueue = [];
    error = null;
  }

  async function processQueueItem(item: UploadQueueItem, skipDuplicateCheck = false): Promise<void> {
    try {
      if (!skipDuplicateCheck && checkDuplicate) {
        updateQueueItem(item.id, { status: "hashing" });
        const duplicate = await checkDuplicate(item.file);
        updateQueueItem(item.id, { status: "checking", hash: duplicate.hash });

        if (duplicate.exists && duplicate.media) {
          updateQueueItem(item.id, {
            status: "duplicate",
            duplicateOf: duplicate.media
          });
          return;
        }
      }

      updateQueueItem(item.id, { status: "creating", duplicateOf: undefined });
      const result = await createUpload({
        file: item.file,
        title: item.title.trim() || null,
        signal: uploadAbort?.signal,
        onProgress: (progress) => {
          updateQueueItem(item.id, {
            status: "uploading",
            progress: progress.percent
          });
        }
      });

      updateQueueItem(item.id, {
        status: "done",
        progress: 100,
        mediaId: result.mediaId,
        hash: result.hash ?? item.hash
      });
    } catch (uploadError) {
      updateQueueItem(item.id, {
        status: "error",
        error: uploadError instanceof Error ? uploadError.message : "Upload failed"
      });
    }
  }

  async function handleUpload(): Promise<void> {
    if (uploadQueue.length === 0) {
      error = "Please select files to upload";
      return;
    }

    uploading = true;
    uploadAbort = new AbortController();
    error = null;

    for (const item of uploadQueue) {
      if (uploadAbort.signal.aborted) {
        break;
      }
      if (item.status === "pending") {
        await processQueueItem(item);
      }
    }

    uploadAbort = null;
    uploading = false;

    if (completedCount > 0 && errorCount === 0) {
      onToast?.("success", `${completedCount} file${completedCount === 1 ? "" : "s"} uploaded`);
    } else if (completedCount > 0 && errorCount > 0) {
      onToast?.("info", `${completedCount} uploaded, ${errorCount} failed`);
    }
  }

  async function retryItem(itemId: string): Promise<void> {
    const item = uploadQueue.find((entry) => entry.id === itemId);
    if (!item) return;

    updateQueueItem(itemId, {
      status: "pending",
      progress: 0,
      error: undefined
    });

    uploading = true;
    await processQueueItem(item);
    uploading = false;
  }

  async function uploadDuplicateAnyway(itemId: string): Promise<void> {
    const item = uploadQueue.find((entry) => entry.id === itemId);
    if (!item) return;

    uploading = true;
    await processQueueItem(item, true);
    uploading = false;
  }

  async function handleSingleUpload(): Promise<void> {
    const file = files[0]?.file;
    if (!file || !replaceMediaId) return;

    const validationError = validateMediaFile(file);
    if (validationError) {
      error = validationError;
      files = [];
      return;
    }

    uploading = true;
    error = null;
    singleUploadProgress = 0;
    singleUploadStage = "hashing";

    uploadAbort = new AbortController();

    try {
      await replaceUpload({
        file,
        mediaId: replaceMediaId,
        signal: uploadAbort.signal,
        onProgress: (progress) => {
          singleUploadStage = "uploading";
          singleUploadProgress = progress.percent;
        }
      });

      singleUploadStage = "done";
      onToast?.("success", "File replaced");
      navigate(`/media/${replaceMediaId}`);
    } catch (uploadError) {
      error = uploadError instanceof Error ? uploadError.message : "Failed to upload media";
      onToast?.("error", error);
      singleUploadStage = "pending";
    } finally {
      uploadAbort = null;
      uploading = false;
    }
  }

  function cancelUpload(fallbackHref: string): void {
    if (uploading && uploadAbort) {
      // Abort mid-flight; the workflow settles via its error/abort path.
      uploadAbort.abort();
      return;
    }
    navigate(fallbackHref);
  }

  function viewMedia(mediaId: string | undefined): void {
    if (mediaId) {
      navigate(`/media/${mediaId}`);
    }
  }

  function getStatusLabel(status: UploadStatus): string {
    switch (status) {
      case "pending":
        return "Waiting";
      case "hashing":
        return "Hashing...";
      case "checking":
        return "Checking...";
      case "creating":
        return "Creating...";
      case "uploading":
        return "Uploading";
      case "finalising":
        return "Finalising...";
      case "done":
        return "Complete";
      case "error":
        return "Failed";
      case "duplicate":
        return "Duplicate";
      default:
        return "";
    }
  }

  function getStatusIcon(status: UploadStatus): string {
    if (status === "done") return "circle-check";
    if (status === "error") return "circle-x";
    if (status === "duplicate") return "triangle-alert";
    return "file";
  }
</script>

<MediaUploadPage
  title={replaceMediaId ? replaceTitle : title}
  backHref={replaceMediaId ? replaceBackHref ?? `/media/${replaceMediaId}` : backHref}
  backLabel={replaceMediaId ? replaceBackLabel : backLabel}
  {error}
  errorTitle="Could not upload media"
>
  <div class="underlay-media-upload-workflow">
    {#if replaceMediaId}
      {#if uploading}
        <section class="underlay-media-upload-workflow__panel">
          <div class="underlay-media-upload-workflow__progress-header">
            <span>
              {singleUploadStage === "hashing" ? "Computing file hash..." :
               singleUploadStage === "uploading" ? "Uploading file..." :
               singleUploadStage === "done" ? "Done!" : "Preparing upload..."}
            </span>
            {#if singleUploadStage === "uploading"}
              <span>{Math.round(singleUploadProgress)}%</span>
            {/if}
          </div>
          <Progress
            value={singleUploadStage === "uploading" ? singleUploadProgress : singleUploadStage === "done" ? 100 : 0}
            max={100}
            ariaLabel="Replace upload progress"
          />
        </section>
      {:else}
        <section class="underlay-media-upload-workflow__panel">
          <FileUpload
            bind:files
            {accept}
            maxSize={maxFileSize}
            disabled={uploading}
            multiple={false}
            maxFiles={1}
          />
          <p class="underlay-media-upload-workflow__hint">
            Supported formats: JPEG, PNG, GIF, WebP, AVIF, PDF. Maximum size: {formatFileSize(maxFileSize)}.
          </p>
        </section>
      {/if}

      <div class="underlay-media-upload-workflow__actions">
        <Button type="button" variant="ghost" onClick={() => cancelUpload(`/media/${replaceMediaId}`)}>
          Cancel
        </Button>
        <Button
          type="button"
          variant="primary"
          disabled={uploading || files.length === 0 || !!error}
          onClick={handleSingleUpload}
        >
          {uploading ? "Uploading..." : "Replace File"}
        </Button>
      </div>
    {:else}
      <section class="underlay-media-upload-workflow__panel">
        <FileUpload
          bind:files
          {accept}
          maxSize={maxFileSize}
          disabled={uploading}
          multiple
        />
        <p class="underlay-media-upload-workflow__hint">
          Supported formats: JPEG, PNG, GIF, WebP, SVG, PDF. Maximum size: {formatFileSize(maxFileSize)} per file.
          Select multiple files to upload them all at once.
        </p>
      </section>

      {#if uploadQueue.length > 0}
        <section class="underlay-media-upload-workflow__panel">
          <div class="underlay-media-upload-workflow__queue-header">
            <h2>Upload Queue ({uploadQueue.length} file{uploadQueue.length === 1 ? "" : "s"})</h2>
            {#if !uploading && !allDone}
              <Button type="button" variant="ghost" size="sm" onClick={clearQueue}>
                Clear All
              </Button>
            {/if}
          </div>

          <div class="underlay-media-upload-workflow__queue">
            {#each uploadQueue as item (item.id)}
              <article
                class="underlay-media-upload-workflow__queue-item"
                data-status={item.status}
              >
                <span class="underlay-media-upload-workflow__queue-icon">
                  <Icon icon={getStatusIcon(item.status)} />
                </span>

                <div class="underlay-media-upload-workflow__queue-body">
                  <strong>{item.file.name}</strong>
                  <span>
                    {formatFileSize(item.file.size)}
                    {#if item.status !== "pending" && item.status !== "done" && item.status !== "error" && item.status !== "duplicate"}
                      · {getStatusLabel(item.status)}
                    {/if}
                    {#if item.status === "error" && item.error}
                      · {item.error}
                    {/if}
                    {#if item.status === "duplicate" && item.duplicateOf}
                      · Duplicate of "{item.duplicateOf.title ?? item.duplicateOf.originalFilename ?? "Untitled"}"
                    {/if}
                  </span>
                  {#if item.status === "uploading"}
                    <Progress value={item.progress} max={100} size="sm" ariaLabel={`Upload progress for ${item.file.name}`} />
                  {/if}
                </div>

                <div class="underlay-media-upload-workflow__queue-actions">
                  {#if item.status === "done"}
                    <Button type="button" variant="ghost" size="sm" onClick={() => viewMedia(item.mediaId)}>
                      View
                    </Button>
                  {:else if item.status === "error"}
                    <Button type="button" variant="ghost" size="sm" disabled={uploading} onClick={() => retryItem(item.id)}>
                      Retry
                    </Button>
                  {:else if item.status === "duplicate"}
                    <Button type="button" variant="ghost" size="sm" onClick={() => viewMedia(item.duplicateOf?.id)}>
                      View Existing
                    </Button>
                    <Button type="button" variant="ghost" size="sm" disabled={uploading} onClick={() => uploadDuplicateAnyway(item.id)}>
                      Upload Anyway
                    </Button>
                  {:else if item.status === "pending"}
                    <IconButton
                      type="button"
                      variant="ghost"
                      tone="danger"
                      size="sm"
                      icon="x"
                      ariaLabel="Remove from queue"
                      tooltip="Remove from queue"
                      onClick={() => removeFromQueue(item.id)}
                    />
                  {/if}
                </div>
              </article>
            {/each}
          </div>
        </section>
      {/if}

      <div class="underlay-media-upload-workflow__actions">
        <Button type="button" variant="ghost" onClick={() => cancelUpload("/media")}>
          Cancel
        </Button>
        {#if allDone}
          <Button type="button" variant="primary" onClick={() => navigate("/media")}>
            Done
          </Button>
        {:else}
          <Button
            type="button"
            variant="primary"
            disabled={uploading || uploadQueue.length === 0 || pendingCount === 0}
            onClick={handleUpload}
          >
            {uploading ? "Uploading..." : `Upload ${pendingCount || uploadQueue.length} file${(pendingCount || uploadQueue.length) === 1 ? "" : "s"}`}
          </Button>
        {/if}
      </div>
    {/if}
  </div>
</MediaUploadPage>

<style>
  .underlay-media-upload-workflow {
    display: grid;
    gap: var(--poodle-space-stack-md);
  }

  .underlay-media-upload-workflow__panel {
    display: grid;
    gap: var(--poodle-space-stack-sm);
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 92%, transparent);
  }

  .underlay-media-upload-workflow__hint {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-body-sm-size);
    line-height: 1.5;
  }

  .underlay-media-upload-workflow__actions {
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-end;
    gap: var(--poodle-space-inline-sm);
  }

  .underlay-media-upload-workflow__progress-header,
  .underlay-media-upload-workflow__queue-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--poodle-space-inline-md);
  }

  .underlay-media-upload-workflow__queue-header h2 {
    margin: 0;
    font-size: var(--poodle-typography-heading-4-size);
  }

  .underlay-media-upload-workflow__queue {
    display: grid;
    gap: var(--poodle-space-stack-xs);
  }

  .underlay-media-upload-workflow__queue-item {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr) auto;
    align-items: start;
    gap: var(--poodle-space-inline-md);
    padding: var(--poodle-space-stack-sm);
    border-radius: var(--poodle-radius-control);
    background: color-mix(in srgb, var(--poodle-color-background-base) 70%, transparent);
  }

  .underlay-media-upload-workflow__queue-item[data-status="done"] {
    background: color-mix(in srgb, var(--poodle-color-success-base) 12%, transparent);
  }

  .underlay-media-upload-workflow__queue-item[data-status="error"] {
    background: color-mix(in srgb, var(--poodle-color-danger-base) 12%, transparent);
  }

  .underlay-media-upload-workflow__queue-item[data-status="duplicate"] {
    background: color-mix(in srgb, var(--poodle-color-warning-base) 12%, transparent);
  }

  .underlay-media-upload-workflow__queue-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: var(--poodle-color-text-secondary);
  }

  .underlay-media-upload-workflow__queue-body {
    display: grid;
    gap: 0.25rem;
    min-width: 0;
  }

  .underlay-media-upload-workflow__queue-body strong,
  .underlay-media-upload-workflow__queue-body span {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .underlay-media-upload-workflow__queue-body span {
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-body-sm-size);
  }

  .underlay-media-upload-workflow__queue-actions {
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-end;
    gap: var(--poodle-space-inline-xs);
  }

  @media (max-width: 640px) {
    .underlay-media-upload-workflow__queue-item {
      grid-template-columns: auto minmax(0, 1fr);
    }

    .underlay-media-upload-workflow__queue-actions {
      grid-column: 1 / -1;
      justify-content: flex-start;
    }
  }
</style>
