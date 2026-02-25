<script lang="ts">
  import { marked } from "marked";
  import { sanitizeHtml } from "../../utils/html.js";

  type MarkdownBlock = {
    data?: {
      text?: string;
    };
  };

  interface Props {
    block: MarkdownBlock;
  }

  let { block }: Props = $props();

  const text = $derived(typeof block?.data?.text === "string" ? block.data.text : "");
  const html = $derived(text.trim() ? (marked.parse(text, { async: false }) as string) : "");
  const safeHtml = $derived(sanitizeHtml(html));
</script>

{#if safeHtml}
  <div data-nightfire-block="markdown">
    {@html safeHtml}
  </div>
{/if}
