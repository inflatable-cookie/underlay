<script lang="ts">
  import { Button } from "@inflatable-cookie/poodle-svelte";

  interface MediaRenditionItem {
    kind: string;
    url?: string | null;
    mimeType?: string | null;
    width?: number | null;
    height?: number | null;
    byteSize?: number | null;
  }

  interface Props {
    renditions?: MediaRenditionItem[] | null;
    formatFileSize: (bytes: number | null) => string;
    showGenerateBanner?: boolean;
    generating?: boolean;
    onGenerateRenditions?: (() => void) | null;
    generateMessage?: string;
    generateLabel?: string;
    generatingLabel?: string;
    title?: string;
  }

  let {
    renditions = [],
    formatFileSize,
    showGenerateBanner = false,
    generating = false,
    onGenerateRenditions = null,
    generateMessage = "No thumbnail has been generated for this image.",
    generateLabel = "Generate Thumbnail",
    generatingLabel = "Generating...",
    title = "Renditions"
  }: Props = $props();

  const hasRenditions = $derived((renditions?.length ?? 0) > 0);
</script>

{#if showGenerateBanner}
  <div class="underlay-media-renditions__banner">
    <p>{generateMessage}</p>
    {#if onGenerateRenditions}
      <Button
        type="button"
        variant="primary"
        size="sm"
        onClick={onGenerateRenditions}
        disabled={generating}
      >
        {generating ? generatingLabel : generateLabel}
      </Button>
    {/if}
  </div>
{/if}

{#if hasRenditions}
  <section class="underlay-media-renditions">
    <h3>{title}</h3>
    <div class="underlay-media-renditions__grid">
      {#each renditions ?? [] as rendition}
        <div class="underlay-media-renditions__card">
          {#if rendition.url && rendition.mimeType?.startsWith("image/")}
            <img
              src={rendition.url}
              alt={rendition.kind}
              class="underlay-media-renditions__preview"
            />
          {:else}
            <div class="underlay-media-renditions__placeholder">
              <span>No preview</span>
            </div>
          {/if}
          <div class="underlay-media-renditions__info">
            <span class="underlay-media-renditions__kind">{rendition.kind}</span>
            <span class="underlay-media-renditions__size">
              {#if rendition.width && rendition.height}
                {rendition.width}×{rendition.height}
              {:else}
                {formatFileSize(rendition.byteSize ?? null)}
              {/if}
            </span>
          </div>
        </div>
      {/each}
    </div>
  </section>
{/if}

<style>
  .underlay-media-renditions__banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.75rem 1rem;
    margin-bottom: 1rem;
    background: var(--underlay-color-surface-warning, rgba(245, 158, 11, 0.15));
    border: 1px solid var(--underlay-color-border-warning, rgba(245, 158, 11, 0.4));
    border-radius: var(--underlay-radius-md, 0.5rem);
  }

  .underlay-media-renditions__banner p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--underlay-color-text, #e5e7eb);
  }

  .underlay-media-renditions {
    background: var(--underlay-color-surface-muted, rgba(255, 255, 255, 0.02));
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.25));
    border-radius: var(--underlay-radius-lg, 1rem);
    padding: 1rem;
  }

  .underlay-media-renditions h3 {
    margin: 0 0 1rem;
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--underlay-color-text, #e5e7eb);
  }

  .underlay-media-renditions__grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
    gap: 1rem;
  }

  .underlay-media-renditions__card {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding: 0.75rem;
    background: rgba(148, 163, 184, 0.08);
    border-radius: var(--underlay-radius-md, 0.5rem);
    border: 1px solid rgba(148, 163, 184, 0.15);
  }

  .underlay-media-renditions__preview {
    width: 100%;
    height: 100px;
    object-fit: cover;
    border-radius: 0.25rem;
    background: white;
  }

  .underlay-media-renditions__placeholder {
    width: 100%;
    height: 100px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(148, 163, 184, 0.12);
    border-radius: 0.25rem;
    font-size: 0.75rem;
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .underlay-media-renditions__info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .underlay-media-renditions__kind {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--underlay-color-text, #e5e7eb);
    text-transform: uppercase;
  }

  .underlay-media-renditions__size {
    font-size: 0.75rem;
    color: var(--underlay-color-text-muted, #9ca3af);
  }
</style>
