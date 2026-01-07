<script lang="ts">
  type SummaryImageSliderBlock = {
    data?: {
      subTitle?: string | null;
      description?: string | null;
      image1Id?: string | null;
      image1Alt?: string | null;
      image2Id?: string | null;
      image2Alt?: string | null;
      startPosition?: string | null;
    };
  };

  export let block: SummaryImageSliderBlock;

  const subTitle =
    typeof block?.data?.subTitle === "string"
      ? block.data.subTitle
      : null;
  const description = block?.data?.description ?? null;
  const image1Id = block?.data?.image1Id ?? null;
  const image1Alt = block?.data?.image1Alt ?? null;
  const image2Id = block?.data?.image2Id ?? null;
  const image2Alt = block?.data?.image2Alt ?? null;
  const startPosition = (block?.data?.startPosition ?? "left") as
    | "left"
    | "right";

  function escapeHtml(value: string): string {
    return value
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/\"/g, "&quot;")
      .replace(/'/g, "&#39;");
  }

  function renderMarkdown(raw: string | null | undefined): string {
    const source = (raw ?? "").trim();
    if (!source) return "";

    let html = escapeHtml(source);

    html = html.replace(/^### (.+)$/gm, "<h3>$1</h3>");
    html = html.replace(/^## (.+)$/gm, "<h2>$1</h2>");
    html = html.replace(/^# (.+)$/gm, "<h1>$1</h1>");

    html = html.replace(/\*\*(.+?)\*\*/g, "<strong>$1</strong>");
    html = html.replace(/\*(.+?)\*/g, "<em>$1</em>");

    const paragraphs = html
      .split(/\n{2,}/)
      .map((p) => `<p>${p.replace(/\n/g, "<br />")}</p>`);

    return paragraphs.join("\n");
  }
</script>

<section data-summary-layout="imageSlider" class="summary-slider">
  {#if subTitle}
    <header class="summary-slider__header">
      <h3>{subTitle}</h3>
    </header>
  {/if}

  <div
    class="summary-slider__track"
    data-start-position={startPosition}
  >
    <div class="summary-slider__image summary-slider__image--primary">
      <div class="summary-slider__image-placeholder">
        <span>{image1Id}</span>
      </div>
      {#if image1Alt}
        <p class="summary-slider__image-caption">
          {image1Alt}
        </p>
      {/if}
    </div>

    <div class="summary-slider__image summary-slider__image--secondary">
      <div class="summary-slider__image-placeholder">
        <span>{image2Id}</span>
      </div>
      {#if image2Alt}
        <p class="summary-slider__image-caption">
          {image2Alt}
        </p>
      {/if}
    </div>
  </div>

  {#if renderMarkdown(description)}
    <div class="summary-slider__description">
      {@html renderMarkdown(description)}
    </div>
  {/if}
</section>

<style>
  .summary-slider {
    display: grid;
    gap: var(--froyo-space-3);
  }

  .summary-slider__header h3 {
    font-size: 0.95rem;
    font-weight: 600;
  }

  .summary-slider__track {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: var(--froyo-space-3);
  }

  @media (max-width: 768px) {
    .summary-slider__track {
      grid-template-columns: minmax(0, 1fr);
    }
  }

  .summary-slider__image-placeholder {
    border-radius: var(--froyo-radius-md);
    border: 1px dashed var(--froyo-color-border-strong);
    min-height: var(--froyo-summary-image-placeholder-min-height);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: var(--froyo-font-size-xs);
    opacity: var(--froyo-summary-placeholder-opacity);
  }

  .summary-slider__image-caption {
    margin-top: var(--froyo-space-1);
    font-size: var(--froyo-font-size-xs);
    opacity: var(--froyo-summary-caption-opacity);
  }

  .summary-slider__description :global(p) {
    margin-block: var(--froyo-space-1);
  }
</style>
