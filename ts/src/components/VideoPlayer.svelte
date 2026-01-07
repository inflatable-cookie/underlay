<script lang="ts">
  export let title: string | undefined;
  export let provider: string | null | undefined;
  export let externalId: string | null | undefined;
  export let embedSource: string;
  export let thumbnailUrl: string | null | undefined = null;

  const normalisedProvider = provider?.toLowerCase() ?? null;

  $: iframeSrc =
    normalisedProvider === "vimeo" && externalId
      ? `https://player.vimeo.com/video/${externalId}`
      : normalisedProvider === "youtube" && externalId
        ? `https://www.youtube.com/embed/${externalId}`
        : null;
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
      {@html embedSource}
    </div>
  {/if}

  {#if title}
    <figcaption>{title}</figcaption>
  {/if}
</figure>
