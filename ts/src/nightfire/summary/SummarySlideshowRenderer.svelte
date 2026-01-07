<script lang="ts">
  type Page = {
    title?: string | null;
    image_id?: string | null;
    body?: string | null;
  };

  type SummarySlideshowBlock = {
    data?: {
      pages?: Page[];
    };
  };

  export let block: SummarySlideshowBlock;

  const pages: Page[] = Array.isArray(block?.data?.pages)
    ? (block.data!.pages as Page[])
    : [];

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

{#if pages.length === 0}
  <div data-summary-layout="slideshow" class="summary-slideshow empty">
    <p>No slideshow slides have been defined yet.</p>
  </div>
{:else}
  <div data-summary-layout="slideshow" class="summary-slideshow">
    {#each pages as page, index}
      <section class="summary-slideshow__slide">
        <header class="summary-slideshow__header">
          <span class="summary-slideshow__index">
            {index + 1}
          </span>
          {#if page.title}
            <h3 class="summary-slideshow__title">{page.title}</h3>
          {/if}
        </header>

        <div class="summary-slideshow__content">
          {#if page.image_id}
            <div class="summary-slideshow__image">
              <div class="summary-slideshow__image-placeholder">
                <span>{page.image_id}</span>
              </div>
            </div>
          {/if}

          {#if renderMarkdown(page.body)}
            <div class="summary-slideshow__body">
              {@html renderMarkdown(page.body)}
            </div>
          {/if}
        </div>
      </section>
    {/each}
  </div>
{/if}

<style>
  .summary-slideshow {
    display: grid;
    gap: var(--froyo-space-5);
  }

  .summary-slideshow__slide {
    border-radius: var(--froyo-radius-md);
    padding: var(--froyo-space-4) var(--froyo-space-5);
    background: var(--froyo-color-surface-muted);
    border: 1px solid var(--froyo-color-border-subtle);
  }

  .summary-slideshow__header {
    display: flex;
    align-items: center;
    gap: var(--froyo-space-3);
    margin-bottom: var(--froyo-space-2);
  }

  .summary-slideshow__index {
    width: 1.6rem;
    height: 1.6rem;
    border-radius: var(--froyo-radius-pill);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: var(--froyo-color-primary);
    color: var(--froyo-color-on-primary);
    font-size: var(--froyo-font-size-xs);
    font-weight: 600;
  }

  .summary-slideshow__title {
    font-size: 0.9rem;
    font-weight: 600;
  }

  .summary-slideshow__content {
    display: grid;
    grid-template-columns: minmax(0, 1.6fr) minmax(0, 2fr);
    gap: var(--froyo-space-3);
  }

  @media (max-width: 768px) {
    .summary-slideshow__content {
      grid-template-columns: minmax(0, 1fr);
    }
  }

  .summary-slideshow__image-placeholder {
    border-radius: var(--froyo-radius-md);
    border: 1px dashed var(--froyo-color-border-strong);
    min-height: var(--froyo-summary-image-placeholder-min-height);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: var(--froyo-font-size-xs);
    opacity: var(--froyo-summary-placeholder-opacity);
  }

  .summary-slideshow__body :global(p) {
    margin-block: var(--froyo-space-1);
  }
</style>
