<script lang="ts">
  /**
   * MediaThumbnail - Renders a media thumbnail with icon fallback.
   *
   * Displays either:
   * - A thumbnail image (if thumbnailUrl is provided)
   * - A fallback icon based on media kind
   *
   * @example
   * ```svelte
   * <MediaThumbnail
   *   thumbnailUrl={media.thumbnailUrl}
   *   kind={media.kind}
   *   alt={media.title}
   * />
   * ```
   */
  import { MediaKind, getMediaKindIcon, getMediaKindAccent } from "../patterns/media-types.js";

  type Size = "sm" | "md" | "lg" | "xl" | number;

  interface Props {
    /** URL to the thumbnail image */
    thumbnailUrl?: string | null;
    /** Media kind (used for fallback icon) */
    kind?: MediaKind;
    /** Alt text for the image */
    alt?: string;
    /** Size preset or pixel value */
    size?: Size;
    /** Whether to show accent color background for icon fallback */
    showAccent?: boolean;
    /** Custom CSS class */
    class?: string;
  }

  let {
    thumbnailUrl = null,
    kind = MediaKind.Other,
    alt = "",
    size = "md",
    showAccent = false,
    class: className = ""
  }: Props = $props();

  // Size mappings
  const sizeMap: Record<string, number> = {
    sm: 32,
    md: 48,
    lg: 64,
    xl: 96
  };

  // Compute pixel size
  const pixelSize = $derived(typeof size === "number" ? size : sizeMap[size] ?? 48);

  // Icon size is slightly smaller than container
  const iconSize = $derived(Math.round(pixelSize * 0.6));

  // Get the icon component for this kind
  const Icon = $derived(getMediaKindIcon(kind));

  // Get accent color for icon background
  const accentColor = $derived(getMediaKindAccent(kind));

  // Container style
  const containerStyle = $derived(
    `width: ${pixelSize}px; height: ${pixelSize}px;` +
    (showAccent && !thumbnailUrl ? ` background-color: ${accentColor}15;` : "")
  );
</script>

<div
  class="media-thumbnail {className}"
  class:media-thumbnail--has-image={!!thumbnailUrl}
  class:media-thumbnail--has-accent={showAccent && !thumbnailUrl}
  style={containerStyle}
>
  {#if thumbnailUrl}
    <img src={thumbnailUrl} {alt} class="media-thumbnail__image" />
  {:else}
    <div class="media-thumbnail__icon" style="color: {accentColor}">
      <Icon size={iconSize} />
    </div>
  {/if}
</div>

<style>
  .media-thumbnail {
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--underlay-radius-md, 0.375rem);
    overflow: hidden;
    flex-shrink: 0;
    background-color: var(--underlay-color-surface-secondary, #f3f4f6);
  }

  .media-thumbnail--has-accent {
    background-color: transparent;
  }

  .media-thumbnail__image {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .media-thumbnail__icon {
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0.8;
  }
</style>
