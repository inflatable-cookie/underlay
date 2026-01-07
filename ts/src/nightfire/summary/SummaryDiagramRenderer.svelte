<script lang="ts">
  type Page = {
    title?: string | null;
    image_id?: string | null;
    body?: string | null;
  };

  type SummaryDiagramBlock = {
    data?: {
      pages?: Page[];
    };
  };

  export let block: SummaryDiagramBlock;

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
  <div data-summary-layout="diagram" class="summary-diagram empty">
    <p>No diagram pages have been defined yet.</p>
  </div>
{:else}
  <div data-summary-layout="diagram" class="summary-diagram">
    {#each pages as page}
      <section class="summary-diagram__page">
        {#if page.title}
          <h3 class="summary-diagram__title">{page.title}</h3>
        {/if}

        <div class="summary-diagram__content">
          {#if page.image_id}
            <div class="summary-diagram__image">
              <div class="summary-diagram__image-placeholder">
                <span>{page.image_id}</span>
              </div>
            </div>
          {/if}

          {#if renderMarkdown(page.body)}
            <div class="summary-diagram__body">
              {@html renderMarkdown(page.body)}
            </div>
          {/if}
        </div>
      </section>
    {/each}
  </div>
{/if}

<style>
  .summary-diagram {
    display: grid;
    gap: var(--froyo-space-5);
  }

  .summary-diagram__page {
    border-radius: var(--froyo-radius-md);
    padding: var(--froyo-space-4) var(--froyo-space-5);
    background: var(--froyo-color-surface-muted);
    border: 1px solid var(--froyo-color-border-subtle);
  }

  .summary-diagram__title {
    font-size: 0.95rem;
    font-weight: 600;
    margin-bottom: var(--froyo-space-2);
  }

  .summary-diagram__content {
    display: grid;
    grid-template-columns: minmax(0, 1.4fr) minmax(0, 2fr);
    gap: var(--froyo-space-3);
  }

  @media (max-width: 768px) {
    .summary-diagram__content {
      grid-template-columns: minmax(0, 1fr);
    }
  }

  .summary-diagram__image-placeholder {
    border-radius: var(--froyo-radius-md);
    border: 1px dashed var(--froyo-color-border-strong);
    min-height: var(--froyo-summary-image-placeholder-min-height);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: var(--froyo-font-size-xs);
    opacity: var(--froyo-summary-placeholder-opacity);
  }

  .summary-diagram__body :global(p) {
    margin-block: var(--froyo-space-1);
  }
</style>
