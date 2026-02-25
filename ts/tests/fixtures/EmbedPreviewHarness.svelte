<script lang="ts">
  import type { ParsedEmbed, RenderOptions } from "../../src/embed/types.js";
  import EmbedPreview from "../../src/components/EmbedPreview.svelte";

  interface Props {
    parsed?: ParsedEmbed | null;
    embedHtml?: string | null;
    options?: RenderOptions | undefined;
    aspectRatio?: number | "auto";
    mediaType?: "video" | "audio";
    loading?: boolean;
    error?: string | null;
    emptyMessage?: string;
    className?: string;
  }

  let {
    parsed = null,
    embedHtml = null,
    options = undefined,
    aspectRatio = undefined,
    mediaType = "video",
    loading = false,
    error = null,
    emptyMessage = "No embed source available.",
    className = ""
  }: Props = $props();
</script>

<EmbedPreview
  {parsed}
  {embedHtml}
  {options}
  {aspectRatio}
  {mediaType}
  {loading}
  {error}
  {emptyMessage}
  class={className}
/>
