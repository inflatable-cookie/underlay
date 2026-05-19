<script lang="ts">
  import { Dialog, FileUpload, Tabs } from "@poodle/svelte";
  import type { FileUploadItem, TabItem } from "@poodle/svelte";
  import {
    createResetMediaBrowseState,
    loadMediaBrowsePage,
    mergeMediaBrowseItems,
    runMediaUploadWorkflow,
    type MediaUploadDisplayStep,
    type MediaUploadInitResult,
    type MediaWorkflowPageResponse
  } from "../runtime/media";
  import type {
    CreateMediaRequest,
    FinaliseUploadRequest,
    InitiateUploadRequest
  } from "../runtime/media";
  import type { MediaPickerBrowseItem, MediaPickerWorkflowItem } from "./template.types";
  import { default as MediaBrowsePanel } from "./MediaBrowsePanel.svelte";
  import { default as MediaUploadStatusPanel } from "./MediaUploadStatusPanel.svelte";

  type PickerMediaKind = "image" | "audio" | "video" | "document" | "embed";

  interface Props {
    open?: boolean;
    title?: string;
    filterKind?: string | null;
    maxFileSize?: number;
    pageSize?: number;
    listPage: (params?: { cursor?: string; limit?: number }) => Promise<MediaWorkflowPageResponse<MediaPickerWorkflowItem>>;
    checkDuplicate: (sha256: string) => Promise<{ exists: boolean; item?: MediaPickerWorkflowItem | null }>;
    createRecord: (request: CreateMediaRequest) => Promise<MediaPickerWorkflowItem>;
    buildCreateRequest: (file: File) => CreateMediaRequest;
    initiateUpload: (
      createdRecord: MediaPickerWorkflowItem,
      request: InitiateUploadRequest
    ) => Promise<MediaUploadInitResult>;
    buildInitiateRequest: (file: File) => InitiateUploadRequest;
    finaliseUpload: (
      createdRecord: MediaPickerWorkflowItem,
      versionId: string,
      request: FinaliseUploadRequest
    ) => Promise<unknown>;
    buildFinaliseRequest: (file: File, fileHash: string) => FinaliseUploadRequest;
    toCreatedItem?: (finaliseResult: unknown, createdRecord: MediaPickerWorkflowItem) => MediaPickerWorkflowItem;
    toPickerItem?: (item: MediaPickerWorkflowItem) => MediaPickerBrowseItem;
    onSelect?: (mediaId: string, media: MediaPickerWorkflowItem) => void;
  }

  let {
    open = $bindable(false),
    title = "Select media",
    filterKind = null,
    maxFileSize = 25 * 1024 * 1024,
    pageSize = 24,
    listPage,
    checkDuplicate,
    createRecord,
    buildCreateRequest,
    initiateUpload,
    buildInitiateRequest,
    finaliseUpload,
    buildFinaliseRequest,
    toCreatedItem = (_, createdRecord) => createdRecord,
    toPickerItem = defaultPickerItem,
    onSelect
  }: Props = $props();

  const tabItems: TabItem[] = [
    { value: "browse", label: "Browse" },
    { value: "upload", label: "Upload" }
  ];

  let activeTab = $state("browse");
  let browseState = $state(createResetMediaBrowseState<MediaPickerWorkflowItem>());
  let browseLoading = $state(false);
  let browseError = $state<string | null>(null);
  let uploadFiles = $state<FileUploadItem[]>([]);
  let uploadFile = $state<File | null>(null);
  let uploadStep = $state<MediaUploadDisplayStep>("select");
  let uploadProgress = $state(0);
  let uploadError = $state<string | null>(null);
  let duplicateMedia = $state<MediaPickerWorkflowItem | null>(null);
  let uploadedMedia = $state<MediaPickerWorkflowItem | null>(null);

  const browseItems = $derived(
    browseState.items.map((media) => toPickerItem(media))
  );
  const accept = $derived(filterKind ? `${filterKind}/*` : null);

  $effect(() => {
    if (open && browseState.items.length === 0 && !browseLoading) {
      void loadBrowseItems();
    }
  });

  $effect(() => {
    if (!open) {
      activeTab = "browse";
      resetUploadState();
    }
  });

  function normalizeKind(kind?: string | null): PickerMediaKind {
    switch (kind) {
      case "image":
      case "audio":
      case "video":
      case "embed":
        return kind;
      default:
        return "document";
    }
  }

  function defaultPickerItem(media: MediaPickerWorkflowItem): MediaPickerBrowseItem {
    return {
      id: media.id,
      label: media.title ?? media.originalFilename ?? "Untitled media",
      thumbnailUrl: media.thumbnailUrl ?? null,
      kind: normalizeKind(media.kind),
      meta: media.originalFilename ?? null
    };
  }

  async function loadBrowseItems(cursor?: string): Promise<void> {
    browseLoading = true;
    browseError = null;

    try {
      const nextPage = await loadMediaBrowsePage<MediaPickerWorkflowItem>({
        listPage,
        cursor,
        limit: pageSize
      });

      const filteredItems = filterKind
        ? nextPage.items.filter((item) => item.kind === filterKind)
        : nextPage.items;

      browseState = {
        items: mergeMediaBrowseItems(browseState.items, filteredItems, cursor),
        nextCursor: nextPage.nextCursor,
        hasMore: nextPage.hasMore
      };
    } catch (error) {
      browseError = error instanceof Error ? error.message : "Could not load media";
    } finally {
      browseLoading = false;
    }
  }

  function resetUploadState(): void {
    uploadFiles = [];
    uploadFile = null;
    uploadStep = "select";
    uploadProgress = 0;
    uploadError = null;
    duplicateMedia = null;
    uploadedMedia = null;
  }

  function handleSelect(media: MediaPickerWorkflowItem): void {
    open = false;
    onSelect?.(media.id, media);
  }

  async function startUpload(file: File, skipDuplicateCheck = false): Promise<void> {
    uploadFile = file;
    uploadError = null;
    duplicateMedia = null;
    uploadedMedia = null;

    try {
      const result = await runMediaUploadWorkflow<
        MediaPickerWorkflowItem,
        MediaPickerWorkflowItem,
        CreateMediaRequest,
        MediaPickerWorkflowItem,
        InitiateUploadRequest,
        FinaliseUploadRequest,
        unknown
      >({
        file,
        fileHash: "",
        maxFileSize,
        checkDuplicate: skipDuplicateCheck
          ? async () => ({ exists: false, item: null })
          : checkDuplicate,
        createRecord,
        buildCreateRequest: (nextFile) => buildCreateRequest(nextFile),
        initiateUpload,
        buildInitiateRequest: (nextFile) => buildInitiateRequest(nextFile),
        finaliseUpload,
        buildFinaliseRequest: (nextFile, fileHash) => buildFinaliseRequest(nextFile, fileHash),
        toCreatedItem,
        onStep: (step) => {
          uploadStep = step;
        },
        onProgress: (percent) => {
          uploadProgress = percent;
        }
      });

      if (result.kind === "duplicate") {
        duplicateMedia = result.existingItem;
        uploadStep = "duplicate";
        return;
      }

      uploadedMedia = result.createdItem;
      uploadStep = "complete";
      browseState = {
        ...browseState,
        items: [result.createdItem, ...browseState.items.filter((item) => item.id !== result.createdItem.id)]
      };
    } catch (error) {
      uploadError = error instanceof Error ? error.message : "Upload failed";
      uploadStep = "error";
    }
  }

  function handleFileChange(nextFiles: FileUploadItem[]): void {
    uploadFiles = nextFiles;
    const nextFile = nextFiles.at(-1)?.file ?? null;

    if (nextFile) {
      void startUpload(nextFile);
    }
  }
</script>

<Dialog bind:open {title} kind="dialog">
  <div class="underlay-media-picker-workflow">
    <Tabs
      items={tabItems}
      value={activeTab}
      onValueChange={(value) => {
        activeTab = value;
      }}
    />

    {#if activeTab === "browse"}
      <MediaBrowsePanel
        items={browseItems}
        hasMore={browseState.hasMore}
        loading={browseLoading}
        error={browseError}
        onSelect={(item) => {
          const selected = browseState.items.find((entry) => entry.id === item.id);
          if (selected) {
            handleSelect(selected);
          }
        }}
        onLoadMore={() => {
          if (browseState.nextCursor) {
            void loadBrowseItems(browseState.nextCursor);
          }
        }}
      />
    {:else if uploadStep === "select"}
      <FileUpload
        {accept}
        maxSize={maxFileSize}
        multiple={false}
        maxFiles={1}
        files={uploadFiles}
        onChange={handleFileChange}
      />
    {:else}
      <MediaUploadStatusPanel
        {uploadStep}
        {uploadProgress}
        uploadError={uploadError}
        duplicateLabel={duplicateMedia?.title ?? duplicateMedia?.originalFilename ?? null}
        uploadedLabel={uploadedMedia?.title ?? uploadedMedia?.originalFilename ?? null}
        onUploadAnyway={() => {
          if (uploadFile) {
            void startUpload(uploadFile, true);
          }
        }}
        onSelectDuplicate={() => {
          if (duplicateMedia) {
            handleSelect(duplicateMedia);
          }
        }}
        onClearUpload={resetUploadState}
        onSelectUploaded={() => {
          if (uploadedMedia) {
            handleSelect(uploadedMedia);
          }
        }}
      />
    {/if}
  </div>
</Dialog>

<style>
  .underlay-media-picker-workflow {
    display: grid;
    gap: var(--poodle-space-stack-sm);
    min-width: min(52rem, 85vw);
  }
</style>
