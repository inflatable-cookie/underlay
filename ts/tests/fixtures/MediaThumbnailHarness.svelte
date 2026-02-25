<script lang="ts">
  import MediaThumbnail from "../../src/components/MediaThumbnail.svelte";
  import { MediaKind } from "../../src/patterns/media-types/enums.js";

  interface Props {
    thumbnailUrl?: string | null;
    kind?: (typeof MediaKind)[keyof typeof MediaKind];
    alt?: string;
    size?: "sm" | "md" | "lg" | "xl" | "fill" | number;
    showAccent?: boolean;
    className?: string;
  }

  let {
    thumbnailUrl = null,
    kind = MediaKind.Other,
    alt = "Media thumbnail",
    size = "md",
    showAccent = false,
    className = ""
  }: Props = $props();
</script>

<MediaThumbnail {thumbnailUrl} {kind} {alt} {size} {showAccent} class={className} />
