<script lang="ts">
  import { MarkdownEditor as PoodleMarkdownEditor } from "@inflatable-cookie/poodle-svelte";
  import type { MarkdownEditorContext } from "./markdown-editor-context";
  import { renderSafeMarkdownPreview } from "./markdown-preview";

  type MarkdownBlock = {
    type?: string;
    version?: string;
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

  const text = $derived(block.data?.text ?? "");

  function handleInput(next: string) {
    onChange({
      type: block.type ?? "markdown",
      version: block.version ?? "initial",
      data: {
        text: next
      }
    });
  }

  function handleTextareaEvent(event: Event) {
    const target = event.target;
    if (!(target instanceof HTMLTextAreaElement)) {
      return;
    }

    onContextChange?.({
      value: target.value ?? "",
      selectionStart: target.selectionStart ?? target.value.length,
      selectionEnd: target.selectionEnd ?? target.selectionStart ?? target.value.length
    });
  }

</script>

<div
  class="underlay-markdown-editor"
  oninput={handleTextareaEvent}
>
  <PoodleMarkdownEditor
    value={text}
    placeholder="Write markdown..."
    minHeight="16rem"
    mode="edit"
    renderHtml={renderSafeMarkdownPreview}
    onValueChange={handleInput}
  />
</div>

<style>
  .underlay-markdown-editor {
    display: grid;
  }
</style>
