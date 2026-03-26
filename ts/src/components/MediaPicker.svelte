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
    type PaginatedResponse,
    type PaginationParams,
    MediaKind,
    type MediaSummary,
    type MediaDetail,
    type CreateMediaRequest,
    type CheckDuplicateResponse,
    type InitiateUploadRequest,
    type InitiateUploadResponse,
    type FinaliseUploadRequest,
    type FinaliseUploadResponse,
    ALLOWED_MEDIA_TYPES,
  } from "../patterns/index.js";
  import {
    runUploadFlow,
    uploadMediaWithKnownHash
  } from "./media-picker/upload-flow";
  import {
    loadMediaBrowsePage,
    mergeMediaBrowseItems
  } from "./media-picker/browse";
  import {
    createResetBrowseState,
    type MediaPickerUploadStep,
  } from "./media-picker/state";
  import {
    MediaBrowsePanel,
    MediaUploadStatusPanel,
    type MediaPickerItem,
  } from "@poodle/svelte-composites";
  import {
    Button,
    Dialog as PoodleDialog,
    FileUpload,
    Tabs,
    type TabItem,
    type FileUploadItem
  } from "@poodle/svelte-primitives";
  import Upload from "lucide-svelte/icons/upload";

  interface Props {
    /** Whether the picker dialog is open */
    open?: boolean;
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

  }

  let {
    open = $bindable(false),
    filterKind = null,
    maxFileSize = 25 * 1024 * 1024,
    listMediaPaginated,
    checkDuplicate,
    createMedia,
    initiateUpload,
    finaliseUpload,
    onselect,
  }: Props = $props();

  // Tabs state
  let activeTab = $state("browse");
  const tabItems = $derived<TabItem[]>([
    { value: "browse", label: "Browse Library" },
    { value: "upload", label: "Upload New" }
  ]);

  // Browse state
  let browseLoading = $state(false);
  let browseError = $state<string | null>(null);
  let browseItems = $state<MediaSummary[]>([]);
  let browseNextCursor = $state<string | null>(null);
  let browseHasMore = $state(false);
  let browseInitialLoadDone = $state(false);
  let browseStale = $state(false);

  // Upload state — FileUpload manages file selection; we derive selectedFile from it
  let uploadFiles = $state<FileUploadItem[]>([]);
  let selectedFile = $derived<File | null>(uploadFiles[0]?.file ?? null);
  let uploadStep = $state<MediaPickerUploadStep>("select");
  let uploadProgress = $state(0);
  let uploadError = $state<string | null>(null);
  let duplicateMedia = $state<MediaSummary | null>(null);
  let fileHash = $state<string | null>(null);
  let createdMedia = $state<MediaSummary | null>(null);

  // Load initial browse data when dialog opens
  $effect(() => {
    if (open && !browseInitialLoadDone && !browseLoading) {
      loadBrowseItems();
    }
  });

  // Refresh browse data when switching to browse tab after an upload
  $effect(() => {
    if (activeTab === "browse" && browseStale && !browseLoading) {
      browseStale = false;
      browseItems = [];
      browseNextCursor = null;
      loadBrowseItems();
    }
  });

  async function loadBrowseItems(cursor?: string) {
    browseLoading = true;
    browseError = null;

    try {
      const response = await loadMediaBrowsePage({
        listMediaPaginated,
        cursor
      });

      browseItems = mergeMediaBrowseItems(browseItems, response.items, cursor);
      browseNextCursor = response.nextCursor;
      browseHasMore = response.hasMore;
    } catch (e) {
      browseError = e instanceof Error ? e.message : "Failed to load media";
    } finally {
      browseLoading = false;
      browseInitialLoadDone = true;
    }
  }

  function loadMoreItems() {
    if (browseNextCursor) {
      loadBrowseItems(browseNextCursor);
    }
  }

  function clearUpload() {
    uploadFiles = [];
    fileHash = null;
    uploadStep = "select";
    uploadProgress = 0;
    uploadError = null;
    duplicateMedia = null;
    createdMedia = null;
  }

  async function startUpload() {
    if (!selectedFile) return;

    uploadError = null;

    try {
      const result = await runUploadFlow({
        file: selectedFile,
        maxFileSize,
        checkDuplicate,
        createMedia,
        initiateUpload,
        finaliseUpload,
        onStep: (stage) => {
          uploadStep = stage;
        },
        onProgress: (percent) => {
          uploadProgress = percent;
        }
      });

      fileHash = result.fileHash;

      if (result.kind === "duplicate") {
        duplicateMedia = result.duplicateMedia;
        uploadStep = "duplicate";
        return;
      }

      createdMedia = result.createdMedia;
      uploadStep = "complete";
      browseStale = true;
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
      createdMedia = await uploadMediaWithKnownHash({
        file: selectedFile,
        fileHash,
        maxFileSize,
        createMedia,
        initiateUpload,
        finaliseUpload,
        onStep: (stage) => {
          uploadStep = stage;
        },
        onProgress: (percent) => {
          uploadProgress = percent;
        }
      });

      uploadStep = "complete";
      browseStale = true;
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
      // For image uploads, use a local blob URL as a temporary thumbnail
      // since the server-side rendition is generated asynchronously.
      if (selectedFile && createdMedia.thumbnailUrl === null && selectedFile.type.startsWith("image/")) {
        createdMedia = {
          ...createdMedia,
          thumbnailUrl: URL.createObjectURL(selectedFile)
        };
      }
      selectMedia(createdMedia);
    }
  }

  function selectMedia(media: MediaSummary) {
    onselect?.(media.id, media);
    closeAndReset();
  }

  function toMediaPickerItem(media: MediaSummary): MediaPickerItem {
    return {
      id: media.id,
      label: media.title ?? media.originalFilename ?? "Untitled",
      thumbnailUrl: media.thumbnailUrl,
      kind:
        media.kind === MediaKind.Image
          ? "image"
          : media.kind === MediaKind.Audio
            ? "audio"
            : media.kind === MediaKind.Video
              ? "video"
              : "document",
      meta:
        media.kind === MediaKind.Image
          ? "Image"
          : media.kind === MediaKind.Audio
            ? "Audio"
            : media.kind === MediaKind.Video
              ? "Video"
              : "Document"
    };
  }

  function closeAndReset() {
    open = false;
    activeTab = "browse";
    clearUpload();
    const browseReset = createResetBrowseState();
    browseItems = browseReset.browseItems;
    browseNextCursor = browseReset.browseNextCursor;
    browseHasMore = browseReset.browseHasMore;
    browseInitialLoadDone = false;
    browseStale = false;
  }

</script>

<PoodleDialog
  bind:open
  contentClassName="media-picker-dialog"
  showCloseButton
>
  <Tabs bind:value={activeTab} items={tabItems} ariaLabel="Media picker sections" let:activeValue>
    {#if activeValue === "browse"}
      <MediaBrowsePanel
        loading={browseLoading}
        error={browseError}
        items={browseItems.map(toMediaPickerItem)}
        hasMore={browseHasMore}
        on:loadMore={loadMoreItems}
        on:select={(event) => {
          const media = browseItems.find((item) => item.id === event.detail.item.id);
          if (media) selectMedia(media);
        }}
      />
    {:else if activeValue === "upload"}
      <div class="underlay-upload-content">
        {#if uploadStep === "select"}
          <FileUpload
            bind:files={uploadFiles}
            accept={ALLOWED_MEDIA_TYPES.join(",")}
            maxSize={maxFileSize}
            showPreview={false}
          />

          {#if selectedFile}
            <div class="underlay-upload-actions">
              <Button variant="secondary" on:click={clearUpload}>Clear</Button>
              <Button variant="primary" on:click={startUpload}>
                <svelte:fragment slot="leading">
                  <Upload size={14} />
                </svelte:fragment>
                Upload
              </Button>
            </div>
          {/if}
        {:else}
          <MediaUploadStatusPanel
            {uploadStep}
            {uploadProgress}
            {uploadError}
            duplicateLabel={duplicateMedia ? (duplicateMedia.title ?? duplicateMedia.originalFilename ?? "Existing media") : null}
            on:uploadAnyway={uploadAnyway}
            on:selectDuplicate={selectDuplicate}
            on:clearUpload={clearUpload}
            on:selectUploaded={selectUploaded}
          />
        {/if}
      </div>
    {/if}
  </Tabs>

</PoodleDialog>

<style>
  :global(.media-picker-dialog) {
    width: min(50rem, calc(100vw - 2rem)) !important;
    max-height: min(85vh, 50rem) !important;
  }

  .underlay-upload-content {
    min-height: 200px;
    margin-top: 1rem;
  }

  .underlay-upload-actions {
    display: flex;
    justify-content: center;
    gap: 0.5rem;
    margin-top: 1rem;
  }
</style>
