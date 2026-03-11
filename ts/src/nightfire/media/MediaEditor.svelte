<script lang="ts">
  import TextInput from "../../components/TextInput.svelte";
  import Select from "../../components/Select.svelte";
  import MediaThumbnail from "../../components/MediaThumbnail.svelte";
  import type { MarkdownEditorContext } from "../../components/markdown-editor-events";
  import { type MediaKind } from "../../patterns/media-types/enums";
  import { useNightfireMedia, type NightfireMediaPickResult } from "./context";

  type MediaBlock = {
    type: string;
    version?: string;
    hash?: string;
    data?: {
      mediaId?: string;
      caption?: string;
      alt?: string;
      display?: string;
    };
  };

  interface Props {
    block: MediaBlock;
    onChange: (block: MediaBlock) => void;
    onContextChange?: ((context: MarkdownEditorContext) => void) | null;
  }

  let { block, onChange, onContextChange = null }: Props = $props();

  const mediaContext = useNightfireMedia();

  // Local state for the selected media preview
  let selectedTitle = $state<string | null>(null);
  let selectedThumbnailUrl = $state<string | null>(null);
  let selectedKind = $state<string | null>(null);
  let picking = $state(false);

  // Derive current data from block
  const mediaId = $derived(block?.data?.mediaId ?? "");
  const caption = $derived(block?.data?.caption ?? "");
  const alt = $derived(block?.data?.alt ?? "");
  const display = $derived(block?.data?.display ?? "inline");

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
      hash: block?.hash ?? "",
      data: {
        mediaId: block?.data?.mediaId ?? "",
        caption: block?.data?.caption ?? "",
        alt: block?.data?.alt ?? "",
        display: block?.data?.display ?? "inline",
        ...updates
      }
    });
  }

  async function handlePickMedia() {
    if (!mediaContext || picking) return;
    picking = true;

    try {
      const result = await mediaContext.pickMedia();
      if (result) {
        applySelection(result);
      }
    } finally {
      picking = false;
    }
  }

  function applySelection(result: NightfireMediaPickResult) {
    selectedTitle = result.title;
    selectedThumbnailUrl = result.thumbnailUrl;
    selectedKind = result.kind;
    emit({ mediaId: result.id });
  }

  function handleRemove() {
    selectedTitle = null;
    selectedThumbnailUrl = null;
    selectedKind = null;
    emit({ mediaId: "" });
  }
</script>

<div class="media-editor">
  <!-- Media selection -->
  <div class="media-editor__selector">
    {#if mediaId}
      <div class="media-editor__preview">
        <MediaThumbnail
          thumbnailUrl={selectedThumbnailUrl}
          kind={(selectedKind ?? "other") as MediaKind}
          alt={selectedTitle ?? "Selected media"}
          size="xl"
          showAccent
        />
        <div class="media-editor__preview-info">
          <span class="media-editor__preview-title">
            {selectedTitle ?? mediaId}
          </span>
          <span class="media-editor__preview-id">{mediaId}</span>
          <div class="media-editor__preview-actions">
            {#if mediaContext}
              <button type="button" class="media-editor__btn" onclick={handlePickMedia} disabled={picking}>
                Change
              </button>
            {/if}
            <button type="button" class="media-editor__btn media-editor__btn--danger" onclick={handleRemove}>
              Remove
            </button>
          </div>
        </div>
      </div>
    {:else if mediaContext}
      <button
        type="button"
        class="media-editor__pick-btn"
        onclick={handlePickMedia}
        disabled={picking}
      >
        {picking ? "Opening picker…" : "Select media"}
      </button>
    {:else}
      <!-- Fallback: plain text input when no media context is configured -->
      <TextInput
        placeholder="Media ID"
        value={mediaId}
        oninput={(v) => emit({ mediaId: v })}
      />
    {/if}
  </div>

  <!-- Fields (only shown when media is selected) -->
  {#if mediaId}
    <div class="media-editor__fields">
      <TextInput
        placeholder="Caption (optional)"
        value={caption}
        oninput={(v) => emit({ caption: v })}
      />
      <TextInput
        placeholder="Alt text (optional)"
        value={alt}
        oninput={(v) => emit({ alt: v })}
      />
      <Select
        value={display}
        items={displayOptions}
        onchange={(v) => emit({ display: v })}
        placeholder="Display mode"
      />
    </div>
  {/if}
</div>

<style>
  .media-editor {
    display: grid;
    gap: var(--underlay-space-3, 0.75rem);
  }

  .media-editor__selector {
    display: grid;
  }

  .media-editor__preview {
    display: flex;
    gap: var(--underlay-space-3, 0.75rem);
    align-items: flex-start;
    padding: var(--underlay-space-3, 0.75rem);
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.35));
    border-radius: var(--underlay-radius-md, 0.375rem);
    background: var(--underlay-color-surface-secondary, rgba(255, 255, 255, 0.03));
  }

  .media-editor__preview-info {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-1, 0.25rem);
    min-width: 0;
    flex: 1;
  }

  .media-editor__preview-title {
    font-weight: 600;
    font-size: calc(1em * var(--underlay-font-scale-sm, 0.875));
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .media-editor__preview-id {
    font-size: calc(1em * var(--underlay-font-scale-xs, 0.75));
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.7));
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-family: var(--underlay-font-mono, monospace);
  }

  .media-editor__preview-actions {
    display: flex;
    gap: var(--underlay-space-2, 0.5rem);
    margin-top: var(--underlay-space-1, 0.25rem);
  }

  .media-editor__btn {
    font-size: calc(1em * var(--underlay-font-scale-xs, 0.75));
    padding: var(--underlay-space-1, 0.25rem) var(--underlay-space-2, 0.5rem);
    border-radius: var(--underlay-radius-sm, 0.25rem);
    border: 1px solid var(--underlay-color-border-strong, rgba(148, 163, 184, 0.5));
    background: transparent;
    color: inherit;
    cursor: pointer;
  }

  .media-editor__btn:hover {
    background: var(--underlay-color-surface-hover, rgba(255, 255, 255, 0.06));
  }

  .media-editor__btn--danger:hover {
    color: var(--underlay-color-danger, #ef4444);
    border-color: var(--underlay-color-danger, #ef4444);
  }

  .media-editor__pick-btn {
    padding: var(--underlay-space-3, 0.75rem) var(--underlay-space-4, 1rem);
    border-radius: var(--underlay-radius-md, 0.375rem);
    border: 2px dashed var(--underlay-color-border-strong, rgba(148, 163, 184, 0.5));
    background: transparent;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.7));
    cursor: pointer;
    font-size: calc(1em * var(--underlay-font-scale-sm, 0.875));
    transition: border-color 0.15s, color 0.15s;
  }

  .media-editor__pick-btn:hover:not(:disabled) {
    border-color: var(--underlay-color-primary, #3b82f6);
    color: var(--underlay-color-primary, #3b82f6);
  }

  .media-editor__pick-btn:disabled {
    opacity: 0.6;
    cursor: wait;
  }

  .media-editor__fields {
    display: grid;
    gap: var(--underlay-space-2, 0.5rem);
  }
</style>
