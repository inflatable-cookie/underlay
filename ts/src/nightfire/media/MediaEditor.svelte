<script lang="ts">
  import { onDestroy } from "svelte";
  import { Button, MediaThumbnail, Select, TextInput } from "@inflatable-cookie/poodle-svelte";
  import { useNightfireMedia, type NightfireMediaPickResult } from "./context";

  type MediaBlock = {
    type: string;
    version?: string;
    data?: {
      media_id?: string;
      caption?: string;
      alt?: string;
      display?: string;
    };
  };

  interface Props {
    block: MediaBlock;
    onChange: (block: MediaBlock) => void;
  }

  let { block, onChange }: Props = $props();

  const uid = $props.id();
  const mediaContext = useNightfireMedia();

  // Local state for the selected media preview (tracks which mediaId it belongs to)
  let selectedMediaId = $state<string | null>(null);
  let selectedTitle = $state<string | null>(null);
  let selectedThumbnailUrl = $state<string | null>(null);
  let selectedKind = $state<string | null>(null);
  let picking = $state(false);
  let pickError = $state<string | null>(null);

  // Derive current data from block
  const mediaId = $derived(block?.data?.media_id ?? "");
  const caption = $derived(block?.data?.caption ?? "");
  const alt = $derived(block?.data?.alt ?? "");
  const display = $derived(block?.data?.display ?? "inline");

  // Clear local preview metadata whenever block.data.media_id changes externally
  $effect(() => {
    if (selectedMediaId !== null && mediaId !== selectedMediaId) {
      selectedMediaId = null;
      selectedTitle = null;
      selectedThumbnailUrl = null;
      selectedKind = null;
    }
  });

  const displayOptions = [
    { value: "inline", label: "Inline" },
    { value: "block", label: "Block (full width)" },
    { value: "float-left", label: "Float left" },
    { value: "thumbnail", label: "Thumbnail" }
  ];

  function emit(updates: Partial<NonNullable<MediaBlock["data"]>>) {
    onChange({
      type: block?.type ?? "media",
      version: block?.version ?? "initial",
      data: {
        media_id: block?.data?.media_id ?? "",
        caption: block?.data?.caption ?? "",
        alt: block?.data?.alt ?? "",
        display: block?.data?.display ?? "inline",
        ...updates
      }
    });
  }

  let disposed = false;

  onDestroy(() => {
    disposed = true;
  });

  async function handlePickMedia() {
    if (!mediaContext || picking || disposed) return;
    picking = true;
    pickError = null;

    try {
      const result = await mediaContext.pickMedia();
      if (disposed) return;
      if (result) {
        applySelection(result);
      }
    } catch (err) {
      if (disposed) return;
      pickError = err instanceof Error ? err.message : "Failed to pick media";
    } finally {
      if (!disposed) {
        picking = false;
      }
    }
  }

  function applySelection(result: NightfireMediaPickResult) {
    selectedMediaId = result.id;
    selectedTitle = result.title || result.originalFilename || null;
    selectedThumbnailUrl = result.thumbnailUrl;
    selectedKind = result.kind;
    emit({ media_id: result.id });
  }

  function handleRemove() {
    selectedMediaId = null;
    selectedTitle = null;
    selectedThumbnailUrl = null;
    selectedKind = null;
    pickError = null;
    emit({ media_id: "" });
  }

  function toPoodleMediaKind(kind: string | null): "image" | "audio" | "video" | "document" {
    if (kind === "image") return "image";
    if (kind === "audio") return "audio";
    if (kind === "video") return "video";
    return "document";
  }
</script>

<div class="underlay-media-editor">
  <!-- Media selection -->
  <div class="underlay-media-editor__selector">
    {#if !mediaContext}
      <!-- Fallback: plain text input when no media context is configured -->
      <label class="underlay-media-editor__field" for={`${uid}-media-id`}>
        <span class="underlay-media-editor__label">Media ID</span>
        <TextInput
          id={`${uid}-media-id`}
          placeholder="Media ID"
          value={mediaId}
          onValueChange={(nextValue) => emit({ media_id: nextValue })}
        />
      </label>
    {:else if mediaId}
      <div class="underlay-media-editor__preview">
        <MediaThumbnail
          kind={toPoodleMediaKind(selectedKind)}
          presentation="compact"
          aspectRatio="square"
          ariaLabel={selectedTitle ?? "Selected media"}
        >
          {#if selectedThumbnailUrl}
            <img
              src={selectedThumbnailUrl}
              alt={selectedTitle ?? "Selected media"}
              class="underlay-media-editor__thumbnail-image"
            />
          {/if}
        </MediaThumbnail>
        <div class="underlay-media-editor__preview-info">
          <span class="underlay-media-editor__preview-title">
            {selectedTitle || "Selected media"}
          </span>
          <span class="underlay-media-editor__preview-id">{mediaId}</span>
          <div class="underlay-media-editor__preview-actions">
            <Button type="button" variant="secondary" size="sm" onClick={handlePickMedia} disabled={picking}>
              Change
            </Button>
            <Button type="button" variant="ghost" tone="danger" size="sm" onClick={handleRemove}>
              Remove
            </Button>
          </div>
        </div>
      </div>
    {:else}
      <Button
        type="button"
        variant="secondary"
        onClick={handlePickMedia}
        loading={picking}
      >
        {picking ? "Opening picker…" : "Select media"}
      </Button>
    {/if}
    {#if pickError}
      <p class="underlay-media-editor__error" role="alert">{pickError}</p>
    {/if}
  </div>

  <!-- Fields (only shown when media is selected) -->
  {#if mediaId}
    <div class="underlay-media-editor__fields">
      <label class="underlay-media-editor__field" for={`${uid}-media-caption`}>
        <span class="underlay-media-editor__label">Caption</span>
        <TextInput
          id={`${uid}-media-caption`}
          placeholder="Caption (optional)"
          value={caption}
          onValueChange={(nextValue) => emit({ caption: nextValue })}
        />
      </label>
      <label class="underlay-media-editor__field" for={`${uid}-media-alt`}>
        <span class="underlay-media-editor__label">Alt text</span>
        <TextInput
          id={`${uid}-media-alt`}
          placeholder="Alt text (optional)"
          value={alt}
          onValueChange={(nextValue) => emit({ alt: nextValue })}
        />
      </label>
      <label class="underlay-media-editor__field" for={`${uid}-media-display`}>
        <span class="underlay-media-editor__label">Display mode</span>
        <Select
          id={`${uid}-media-display`}
          value={display}
          options={displayOptions}
          onValueChange={(value) => emit({ display: value })}
          placeholder="Display mode"
        />
      </label>
    </div>
  {/if}
</div>

<style>
  .underlay-media-editor {
    display: grid;
    gap: var(--underlay-space-3, 0.75rem);
  }

  .underlay-media-editor__selector {
    display: grid;
    gap: var(--underlay-space-2, 0.5rem);
  }

  .underlay-media-editor__error {
    margin: 0;
    font-size: var(--underlay-font-size-sm, 0.8rem);
    color: var(--underlay-color-error, #dc2626);
  }

  .underlay-media-editor__field {
    display: grid;
    gap: var(--underlay-space-1, 0.25rem);
  }

  .underlay-media-editor__label {
    font-size: var(--underlay-font-size-xs, 0.75rem);
    font-weight: 500;
    color: var(--underlay-color-text-muted, #4b5563);
  }

  .underlay-media-editor__preview {
    display: flex;
    gap: var(--underlay-space-3, 0.75rem);
    align-items: flex-start;
    padding: var(--underlay-space-3, 0.75rem);
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.35));
    border-radius: var(--underlay-radius-md, 0.75rem);
    background: var(--underlay-color-surface-secondary, #f3f4f6);
  }

  .underlay-media-editor__preview :global(.media-thumbnail) {
    width: 6rem;
    flex-shrink: 0;
  }

  .underlay-media-editor__thumbnail-image {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .underlay-media-editor__preview-info {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-1, 0.25rem);
    min-width: 0;
    flex: 1;
  }

  .underlay-media-editor__preview-title {
    font-weight: 600;
    font-size: calc(1em * var(--underlay-font-scale-sm, 0.875));
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .underlay-media-editor__preview-id {
    font-size: calc(1em * var(--underlay-font-scale-xs, 0.75));
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.7));
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-family: var(--underlay-font-mono, monospace);
  }

  .underlay-media-editor__preview-actions {
    display: flex;
    gap: var(--underlay-space-2, 0.5rem);
    margin-top: var(--underlay-space-1, 0.25rem);
  }

  .underlay-media-editor__fields {
    display: grid;
    gap: var(--underlay-space-2, 0.5rem);
  }
</style>
