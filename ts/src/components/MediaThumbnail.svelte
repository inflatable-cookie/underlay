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
   *
   * @example Using "fill" to fill parent container (e.g., in ListCard)
   * ```svelte
   * <MediaThumbnail
   *   thumbnailUrl={media.thumbnailUrl}
   *   kind={media.kind}
   *   size="fill"
   * />
   * ```
   */
  import { MediaKind, getMediaKindIcon, getMediaKindAccent } from "../patterns/media-types.js";

  type Size = "sm" | "md" | "lg" | "xl" | "fill" | number;

  interface Props {
    /** URL to the thumbnail image */
    thumbnailUrl?: string | null;
    /** Media kind (used for fallback icon) */
    kind?: MediaKind;
    /** Alt text for the image */
    alt?: string;
    /** Size preset, pixel value, or "fill" to fill parent container */
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

  // Check if using fill mode
  const isFillMode = $derived(size === "fill");

  // Compute pixel size (only used when not in fill mode)
  const pixelSize = $derived(
    isFillMode ? 0 : (typeof size === "number" ? size : sizeMap[size] ?? 48)
  );

  // Icon size - for fill mode, use a reasonable default
  const iconSize = $derived(isFillMode ? 28 : Math.round(pixelSize * 0.6));

  // Get the icon component for this kind
  const Icon = $derived(getMediaKindIcon(kind));

  // Get accent color for icon background
  const accentColor = $derived(getMediaKindAccent(kind));
  const iconColor = $derived(showAccent ? accentColor : "currentColor");

  // Container style
  const containerStyle = $derived(
    isFillMode
      ? (showAccent && !thumbnailUrl ? `background-color: ${accentColor}15;` : "")
      : `width: ${pixelSize}px; height: ${pixelSize}px;` +
        (showAccent && !thumbnailUrl ? ` background-color: ${accentColor}15;` : "")
  );
</script>

<div
  class="underlay-media-thumbnail {className}"
  class:underlay-media-thumbnail--has-image={!!thumbnailUrl}
  class:underlay-media-thumbnail--has-accent={showAccent && !thumbnailUrl}
  class:underlay-media-thumbnail--fill={isFillMode}
  style={containerStyle || undefined}
>
  {#if thumbnailUrl}
    <img src={thumbnailUrl} {alt} class="underlay-media-thumbnail__image" />
  {:else}
    <div class="underlay-media-thumbnail__icon" style="color: {iconColor}">
      <Icon size={iconSize} />
    </div>
  {/if}
</div>

<style>
  .underlay-media-thumbnail {
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--underlay-radius-md, 0.375rem);
    overflow: hidden;
    flex-shrink: 0;
  }

  .underlay-media-thumbnail--has-image {
    background-color: var(--underlay-color-surface-secondary, #f3f4f6);
  }

  .underlay-media-thumbnail--has-accent {
    background-color: transparent;
  }

  .underlay-media-thumbnail--fill {
    width: 100%;
    height: 100%;
  }

  .underlay-media-thumbnail__image {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .underlay-media-thumbnail__icon {
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0.8;
  }
</style>
