<script lang="ts">
  import UnderlayMarkdownEditor from "../../components/MarkdownEditor.svelte";
  import { untrack } from "svelte";

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

  // Initialize text from block once - don't use reactive binding
  const initialText = untrack(() => block.data?.text ?? "");

  function handleInput(next: string) {
    // Don't update local state - let the editor manage its own state
    // Just propagate the change up
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
  <UnderlayMarkdownEditor value={initialText} onChange={handleInput} />
</div>

<style>
  .markdown-editor {
    display: grid;
  }
</style>
