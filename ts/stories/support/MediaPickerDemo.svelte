<script lang="ts">
  import { Button } from "@poodle/svelte-primitives";
  import MediaPicker from "../../src/components/MediaPicker.svelte";
  import { MediaKind, MediaVisibility, type MediaSummary } from "../../src/patterns";

  let open = $state(false);
  let selectedLabel = $state<string | null>(null);

  const media: MediaSummary[] = [
    {
      id: "media-1",
      kind: MediaKind.Image,
      visibility: MediaVisibility.Public,
      title: "Launch Banner",
      originalFilename: "launch-banner.png",
      currentVersionId: "version-1",
      createdAt: "2026-03-24T10:00:00Z",
      updatedAt: "2026-03-24T10:00:00Z",
      deletedAt: null,
      byteSize: 128000,
      mimeType: "image/png",
      thumbnailUrl: "https://picsum.photos/seed/banner/400/240"
    },
    {
      id: "media-2",
      kind: MediaKind.Video,
      visibility: MediaVisibility.Restricted,
      title: "Feature Walkthrough",
      originalFilename: "walkthrough.mp4",
      currentVersionId: "version-2",
      createdAt: "2026-03-23T09:30:00Z",
      updatedAt: "2026-03-23T09:30:00Z",
      deletedAt: null,
      byteSize: 5120000,
      mimeType: "video/mp4",
      thumbnailUrl: "https://picsum.photos/seed/video/400/240"
    }
  ];

  async function wait(ms: number) {
    return new Promise<void>((resolve) => setTimeout(resolve, ms));
  }

  async function listMediaPaginated(params?: { cursor?: string | null }) {
    await wait(150);
    return {
      items: media,
      nextCursor: params?.cursor ? null : null,
      hasMore: false
    };
  }

  async function checkDuplicate(_sha256: string) {
    await wait(100);
    return { duplicate: false, media: null };
  }

  async function createMedia(request: { filename: string; kind: string; mimeType?: string | null }) {
    await wait(120);
    return {
      id: "media-uploaded",
      kind: request.kind,
      visibility: MediaVisibility.Restricted,
      title: request.filename,
      originalFilename: request.filename,
      currentVersionId: "version-uploaded",
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
      deletedAt: null,
      byteSize: 0,
      mimeType: request.mimeType ?? null,
      thumbnailUrl: null
    };
  }

  async function initiateUpload(_mediaId: string, _request: unknown) {
    await wait(120);
    return {
      versionId: "version-uploaded",
      uploadPlan: {
        uploadUrl: "https://example.test/upload",
        method: "PUT",
        headers: {}
      }
    };
  }

  async function finaliseUpload(_mediaId: string, _versionId: string, _request: unknown) {
    await wait(120);
    return { success: true };
  }
</script>

<div class="media-picker-demo">
  <Button variant="primary" onclick={() => (open = true)}>Open media picker</Button>

  {#if selectedLabel}
    <p class="media-picker-demo__selection">Selected: {selectedLabel}</p>
  {/if}
</div>

<MediaPicker
  bind:open
  {listMediaPaginated}
  {checkDuplicate}
  {createMedia}
  {initiateUpload}
  {finaliseUpload}
  onselect={(_id, item) => {
    selectedLabel = item.title ?? item.originalFilename ?? item.id;
  }}
/>

<style>
  .media-picker-demo {
    min-width: 20rem;
  }

  .media-picker-demo__selection {
    margin-top: 0.75rem;
    color: var(--underlay-color-text-muted, #94a3b8);
  }
</style>
