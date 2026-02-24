<script lang="ts">
  import {
    renderEmbed,
    type ParsedEmbed,
    type RenderOptions,
  } from "../embed/index.js";
  import { sanitizeEmbedHtml } from "../utils/html.js";
  import Film from "lucide-svelte/icons/film";
  import Headphones from "lucide-svelte/icons/headphones";
  import AlertCircle from "lucide-svelte/icons/alert-circle";

  interface Props {
    /** Parsed embed to preview */
    parsed?: ParsedEmbed | null;
    /** Raw embed HTML (alternative to parsed) */
    embedHtml?: string | null;
    /** Embed options */
    options?: RenderOptions;
    /** Aspect ratio (default: 16/9 for video, auto for audio) */
    aspectRatio?: number | "auto";
    /** Media type hint (video or audio) */
    mediaType?: "video" | "audio";
    /** Show loading state */
    loading?: boolean;
    /** Error message to display */
    error?: string | null;
    /** Empty state message */
    emptyMessage?: string;
    /** Additional CSS class */
    class?: string;
  }

  let {
    parsed,
    embedHtml,
    options,
    aspectRatio,
    mediaType = "video",
    loading = false,
    error,
    emptyMessage = "No embed source available.",
    class: className,
  }: Props = $props();

  // Generate embed HTML from parsed if not provided directly
  const html = $derived.by(() => {
    if (embedHtml) return embedHtml;
    if (!parsed) return null;

    // For generic passthrough, use the original embed
    if (parsed.provider === "generic" && parsed.originalEmbed) {
      return parsed.originalEmbed;
    }

    // Generate embed HTML
    return renderEmbed(parsed, options);
  });

  const sanitizedHtml = $derived(
    html ? sanitizeEmbedHtml(html) : null
  );

  // Determine if we should use aspect ratio container
  const useAspectRatio = $derived(
    aspectRatio !== "auto" && mediaType === "video"
  );

  // Calculate padding-bottom for aspect ratio
  const aspectPadding = $derived.by(() => {
    if (!useAspectRatio) return undefined;
    const ratio = typeof aspectRatio === "number" ? aspectRatio : 16 / 9;
    return `${(1 / ratio) * 100}%`;
  });
</script>

<div
  class="underlay-embed-preview underlay-embed-preview--{mediaType} {className ?? ''}"
>
  {#if loading}
    <div class="underlay-embed-preview__loading">
      <span class="underlay-embed-preview__spinner"></span>
      <p>Loading preview...</p>
    </div>
  {:else if error}
    <div class="underlay-embed-preview__error">
      <AlertCircle size={48} />
      <p>{error}</p>
    </div>
  {:else if sanitizedHtml}
    {#if useAspectRatio}
      <div
        class="underlay-embed-preview__aspect"
        style:padding-bottom={aspectPadding}
      >
        {@html sanitizedHtml}
      </div>
    {:else}
      <div class="underlay-embed-preview__content">
        {@html sanitizedHtml}
      </div>
    {/if}
  {:else}
    <div class="underlay-embed-preview__empty">
      {#if mediaType === "audio"}
        <Headphones size={48} />
      {:else}
        <Film size={48} />
      {/if}
      <p>{emptyMessage}</p>
    </div>
  {/if}
</div>

<style>
  .underlay-embed-preview {
    background: var(--underlay-color-surface-subtle, rgba(148, 163, 184, 0.08));
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.15));
    border-radius: 0.5rem;
    overflow: hidden;
  }

  /* Aspect ratio container for video */
  .underlay-embed-preview__aspect {
    position: relative;
    width: 100%;
  }

  .underlay-embed-preview__aspect :global(iframe),
  .underlay-embed-preview__aspect :global(video),
  .underlay-embed-preview__aspect :global(embed),
  .underlay-embed-preview__aspect :global(object) {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    border: 0;
  }

  /* Audio content container */
  .underlay-embed-preview--audio .underlay-embed-preview__content {
    padding: 1.5rem;
  }

  .underlay-embed-preview__content :global(iframe) {
    width: 100%;
    min-height: 160px;
    border: 0;
    border-radius: 0.375rem;
  }

  .underlay-embed-preview__content :global(audio) {
    width: 100%;
  }

  /* Loading state */
  .underlay-embed-preview__loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    padding: 3rem;
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .underlay-embed-preview__spinner {
    width: 2rem;
    height: 2rem;
    border: 3px solid var(--underlay-color-border, rgba(148, 163, 184, 0.25));
    border-top-color: var(--underlay-color-primary, #2563eb);
    border-radius: 50%;
    animation: underlay-embed-spin 0.8s linear infinite;
  }

  @keyframes underlay-embed-spin {
    to {
      transform: rotate(360deg);
    }
  }

  .underlay-embed-preview__loading p {
    margin: 0;
    font-size: 0.9rem;
  }

  /* Error state */
  .underlay-embed-preview__error {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    padding: 3rem;
    color: var(--underlay-color-danger, #ef4444);
  }

  .underlay-embed-preview__error p {
    margin: 0;
    font-size: 0.9rem;
  }

  /* Empty state */
  .underlay-embed-preview__empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    padding: 3rem;
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .underlay-embed-preview__empty p {
    margin: 0;
    font-size: 0.9rem;
  }
</style>
