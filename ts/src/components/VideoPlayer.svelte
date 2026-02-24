<script lang="ts">
  import { sanitizeEmbedHtml } from "../utils/html.js";

  interface Props {
    title: string | undefined;
    provider: string | null | undefined;
    externalId: string | null | undefined;
    embedSource: string;
    thumbnailUrl?: string | null | undefined;
  }

  let { title, provider, externalId, embedSource, thumbnailUrl = null }: Props = $props();

  const normalisedProvider = $derived(provider?.toLowerCase() ?? null);

  const iframeSrc = $derived(
    normalisedProvider === "vimeo" && externalId
      ? `https://player.vimeo.com/video/${externalId}`
      : normalisedProvider === "youtube" && externalId
        ? `https://www.youtube.com/embed/${externalId}`
        : null
  );

  const safeEmbedSource = $derived(sanitizeEmbedHtml(embedSource));
</script>

<figure data-video-player>
  {#if iframeSrc}
    <div data-video-embed-frame>
      <iframe
        src={iframeSrc}
        title={title ?? "Video"}
        loading="lazy"
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
        allowfullscreen
      ></iframe>
    </div>
  {:else}
    {#if thumbnailUrl}
      <div data-video-thumbnail>
        <img src={thumbnailUrl} alt={title ?? "Video thumbnail"} loading="lazy" />
      </div>
    {/if}
    <div data-video-raw-embed>
      {@html safeEmbedSource}
    </div>
  {/if}

  {#if title}
    <figcaption>{title}</figcaption>
  {/if}
</figure>
