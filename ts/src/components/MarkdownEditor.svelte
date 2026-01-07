<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { createEventDispatcher } from "svelte";
  import type { Snippet } from "svelte";
  import { lazyLoadEasyMde } from "./lazy-load-easymde";

  const dispatch = createEventDispatcher<{ change: string }>();

  export let label: string | null = null;
  export let hint: string | null = null;
  export let name: string | null = null;
  export let value: string | null = null;

  export let required: boolean = false;

  export let loading: boolean = false;

  export let showPreview: boolean = true;

  export let className = "";

  export let children: Snippet | null = null;

  export let placeholder: string | null = null;

  export let onChange: ((next: string) => void) | null = null;

  let textareaElement: HTMLTextAreaElement | null = null;
  let editorInstance: any = null;

  function handleChange(next: string) {
    value = next;
    onChange?.(next);
    dispatch("change", next);
  }

  onMount(async () => {
    const isBrowser = typeof window !== "undefined";
    if (!isBrowser) return;

    // If we are not showing the editor or are in a loading state, skip setup.
    if (!showPreview || loading) return;

    const EasyMDE = await lazyLoadEasyMde();

    if (!textareaElement) return;

    editorInstance = new EasyMDE({
      element: textareaElement,
      autoDownloadFontAwesome: false,
      status: false,
      spellChecker: false,
      initialValue: value ?? "",
      placeholder: placeholder ?? undefined,
      toolbar: [
        "bold",
        "italic",
        "heading",
        "|",
        "quote",
        "unordered-list",
        "ordered-list",
        "|",
        "link",
        "image",
        "|",
        "preview",
        "side-by-side",
        "fullscreen"
      ]
    });

    editorInstance.codemirror.on("change", () => {
      const next = editorInstance?.value?.() ?? "";
      handleChange(next);
    });
  });

  onDestroy(() => {
    if (editorInstance) {
      editorInstance.toTextArea();
      editorInstance = null;
    }
  });
</script>

<div
  class={`markdown-editor-root ${showPreview ? "" : "preview-hidden"} ${className}`}
>
  {#if loading}
    <div class="markdown-editor-spinner">
      <span class="spinner-dot" aria-hidden="true"></span>
      <span>Loading markdown editor…</span>
    </div>
  {/if}

  {#if label}
    <label class="markdown-editor-label">
      <span class="markdown-editor-label__text">{label}</span>
      <textarea
        bind:this={textareaElement}
        class="markdown-editor-textarea"
        class:is-hidden={showPreview && !loading}
        name={name ?? undefined}
        bind:value
        required={required}
        placeholder={placeholder ?? undefined}
      ></textarea>
    </label>
  {:else}
    <textarea
      bind:this={textareaElement}
      class="markdown-editor-textarea"
      class:is-hidden={showPreview && !loading}
      name={name ?? undefined}
      bind:value
      required={required}
      placeholder={placeholder ?? undefined}
    ></textarea>
  {/if}

  {#if hint}
    <div class="markdown-editor-help">{hint}</div>
  {/if}

  {#if children}
    {@render children()}
  {/if}
</div>

<style>
  .markdown-editor-root {
    display: grid;
    gap: var(--underlay-space-1, var(--froyo-space-1, 0.25rem));
    padding-top: var(--underlay-space-1, var(--froyo-space-1, 0.25rem));
  }

  .markdown-editor-label {
    display: grid;
    gap: var(--underlay-space-1, var(--froyo-space-1, 0.25rem));
  }

  .markdown-editor-label__text {
    font-size: var(--underlay-font-size-sm, calc(1em * 0.9));
    font-weight: 500;
  }

  .markdown-editor-textarea {
    width: 100%;
    min-height: var(
      --underlay-markdown-editor-min-height,
      var(--froyo-markdown-editor-min-height, 12rem)
    );
    padding: var(--underlay-field-padding-block, var(--froyo-field-padding-block, 0.55em))
      var(--underlay-field-padding-inline, var(--froyo-field-padding-inline, 0.7em));
    border-radius: var(--underlay-radius-sm, var(--froyo-radius-sm, 0.35rem));
    border: none;
    background: var(
      --underlay-color-field-bg,
      var(--froyo-color-field-bg, rgba(148, 163, 184, 0.18))
    );
    color: var(--underlay-color-text, var(--froyo-color-text, inherit));
    font-size: var(--underlay-font-size-sm, calc(1em * 0.9));
    resize: vertical;
  }

  .markdown-editor-textarea:focus,
  .markdown-editor-textarea:focus-visible {
    outline: var(--underlay-focus-ring-width, var(--froyo-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--froyo-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--froyo-focus-ring-offset, 2px));
  }

  .markdown-editor-textarea.is-hidden {
    display: none;
  }

  .markdown-editor-help {
    font-size: var(--underlay-font-size-xs, calc(1em * 0.8));
    opacity: 0.8;
  }

  .markdown-editor-spinner {
    display: flex;
    align-items: center;
    gap: var(--underlay-space-2, var(--froyo-space-2, 0.5rem));
    font-size: var(--underlay-font-size-xs, calc(1em * 0.8));
    color: var(--underlay-color-text-muted, var(--froyo-color-text-muted, #9ca3af));
    margin-bottom: var(--underlay-stack-tight-margin, var(--froyo-stack-tight-margin, 0.5rem));
    width: 100%;
    min-height: var(
      --underlay-markdown-editor-min-height,
      var(--froyo-markdown-editor-min-height, 12rem)
    );
    padding: var(--underlay-field-padding-block, var(--froyo-field-padding-block, 0.55em))
      var(--underlay-field-padding-inline, var(--froyo-field-padding-inline, 0.7em));
    border-radius: var(--underlay-radius-sm, var(--froyo-radius-sm, 0.35rem));
    border: none;
    background: var(
      --underlay-color-field-bg,
      var(--froyo-color-field-bg, rgba(148, 163, 184, 0.18))
    );
  }

  .markdown-editor-spinner .spinner-dot {
    width: var(--underlay-markdown-editor-spinner-dot-size, var(--froyo-markdown-editor-spinner-dot-size, 0.8rem));
    height: var(--underlay-markdown-editor-spinner-dot-size, var(--froyo-markdown-editor-spinner-dot-size, 0.8rem));
    border-radius: var(--underlay-radius-pill, var(--froyo-radius-pill, 999px));
    border: var(--underlay-focus-ring-width, var(--froyo-focus-ring-width, 2px)) solid
      var(
        --underlay-color-border-strong,
        var(--froyo-color-border-strong, rgba(148, 163, 184, 0.7))
      );
    border-top-color: transparent;
    animation: md-spin 0.9s linear infinite;
  }

  @keyframes md-spin {
    to {
      transform: rotate(360deg);
    }
  }

  /* Style the EasyMDE / CodeMirror surface */
  .markdown-editor-root :global(.EasyMDEContainer) {
    border-radius: var(--underlay-radius-sm, var(--froyo-radius-sm, 0.35rem));
  }

  .markdown-editor-root :global(.EasyMDEContainer .CodeMirror) {
    background: var(
      --underlay-color-field-bg,
      var(--froyo-color-field-bg, rgba(148, 163, 184, 0.18))
    );
    color: var(--underlay-color-text, var(--froyo-color-text, inherit));
    border-radius: var(--underlay-radius-sm, var(--froyo-radius-sm, 0.35rem));
    border: none;
    padding: var(--underlay-field-padding-block, var(--froyo-field-padding-block, 0.55em))
      calc(
        var(--underlay-field-padding-inline, var(--froyo-field-padding-inline, 0.7em)) -
          var(--underlay-markdown-editor-inline-nudge, var(--froyo-markdown-editor-inline-nudge, 0.35rem))
      );
    font-size: var(--underlay-font-size-sm, calc(1em * 0.9));
    min-height: var(
      --underlay-markdown-editor-min-height,
      var(--froyo-markdown-editor-min-height, 12rem)
    );
  }

  .markdown-editor-root
    :global(.EasyMDEContainer .CodeMirror.CodeMirror-focused) {
    outline: var(--underlay-focus-ring-width, var(--froyo-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--froyo-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--froyo-focus-ring-offset, 2px));
  }

  .markdown-editor-root
    :global(.EasyMDEContainer .CodeMirror pre.CodeMirror-line) {
    font-family: inherit;
  }

  .markdown-editor-root
    :global(.EasyMDEContainer .CodeMirror div.CodeMirror-cursor) {
    border-left-color: var(--underlay-color-text, var(--froyo-color-text, inherit));
  }

  .markdown-editor-root
    :global(.EasyMDEContainer .CodeMirror .CodeMirror-selected) {
    background: color-mix(
      in srgb,
      var(--underlay-color-primary, var(--froyo-color-primary, #2563eb)) 35%,
      transparent
    );
  }

  .markdown-editor-root :global(.EasyMDEContainer .editor-preview),
  .markdown-editor-root :global(.EasyMDEContainer .editor-preview-side) {
    background: var(
      --underlay-color-surface-muted,
      var(--froyo-color-surface-muted, rgba(255, 255, 255, 0.02))
    );
    color: var(--underlay-color-text, var(--froyo-color-text, inherit));
  }

  .markdown-editor-root :global(.EasyMDEContainer .editor-toolbar) {
    background: var(
      --underlay-color-surface-muted,
      var(--froyo-color-surface-muted, rgba(255, 255, 255, 0.02))
    );
    border-color: var(
      --underlay-color-border-subtle,
      var(--froyo-color-border-subtle, rgba(148, 163, 184, 0.25))
    );
  }

  .markdown-editor-root.preview-active
    :global(.EasyMDEContainer .editor-preview),
  .markdown-editor-root.preview-active
    :global(.EasyMDEContainer .editor-preview-side) {
    border: 1px solid var(--underlay-color-primary, var(--froyo-color-primary, #2563eb));
    border-radius: var(--underlay-radius-sm, var(--froyo-radius-sm, 0.35rem));
  }
</style>
