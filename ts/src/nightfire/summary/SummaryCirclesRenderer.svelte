<script lang="ts">
  type Page = {
    title?: string | null;
    body?: string | null;
  };

  type SummaryCirclesBlock = {
    data?: {
      subTitle?: string | null;
      pages?: Page[];
    };
  };

  export let block: SummaryCirclesBlock;

  const subTitle =
    typeof block?.data?.subTitle === "string"
      ? block.data.subTitle
      : null;

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
  <div data-summary-layout="circles" class="summary-circles empty">
    <p>No summary panels have been defined yet.</p>
  </div>
{:else}
  <section data-summary-layout="circles" class="summary-circles">
    {#if subTitle}
      <header class="summary-circles__header">
        <h3>{subTitle}</h3>
      </header>
    {/if}

    <div class="summary-circles__grid">
      {#each pages as page}
        <article class="summary-circles__item">
          {#if page.title}
            <h4 class="summary-circles__item-title">
              {page.title}
            </h4>
          {/if}
          {#if renderMarkdown(page.body)}
            <div class="summary-circles__item-body">
              {@html renderMarkdown(page.body)}
            </div>
          {/if}
        </article>
      {/each}
    </div>
  </section>
{/if}

<style>
  .summary-circles {
    display: grid;
    gap: var(--froyo-space-4);
  }

  .summary-circles__header h3 {
    font-size: 0.95rem;
    font-weight: 600;
    opacity: 0.9;
  }

  .summary-circles__grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(12rem, 1fr));
    gap: var(--froyo-space-3);
  }

  .summary-circles__item {
    border-radius: var(--froyo-radius-pill);
    padding: var(--froyo-space-3) var(--froyo-space-4);
    background: radial-gradient(
        circle at top left,
        color-mix(in srgb, var(--froyo-color-primary) 18%, transparent),
        transparent 55%
      ),
      var(--froyo-color-surface-muted);
    border: 1px solid var(--froyo-color-border-subtle);
  }

  .summary-circles__item-title {
    font-size: var(--froyo-font-size-md);
    font-weight: 600;
    margin-bottom: calc(var(--froyo-space-1) * 0.6);
  }

  .summary-circles__item-body :global(p) {
    margin-block: calc(var(--froyo-space-1) * 0.6);
    font-size: var(--froyo-font-size-sm);
  }
</style>
