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

  export let block: MarkdownBlock;
  export let onChange: (next: MarkdownBlock) => void = () => {};

  const current = {
    type: block.type ?? "markdown",
    version: block.version ?? "initial",
    hash: block.hash ?? "",
    data: {
      text: block.data?.text ?? ""
    }
  };

  let text = current.data.text;

  function handleInput(next: string) {
    text = next;
    onChange({
      ...current,
      data: {
        text
      }
    });
  }
</script>

<div class="markdown-editor">
  <UnderlayMarkdownEditor bind:value={text} onChange={handleInput} />
</div>

<style>
  .markdown-editor {
    display: grid;
  }
</style>
