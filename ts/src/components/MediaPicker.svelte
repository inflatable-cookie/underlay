<script lang="ts">
  /**
   * MediaPicker - A modal dialog for selecting media from the library or uploading new media.
   *
   * This is a generic component that accepts callback props for media operations,
   * allowing it to work with any backend implementation.
   *
   * Usage:
   * ```svelte
   * <MediaPicker
   *   bind:open={pickerOpen}
   *   listMediaPaginated={...}
   *   checkDuplicate={...}
   *   createMedia={...}
   *   initiateUpload={...}
   *   finaliseUpload={...}
   *   getToken={...}
   *   onselect={(mediaId, media) => handleSelect(mediaId, media)}
   * />
   * ```
   */
  import {
    uploadToBlob,
    computeFileHash,
    validateFile,
    formatFileSize,
    getFileTypeDescription,
    ALLOWED_MEDIA_TYPES,
    type UploadPlan,
    type PaginatedResponse,
    type PaginationParams,
    MediaKind,
    MediaVisibility,
    getMediaKindLabel,
    getMediaKindAccent,
    getMediaDisplayName,
    type MediaSummary,
    type MediaDetail,
    type CreateMediaRequest,
    type CheckDuplicateResponse,
    type InitiateUploadRequest,
    type InitiateUploadResponse,
    type FinaliseUploadRequest,
    type FinaliseUploadResponse,
  } from "../patterns/index.js";
  import Button from "./Button.svelte";
  import Dialog from "./Dialog.svelte";
  import FormError from "./FormError.svelte";
  import PageLoading from "./PageLoading.svelte";
  import TabsRoot from "./TabsRoot.svelte";
  import TabsList from "./TabsList.svelte";
  import TabsTrigger from "./TabsTrigger.svelte";
  import TabsContent from "./TabsContent.svelte";
  import Image from "lucide-svelte/icons/image";
  import FileText from "lucide-svelte/icons/file-text";
  import Upload from "lucide-svelte/icons/upload";
  import Check from "lucide-svelte/icons/check";
  import AlertCircle from "lucide-svelte/icons/alert-circle";
  import Search from "lucide-svelte/icons/search";

  interface Props {
    /** Whether the picker dialog is open */
    open?: boolean;
    /** Dialog title */
    title?: string;
    /** Filter by media kind */
    filterKind?: MediaKind | null;
    /** Maximum file size in bytes (default 25MB) */
    maxFileSize?: number;

    // =========================================================================
    // Media operation callbacks
    // =========================================================================

    /** List media with pagination */
    listMediaPaginated: (
      params?: PaginationParams
    ) => Promise<PaginatedResponse<MediaSummary>>;

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

    // =========================================================================
    // Events
    // =========================================================================

    /** Callback when media is selected */
    onselect?: (mediaId: string, media: MediaSummary) => void;

    /** Callback when picker is cancelled */
    oncancel?: () => void;
  }

  let {
    open = $bindable(false),
    title = "Select Media",
    filterKind = null,
    maxFileSize = 25 * 1024 * 1024,
    listMediaPaginated,
    checkDuplicate,
    createMedia,
    initiateUpload,
    finaliseUpload,
    onselect,
    oncancel,
  }: Props = $props();

  // Tabs state
  let activeTab = $state("browse");

  // Browse state
  let browseLoading = $state(false);
  let browseError = $state<string | null>(null);
  let browseItems = $state<MediaSummary[]>([]);
  let browseNextCursor = $state<string | null>(null);
  let browseHasMore = $state(false);

  // Upload state
  let selectedFile = $state<File | null>(null);
  let fileError = $state<string | null>(null);
  let uploadStep = $state<
    "select" | "checking" | "duplicate" | "uploading" | "finalising" | "complete" | "error"
  >("select");
  let uploadProgress = $state(0);
  let uploadError = $state<string | null>(null);
  let duplicateMedia = $state<MediaSummary | null>(null);
  let fileHash = $state<string | null>(null);
  let createdMedia = $state<MediaSummary | null>(null);

  // Load initial browse data when dialog opens
  $effect(() => {
    if (open && browseItems.length === 0 && !browseLoading) {
      loadBrowseItems();
    }
  });

  async function loadBrowseItems(cursor?: string) {
    browseLoading = true;
    browseError = null;

    try {
      const response = await listMediaPaginated({
        cursor: cursor ?? undefined,
        limit: 12,
      });
      if (cursor) {
        browseItems = [...browseItems, ...response.data];
      } else {
        browseItems = response.data;
      }
      browseNextCursor = response.nextCursor;
      browseHasMore = response.hasMore;
    } catch (e) {
      browseError = e instanceof Error ? e.message : "Failed to load media";
    } finally {
      browseLoading = false;
    }
  }

  function loadMoreItems() {
    if (browseNextCursor) {
      loadBrowseItems(browseNextCursor);
    }
  }

  function handleFileSelect(event: Event) {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0];
    if (file) {
      validateAndSetFile(file);
    }
  }

  function handleDrop(event: DragEvent) {
    event.preventDefault();
    const file = event.dataTransfer?.files?.[0];
    if (file) {
      validateAndSetFile(file);
    }
  }

  function handleDragOver(event: DragEvent) {
    event.preventDefault();
  }

  function validateAndSetFile(file: File) {
    fileError = null;
    selectedFile = null;

    try {
      validateFile(file, maxFileSize);
      selectedFile = file;
    } catch (e) {
      fileError = e instanceof Error ? e.message : "Invalid file";
    }
  }

  function clearUpload() {
    selectedFile = null;
    fileError = null;
    fileHash = null;
    uploadStep = "select";
    uploadProgress = 0;
    uploadError = null;
    duplicateMedia = null;
    createdMedia = null;
  }

  function getMediaKindFromFile(file: File): MediaKind {
    if (file.type.startsWith("image/")) return MediaKind.Image;
    if (file.type === "application/pdf") return MediaKind.Pdf;
    return MediaKind.Image;
  }

  async function startUpload() {
    if (!selectedFile) return;

    uploadStep = "checking";
    uploadError = null;

    try {
      fileHash = await computeFileHash(selectedFile);

      const duplicateCheck = await checkDuplicate(fileHash);

      if (duplicateCheck.exists && duplicateCheck.media) {
        duplicateMedia = duplicateCheck.media;
        uploadStep = "duplicate";
        return;
      }

      await proceedWithUpload();
    } catch (e) {
      console.error("Upload failed", e);
      uploadError = e instanceof Error ? e.message : "Upload failed";
      uploadStep = "error";
    }
  }

  async function proceedWithUpload() {
    if (!selectedFile || !fileHash) return;

    try {
      uploadStep = "uploading";
      uploadProgress = 0;

      const media = await createMedia({
        kind: getMediaKindFromFile(selectedFile),
        visibility: MediaVisibility.Public,
        title: selectedFile.name.replace(/\.[^/.]+$/, ""),
        originalFilename: selectedFile.name,
      });

      const uploadResponse = await initiateUpload(media.id, {
        contentType: selectedFile.type,
        contentLength: selectedFile.size,
        sha256: fileHash,
      });

      const plan: UploadPlan = {
        uploadUrl: uploadResponse.uploadPlan.uploadUrl,
        method: uploadResponse.uploadPlan.method,
        requiredHeaders: uploadResponse.uploadPlan.headers,
        expiresAt: uploadResponse.uploadPlan.expiresAt,
        maxBytes: uploadResponse.uploadPlan.maxBytes || maxFileSize,
        allowedContentTypes: uploadResponse.uploadPlan.allowedContentTypes || [],
        objectKey: "",
      };

      await uploadToBlob(plan, selectedFile, {
        onProgress: (progress) => {
          uploadProgress = progress.percent;
        },
      });

      uploadStep = "finalising";
      await finaliseUpload(media.id, uploadResponse.versionId, {
        sha256: fileHash,
        contentType: selectedFile.type,
      });

      // Create a summary for selection
      createdMedia = {
        id: media.id,
        kind: media.kind,
        visibility: media.visibility,
        title: media.title,
        originalFilename: media.originalFilename,
        currentVersionId: media.currentVersionId,
        createdAt: media.createdAt,
        updatedAt: media.updatedAt,
        deletedAt: null,
        byteSize: null,
        mimeType: null,
        thumbnailUrl: null, // Thumbnail not yet generated
      };

      uploadStep = "complete";
    } catch (e) {
      console.error("Upload failed", e);
      uploadError = e instanceof Error ? e.message : "Upload failed";
      uploadStep = "error";
    }
  }

  function selectDuplicate() {
    if (duplicateMedia) {
      selectMedia(duplicateMedia);
    }
  }

  async function uploadAnyway() {
    duplicateMedia = null;
    await proceedWithUpload();
  }

  function selectUploaded() {
    if (createdMedia) {
      selectMedia(createdMedia);
    }
  }

  function selectMedia(media: MediaSummary) {
    onselect?.(media.id, media);
    closeAndReset();
  }

  function handleCancel() {
    oncancel?.();
    closeAndReset();
  }

  function closeAndReset() {
    open = false;
    activeTab = "browse";
    clearUpload();
    // Reset browse state for next open
    browseItems = [];
    browseNextCursor = null;
    browseHasMore = false;
  }

  function getMediaIcon(kind: MediaKind) {
    return kind === MediaKind.Image ? Image : FileText;
  }
</script>

<Dialog
  bind:open
  {title}
  showTrigger={false}
  contentClassName="media-picker-dialog"
>
  <TabsRoot bind:value={activeTab}>
    <TabsList>
      <TabsTrigger value="browse">
        <Search size={14} />
        Browse Library
      </TabsTrigger>
      <TabsTrigger value="upload">
        <Upload size={14} />
        Upload New
      </TabsTrigger>
    </TabsList>

    <TabsContent value="browse">
      <div class="browse-content">
        {#if browseLoading && browseItems.length === 0}
          <PageLoading message="Loading media..." />
        {:else if browseError}
          <FormError message={browseError} />
        {:else if browseItems.length === 0}
          <div class="empty-state">
            <Image size={32} />
            <p>No media found</p>
          </div>
        {:else}
          <div class="media-grid">
            {#each browseItems as item}
              {@const Icon = getMediaIcon(item.kind)}
              <button
                type="button"
                class="media-item"
                onclick={() => selectMedia(item)}
              >
                <div
                  class="media-item__icon"
                  style="color: {getMediaKindAccent(item.kind)}"
                >
                  <Icon size={24} />
                </div>
                <div class="media-item__info">
                  <span class="media-item__title"
                    >{getMediaDisplayName(item)}</span
                  >
                  <span class="media-item__meta"
                    >{getMediaKindLabel(item.kind)}</span
                  >
                </div>
              </button>
            {/each}
          </div>

          {#if browseHasMore}
            <div class="load-more">
              <Button
                variant="secondary"
                onclick={loadMoreItems}
                disabled={browseLoading}
              >
                {browseLoading ? "Loading..." : "Load more"}
              </Button>
            </div>
          {/if}
        {/if}
      </div>
    </TabsContent>

    <TabsContent value="upload">
      <div class="upload-content">
        {#if uploadStep === "select"}
          <div
            class="dropzone"
            class:dropzone--has-file={selectedFile}
            class:dropzone--has-error={fileError}
            ondrop={handleDrop}
            ondragover={handleDragOver}
            role="button"
            tabindex="0"
          >
            {#if selectedFile}
              <div class="selected-file">
                {#if selectedFile.type.startsWith("image/")}
                  <Image size={32} />
                {:else}
                  <FileText size={32} />
                {/if}
                <div class="selected-file__info">
                  <span class="selected-file__name">{selectedFile.name}</span>
                  <span class="selected-file__meta">
                    {getFileTypeDescription(selectedFile.type)} &middot; {formatFileSize(
                      selectedFile.size
                    )}
                  </span>
                </div>
              </div>
            {:else}
              <Upload size={32} />
              <p>Drop file here or click to browse</p>
              <p class="dropzone__hint">Images and PDFs up to 25MB</p>
              <input
                type="file"
                class="dropzone__input"
                accept={ALLOWED_MEDIA_TYPES.join(",")}
                onchange={handleFileSelect}
              />
            {/if}
          </div>

          {#if fileError}
            <FormError message={fileError} />
          {/if}

          {#if selectedFile}
            <div class="upload-actions">
              <Button variant="secondary" onclick={clearUpload}>Clear</Button>
              <Button variant="primary" onclick={startUpload}>
                <Upload size={14} />
                Upload
              </Button>
            </div>
          {/if}
        {:else if uploadStep === "checking"}
          <div class="status-panel">
            <div class="spinner"></div>
            <p>Checking for duplicates...</p>
          </div>
        {:else if uploadStep === "duplicate"}
          <div class="status-panel status-panel--warning">
            <AlertCircle size={32} />
            <p>This file already exists</p>
            {#if duplicateMedia}
              <span class="duplicate-name"
                >{getMediaDisplayName(duplicateMedia)}</span
              >
            {/if}
            <div class="upload-actions">
              <Button variant="secondary" onclick={uploadAnyway}
                >Upload as new</Button
              >
              <Button variant="primary" onclick={selectDuplicate}
                >Use existing</Button
              >
            </div>
          </div>
        {:else if uploadStep === "uploading"}
          <div class="status-panel">
            <div class="progress-bar">
              <div
                class="progress-bar__fill"
                style="width: {uploadProgress}%"
              ></div>
            </div>
            <p>Uploading... {uploadProgress.toFixed(0)}%</p>
          </div>
        {:else if uploadStep === "finalising"}
          <div class="status-panel">
            <div class="spinner"></div>
            <p>Finalising...</p>
          </div>
        {:else if uploadStep === "complete"}
          <div class="status-panel status-panel--success">
            <Check size={32} />
            <p>Upload complete!</p>
            <div class="upload-actions">
              <Button variant="secondary" onclick={clearUpload}
                >Upload another</Button
              >
              <Button variant="primary" onclick={selectUploaded}
                >Use this media</Button
              >
            </div>
          </div>
        {:else if uploadStep === "error"}
          <div class="status-panel status-panel--error">
            <AlertCircle size={32} />
            <p>{uploadError || "Upload failed"}</p>
            <Button variant="secondary" onclick={clearUpload}>Try again</Button>
          </div>
        {/if}
      </div>
    </TabsContent>
  </TabsRoot>

  {#snippet footer()}
    <Button variant="secondary" onclick={handleCancel}>Cancel</Button>
  {/snippet}
</Dialog>

<style>
  :global(.media-picker-dialog) {
    width: min(50rem, calc(100vw - 2rem)) !important;
    max-height: min(85vh, 50rem) !important;
  }

  .browse-content {
    min-height: 300px;
  }

  .media-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: 0.75rem;
    margin-top: 1rem;
  }

  .media-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    padding: 1rem;
    border: 1px solid var(--underlay-color-border, #374151);
    border-radius: 0.5rem;
    background: var(--underlay-color-surface, #1f2937);
    cursor: pointer;
    transition:
      border-color 0.15s,
      background-color 0.15s;
    text-align: center;
  }

  .media-item:hover {
    border-color: var(--underlay-color-primary, #3b82f6);
    background: var(--underlay-color-surface-raised, #374151);
  }

  .media-item__icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 48px;
    height: 48px;
    border-radius: 0.375rem;
    background: var(--underlay-color-surface-raised, #374151);
  }

  .media-item__info {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }

  .media-item__title {
    font-size: 0.875rem;
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 150px;
  }

  .media-item__meta {
    font-size: 0.75rem;
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    padding: 2rem;
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .load-more {
    display: flex;
    justify-content: center;
    margin-top: 1rem;
  }

  .upload-content {
    min-height: 200px;
    margin-top: 1rem;
  }

  .dropzone {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 2rem;
    border: 2px dashed var(--underlay-color-border, #374151);
    border-radius: 0.5rem;
    background: var(--underlay-color-surface, #1f2937);
    cursor: pointer;
    transition: border-color 0.15s;
    text-align: center;
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .dropzone:hover {
    border-color: var(--underlay-color-primary, #3b82f6);
  }

  .dropzone--has-file {
    border-style: solid;
    border-color: var(--underlay-color-primary, #3b82f6);
    cursor: default;
  }

  .dropzone--has-error {
    border-color: var(--underlay-color-danger, #ef4444);
  }

  .dropzone__hint {
    font-size: 0.75rem;
    opacity: 0.7;
  }

  .dropzone__input {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    opacity: 0;
    cursor: pointer;
  }

  .dropzone--has-file .dropzone__input {
    display: none;
  }

  .selected-file {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    color: var(--underlay-color-text, #f3f4f6);
  }

  .selected-file__info {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 0.125rem;
  }

  .selected-file__name {
    font-weight: 500;
  }

  .selected-file__meta {
    font-size: 0.75rem;
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .upload-actions {
    display: flex;
    justify-content: center;
    gap: 0.5rem;
    margin-top: 1rem;
  }

  .status-panel {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
    padding: 2rem;
    text-align: center;
  }

  .status-panel p {
    margin: 0;
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .status-panel--warning {
    color: var(--underlay-color-warning, #f59e0b);
  }

  .status-panel--success {
    color: var(--underlay-color-success, #22c55e);
  }

  .status-panel--error {
    color: var(--underlay-color-danger, #ef4444);
  }

  .duplicate-name {
    font-weight: 500;
    color: var(--underlay-color-text, #f3f4f6);
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid var(--underlay-color-border, #374151);
    border-top-color: var(--underlay-color-primary, #3b82f6);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .progress-bar {
    width: 100%;
    max-width: 200px;
    height: 6px;
    background: var(--underlay-color-surface-raised, #374151);
    border-radius: 3px;
    overflow: hidden;
  }

  .progress-bar__fill {
    height: 100%;
    background: var(--underlay-color-primary, #3b82f6);
    transition: width 0.1s ease-out;
  }
</style>
