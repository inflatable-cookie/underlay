<script lang="ts">
  import UnderlayMarkdownEditor from "../../components/MarkdownEditor.svelte";

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
  }

  let { block, onChange = () => {} }: Props = $props();

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

<div class="markdown-editor">
  <UnderlayMarkdownEditor value={text} onChange={handleInput} />
</div>

<style>
  .markdown-editor {
    display: grid;
  }
</style>
