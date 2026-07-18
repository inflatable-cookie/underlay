<script lang="ts">
  import { MarkdownEditor } from "@poodle/svelte";
  import type { Snippet } from "svelte";
  import type { MarkdownEditorContext } from "./markdown-editor-context";
  import { renderSafeMarkdownPreview } from "./markdown-preview";

  interface Props {
    label?: string | null;
    hint?: string | null;
    name?: string | null;
    value?: string | null;
    required?: boolean;
    loading?: boolean;
    showPreview?: boolean;
    className?: string;
    children?: Snippet | null;
    placeholder?: string | null;
    onChange?: ((next: string) => void) | null;
    onContextChange?: ((context: MarkdownEditorContext) => void) | null;
  }

  let {
    label = null,
    hint = null,
    name = null,
    value = $bindable(null),
    required = false,
    loading = false,
    showPreview = true,
    className = "",
    children = null,
    placeholder = null,
    onChange = null,
    onContextChange = null
  }: Props = $props();

  function emitContextFromTextarea(textarea: HTMLTextAreaElement | null) {
    if (!textarea) {
      return;
    }

    onContextChange?.({
      value: textarea.value ?? "",
      selectionStart: textarea.selectionStart ?? textarea.value.length,
      selectionEnd: textarea.selectionEnd ?? textarea.selectionStart ?? textarea.value.length
    });
  }

  function handleValueChange(nextValue: string) {
    value = nextValue;
    onChange?.(nextValue);
  }

  function handleTextareaEvent(event: Event) {
    const target = event.target;
    if (!(target instanceof HTMLTextAreaElement)) {
      return;
    }

    emitContextFromTextarea(target);
  }

</script>

<div
  class={`underlay-markdown-editor-root ${className}`.trim()}
  oninput={handleTextareaEvent}
>
  {#if label}
    <label class="underlay-markdown-editor-label">
      <span class="underlay-markdown-editor-label__text">{label}</span>
      <MarkdownEditor
        value={value ?? ""}
        {name}
        {required}
        disabled={loading}
        placeholder={placeholder ?? undefined}
        mode={showPreview ? "split" : "edit"}
        minHeight="5em"
        renderHtml={renderSafeMarkdownPreview}
        onValueChange={handleValueChange}
      />
    </label>
  {:else}
    <MarkdownEditor
      value={value ?? ""}
      {name}
      {required}
      disabled={loading}
      placeholder={placeholder ?? undefined}
      mode={showPreview ? "split" : "edit"}
      minHeight="5em"
      renderHtml={renderSafeMarkdownPreview}
      onValueChange={handleValueChange}
    />
  {/if}

  {#if hint}
    <div class="underlay-markdown-editor-help">{hint}</div>
  {/if}

  {@render children?.()}
</div>

<style>
  .underlay-markdown-editor-root {
    display: grid;
    gap: var(--underlay-space-2, 0.5rem);
  }

  .underlay-markdown-editor-label {
    display: grid;
    gap: var(--underlay-space-1, 0.25rem);
  }

  .underlay-markdown-editor-label__text {
    font-weight: 600;
  }

  .underlay-markdown-editor-help {
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.8));
    font-size: 0.875rem;
  }
</style>
