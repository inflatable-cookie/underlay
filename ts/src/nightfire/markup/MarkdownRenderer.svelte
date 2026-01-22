<script lang="ts">
  import { marked } from "marked";

  type MarkdownBlock = {
    data?: {
      text?: string;
    };
  };

  export let block: MarkdownBlock;

  const text =
    typeof block?.data?.text === "string" ? block.data.text : "";
  const html = text.trim() ? (marked.parse(text, { async: false }) as string) : "";
</script>

{#if html}
  <div data-nightfire-block="markdown">
    {@html html}
  </div>
{/if}
