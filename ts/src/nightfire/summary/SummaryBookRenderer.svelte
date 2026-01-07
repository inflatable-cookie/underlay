<script lang="ts">
  type Page = {
    title?: string | null;
    body?: string | null;
  };

  type SummaryBookBlock = {
    data?: {
      pages?: Page[];
    };
  };

  export let block: SummaryBookBlock;

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
  <div data-summary-layout="book" class="summary-book empty">
    <p>No summary pages have been defined yet.</p>
  </div>
{:else}
  <div data-summary-layout="book" class="summary-book">
    {#each pages as page, index}
      <section class="summary-book__page">
        <header class="summary-book__page-header">
          <div class="summary-book__page-index">
            {index + 1}
          </div>
          {#if page.title}
            <h3 class="summary-book__page-title">
              {page.title}
            </h3>
          {/if}
        </header>
        {#if renderMarkdown(page.body)}
          <div class="summary-book__page-body">
            {@html renderMarkdown(page.body)}
          </div>
        {/if}
      </section>
    {/each}
  </div>
{/if}

<style>
  .summary-book {
    display: grid;
    gap: var(--froyo-space-5);
  }

  .summary-book__page {
    border-radius: var(--froyo-radius-md);
    padding: var(--froyo-space-4) var(--froyo-space-5);
    background: var(--froyo-color-surface-muted);
    border: 1px solid var(--froyo-color-border-subtle);
  }

  .summary-book__page-header {
    display: flex;
    align-items: center;
    gap: var(--froyo-space-3);
    margin-bottom: var(--froyo-space-2);
  }

  .summary-book__page-index {
    width: 1.75rem;
    height: 1.75rem;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--froyo-radius-pill);
    background: var(--froyo-color-primary);
    color: var(--froyo-color-on-primary);
    font-size: var(--froyo-font-size-xs);
    font-weight: 600;
  }

  .summary-book__page-title {
    font-size: 0.95rem;
    font-weight: 600;
  }

  .summary-book__page-body :global(p) {
    margin-block: var(--froyo-space-1);
  }
</style>
