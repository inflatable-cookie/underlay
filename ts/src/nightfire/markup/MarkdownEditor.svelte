<script lang="ts">
  import MarkdownEditorSurface from "./MarkdownEditorSurface.svelte";
  import type { MarkdownEditorContext } from "./markdown-editor-context";

  type MarkdownBlock = {
    type?: string;
    version?: string;
    hash?: string;
    data?: {
      text?: string;
    };
  };

  interface Props {
    block: MarkdownBlock;
    onChange?: (next: MarkdownBlock) => void;
    onContextChange?: ((context: MarkdownEditorContext) => void) | null;
  }

  let { block, onChange = () => {}, onContextChange = null }: Props = $props();

  // Keep local editor text in sync with incoming block updates.
  let text = $state("");

  $effect(() => {
    const next = block.data?.text ?? "";
    if (next !== text) {
      text = next;
    }
  });

  function handleInput(next: string) {
    text = next;
    onChange({
      type: block.type ?? "markdown",
      version: block.version ?? "initial",
      hash: block.hash ?? "",
      data: {
        text: next
      }
    });
  }
</script>

<div class="underlay-markdown-editor">
  <MarkdownEditorSurface value={text} onChange={handleInput} {onContextChange} />
</div>

<style>
  .underlay-markdown-editor {
    display: grid;
  }
</style>
