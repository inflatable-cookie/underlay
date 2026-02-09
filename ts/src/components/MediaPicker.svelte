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
    computeFileHash,
    validateFile,
    type PaginatedResponse,
    type PaginationParams,
    MediaKind,
    getMediaKindLabel,
    type MediaSummary,
    type MediaDetail,
    type CreateMediaRequest,
    type CheckDuplicateResponse,
    type InitiateUploadRequest,
    type InitiateUploadResponse,
    type FinaliseUploadRequest,
    type FinaliseUploadResponse,
  } from "../patterns/index.js";
  import { uploadNewMedia } from "./media-picker/upload";
  import MediaBrowseItem from "./media-picker/MediaBrowseItem.svelte";
  import MediaUploadDropzone from "./media-picker/MediaUploadDropzone.svelte";
  import MediaUploadStatusPanel from "./media-picker/MediaUploadStatusPanel.svelte";
  import Button from "./Button.svelte";
  import Dialog from "./Dialog.svelte";
  import FormError from "./FormError.svelte";
  import PageLoading from "./PageLoading.svelte";
  import TabsRoot from "./TabsRoot.svelte";
  import TabsList from "./TabsList.svelte";
  import TabsTrigger from "./TabsTrigger.svelte";
  import TabsContent from "./TabsContent.svelte";
  import Image from "lucide-svelte/icons/image";
  import Upload from "lucide-svelte/icons/upload";
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
      uploadProgress = 0;
      createdMedia = await uploadNewMedia({
        file: selectedFile,
        fileHash,
        maxFileSize,
        createMedia,
        initiateUpload,
        finaliseUpload,
        onStage: (stage) => {
          uploadStep = stage;
        },
        onProgress: (percent) => {
          uploadProgress = percent;
        }
      });

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
              <MediaBrowseItem {item} onSelect={selectMedia} />
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
          <MediaUploadDropzone
            {selectedFile}
            hasError={!!fileError}
            onDrop={handleDrop}
            onDragOver={handleDragOver}
            onFileSelect={handleFileSelect}
          />

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
        {:else}
          <MediaUploadStatusPanel
            {uploadStep}
            {duplicateMedia}
            {uploadProgress}
            {uploadError}
            onUploadAnyway={uploadAnyway}
            onSelectDuplicate={selectDuplicate}
            onClearUpload={clearUpload}
            onSelectUploaded={selectUploaded}
          />
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

</style>
