<script lang="ts">
  type MarkdownBlock = {
    data?: {
      text?: string;
    };
  };

  export let block: MarkdownBlock;

  function escapeHtml(value: string): string {
    return value
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/\"/g, "&quot;")
      .replace(/'/g, "&#39;");
  }

  function renderMarkdown(raw: string): string {
    if (!raw.trim()) return "";

    let html = escapeHtml(raw);

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

  const text =
    typeof block?.data?.text === "string" ? block.data.text : "";
  const html = renderMarkdown(text);
</script>

{#if html}
  <div data-nightfire-block="markdown">
    {@html html}
  </div>
{/if}
