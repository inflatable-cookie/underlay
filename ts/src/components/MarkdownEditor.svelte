<script lang="ts">
  import { onDestroy, onMount, untrack } from "svelte";
  import type { Snippet } from "svelte";
  import { lazyLoadEasyMde } from "./lazy-load-easymde";

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
    onChange = null
  }: Props = $props();

  let textareaElement: HTMLTextAreaElement | null = $state(null);
  let editorInstance: any = $state(null);
  let editorReady = $state(false);
  let mounted = $state(false);

  let initAttempt = $state(0);
  let changeScheduled = $state(false);
  let queuedChange = $state("");

  function handleChange(next: string) {
    value = next;
    onChange?.(next);
  }

  function scheduleChange(next: string) {
    queuedChange = next;
    if (changeScheduled) return;

    changeScheduled = true;
    queueMicrotask(() => {
      changeScheduled = false;
      handleChange(queuedChange);
    });
  }

  async function ensureEditor() {
    const isBrowser = typeof window !== "undefined";
    if (!isBrowser) return;

    if (!showPreview || loading) return;
    if (!textareaElement) return;
    if (editorInstance) return;

    editorReady = false;

    initAttempt += 1;
    const attemptId = initAttempt;

    const EasyMDE = await lazyLoadEasyMde();

    // Component state changed while waiting for dynamic import.
    if (attemptId !== initAttempt) return;
    if (!mounted) return;
    if (!showPreview || loading) return;
    if (!textareaElement) return;

    const icon = {
      bold: '<svg class="underlay-md-icon" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 12h8a4 4 0 0 0 0-8H6z"/><path d="M6 12h9a4 4 0 0 1 0 8H6z"/></svg>',
      italic: '<svg class="underlay-md-icon" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="19" x2="10" y1="4" y2="4"/><line x1="14" x2="5" y1="20" y2="20"/><line x1="15" x2="9" y1="4" y2="20"/></svg>',
      heading: '<svg class="underlay-md-icon" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 12h12"/><path d="M6 20V4"/><path d="M18 20V4"/></svg>',
      quote: '<svg class="underlay-md-icon" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 21c3 0 7-1 7-8V5c0-1.1-.9-2-2-2H5c-1.1 0-2 .9-2 2v6c0 1.1.9 2 2 2h3"/><path d="M14 21c3 0 7-1 7-8V5c0-1.1-.9-2-2-2h-3c-1.1 0-2 .9-2 2v6c0 1.1.9 2 2 2h3"/></svg>',
      list: '<svg class="underlay-md-icon" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="8" x2="21" y1="6" y2="6"/><line x1="8" x2="21" y1="12" y2="12"/><line x1="8" x2="21" y1="18" y2="18"/><line x1="3" x2="3.01" y1="6" y2="6"/><line x1="3" x2="3.01" y1="12" y2="12"/><line x1="3" x2="3.01" y1="18" y2="18"/></svg>',
      listOrdered: '<svg class="underlay-md-icon" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="10" x2="21" y1="6" y2="6"/><line x1="10" x2="21" y1="12" y2="12"/><line x1="10" x2="21" y1="18" y2="18"/><path d="M4 6h1v4"/><path d="M4 10h2"/><path d="M6 18H4c0-1 2-2 2-3s-1-1.5-2-1"/></svg>',
      link: '<svg class="underlay-md-icon" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/><path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/></svg>',
      image: '<svg class="underlay-md-icon" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><circle cx="9" cy="9" r="2"/><path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"/></svg>',
      preview: '<svg class="underlay-md-icon" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M2.062 12.348a1 1 0 0 1 0-.696C3.423 7.51 7.36 4.5 12 4.5s8.577 3.01 9.938 7.152a1 1 0 0 1 0 .696C20.577 16.49 16.64 19.5 12 19.5S3.423 16.49 2.062 12.348"/><circle cx="12" cy="12" r="3"/></svg>',
      sideBySide: '<svg class="underlay-md-icon" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><line x1="12" x2="12" y1="3" y2="21"/></svg>',
      fullscreen: '<svg class="underlay-md-icon" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M8 3H5a2 2 0 0 0-2 2v3"/><path d="M21 8V5a2 2 0 0 0-2-2h-3"/><path d="M3 16v3a2 2 0 0 0 2 2h3"/><path d="M16 21h3a2 2 0 0 0 2-2v-3"/></svg>'
    };

    editorInstance = new EasyMDE({
      element: textareaElement,
      // Avoid relying on a CDN for FontAwesome.
      autoDownloadFontAwesome: false,
      status: false,
      spellChecker: false,
      // Keep JS config aligned with the design token used in CSS.
      minHeight: "var(--underlay-markdown-editor-min-height, 5em)",
      initialValue: value ?? "",
      placeholder: placeholder ?? undefined,
      toolbar: [
        {
          name: "bold",
          action: EasyMDE.toggleBold,
          title: "Bold",
          icon: icon.bold
        },
        {
          name: "italic",
          action: EasyMDE.toggleItalic,
          title: "Italic",
          icon: icon.italic
        },
        {
          name: "heading",
          action: EasyMDE.toggleHeadingSmaller,
          title: "Heading",
          icon: icon.heading
        },
        "|",
        {
          name: "quote",
          action: EasyMDE.toggleBlockquote,
          title: "Quote",
          icon: icon.quote
        },
        {
          name: "unordered-list",
          action: EasyMDE.toggleUnorderedList,
          title: "Bulleted list",
          icon: icon.list
        },
        {
          name: "ordered-list",
          action: EasyMDE.toggleOrderedList,
          title: "Numbered list",
          icon: icon.listOrdered
        },
        "|",
        {
          name: "link",
          action: EasyMDE.drawLink,
          title: "Link",
          icon: icon.link
        },
        {
          name: "image",
          action: EasyMDE.drawImage,
          title: "Image",
          icon: icon.image
        },
        "|",
        {
          name: "preview",
          action: EasyMDE.togglePreview,
          title: "Preview",
          icon: icon.preview
        },
        {
          name: "side-by-side",
          action: EasyMDE.toggleSideBySide,
          title: "Side by side",
          icon: icon.sideBySide
        },
        {
          name: "fullscreen",
          action: EasyMDE.toggleFullScreen,
          title: "Fullscreen",
          icon: icon.fullscreen
        }
      ]
    });

    editorInstance.codemirror.on("change", () => {
      const next = editorInstance?.value?.() ?? "";
      scheduleChange(next);
    });

    editorReady = true;
  }

  function destroyEditor() {
    initAttempt += 1;

    if (editorInstance) {
      editorInstance.toTextArea();
      editorInstance = null;
    }

    editorReady = false;
  }

  onMount(() => {
    mounted = true;
    void ensureEditor();

    return () => {
      mounted = false;
      destroyEditor();
    };
  });

  $effect(() => {
    // Read reactive dependencies first
    const shouldInit = mounted && showPreview && !loading;
    
    // Use untrack to prevent state writes from re-triggering this effect
    untrack(() => {
      if (shouldInit) {
        void ensureEditor();
      } else if (mounted) {
        destroyEditor();
      }
    });
  });

  onDestroy(() => {
    destroyEditor();
  });
</script>

  <div
    class={`markdown-editor-root ${showPreview ? "" : "preview-hidden"} ${className}`}
  >
    {#if loading || (showPreview && !editorReady)}
      <div class="markdown-editor-spinner">
        <span class="spinner-dot" aria-hidden="true"></span>
        <span>Loading markdown editor...</span>
      </div>
    {/if}


  {#if label}
    <label class="markdown-editor-label">
      <span class="markdown-editor-label__text">{label}</span>
      <textarea
        bind:this={textareaElement}
        class="markdown-editor-textarea"
        class:is-hidden={showPreview && editorReady}
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
      class:is-hidden={showPreview && editorReady}
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
    gap: var(--underlay-space-1, var(--underlay-space-1, 0.25rem));
    padding-top: 0;
  }

  .markdown-editor-label {
    display: grid;
    gap: var(--underlay-space-1, var(--underlay-space-1, 0.25rem));
  }

  .markdown-editor-label__text {
    font-size: var(--underlay-font-size-sm, calc(1em * 0.9));
    font-weight: 500;
  }

  .markdown-editor-textarea {
    width: 100%;
    min-height: var(
      --underlay-markdown-editor-min-height,
      var(--underlay-markdown-editor-min-height, 12rem)
    );
    padding: var(--underlay-field-padding-block, var(--underlay-field-padding-block, 0.55em))
      var(--underlay-field-padding-inline, var(--underlay-field-padding-inline, 0.7em));
    border-radius: var(--underlay-radius-sm, var(--underlay-radius-sm, 0.35rem));
    border: none;
    background: var(
      --underlay-color-field-bg,
      var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18))
    );
    color: var(--underlay-color-text, var(--underlay-color-text, inherit));
    font-size: var(--underlay-font-size-sm, calc(1em * 0.9));
    resize: vertical;
  }

  .markdown-editor-textarea:focus,
  .markdown-editor-textarea:focus-visible {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--underlay-focus-ring-offset, 2px));
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
    gap: var(--underlay-space-2, var(--underlay-space-2, 0.5rem));
    font-size: var(--underlay-font-size-xs, calc(1em * 0.8));
    color: var(--underlay-color-text-muted, var(--underlay-color-text-muted, #9ca3af));
    margin-bottom: var(--underlay-stack-tight-margin, var(--underlay-stack-tight-margin, 0.5rem));
    width: 100%;
    min-height: var(
      --underlay-markdown-editor-min-height,
      var(--underlay-markdown-editor-min-height, 12rem)
    );
    padding: var(--underlay-field-padding-block, var(--underlay-field-padding-block, 0.55em))
      var(--underlay-field-padding-inline, var(--underlay-field-padding-inline, 0.7em));
    border-radius: var(--underlay-radius-sm, var(--underlay-radius-sm, 0.35rem));
    border: none;
    background: var(
      --underlay-color-field-bg,
      var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18))
    );
  }

  .markdown-editor-spinner .spinner-dot {
    width: var(--underlay-markdown-editor-spinner-dot-size, var(--underlay-markdown-editor-spinner-dot-size, 0.8rem));
    height: var(--underlay-markdown-editor-spinner-dot-size, var(--underlay-markdown-editor-spinner-dot-size, 0.8rem));
    border-radius: var(--underlay-radius-pill, var(--underlay-radius-pill, 999px));
    border: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(
        --underlay-color-border-strong,
        var(--underlay-color-border-strong, rgba(148, 163, 184, 0.7))
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
    border-radius: var(--underlay-radius-sm, var(--underlay-radius-sm, 0.35rem));
  }

  .markdown-editor-root :global(.EasyMDEContainer .CodeMirror) {
    background: var(
      --underlay-color-field-bg,
      var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18))
    );
    color: var(--underlay-color-text, var(--underlay-color-text, inherit));
    border-radius: var(--underlay-radius-sm, var(--underlay-radius-sm, 0.35rem));
    border: none;
    padding: var(--underlay-field-padding-block, var(--underlay-field-padding-block, 0.55em))
      calc(
        var(--underlay-field-padding-inline, var(--underlay-field-padding-inline, 0.7em)) -
          var(--underlay-markdown-editor-inline-nudge, var(--underlay-markdown-editor-inline-nudge, 0.35rem))
      );
    font-size: var(--underlay-font-size-sm, calc(1em * 0.9));
    min-height: var(
      --underlay-markdown-editor-min-height,
      var(--underlay-markdown-editor-min-height, 12rem)
    );
  }

  .markdown-editor-root
    :global(.EasyMDEContainer .CodeMirror.CodeMirror-focused) {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--underlay-focus-ring-offset, 2px));
  }

  .markdown-editor-root
    :global(.EasyMDEContainer .CodeMirror pre.CodeMirror-line) {
    font-family: inherit;
  }

  .markdown-editor-root
    :global(.EasyMDEContainer .CodeMirror div.CodeMirror-cursor) {
    border-left-color: var(--underlay-color-text, var(--underlay-color-text, inherit));
  }

  .markdown-editor-root
    :global(.EasyMDEContainer .CodeMirror .CodeMirror-selected) {
    background: color-mix(
      in srgb,
      var(--underlay-color-primary, var(--underlay-color-primary, #2563eb)) 35%,
      transparent
    );
  }

  .markdown-editor-root :global(.EasyMDEContainer .editor-preview),
  .markdown-editor-root :global(.EasyMDEContainer .editor-preview-side) {
    background: var(
      --underlay-color-surface-muted,
      var(--underlay-color-surface-muted, rgba(255, 255, 255, 0.02))
    );
    color: var(--underlay-color-text, var(--underlay-color-text, inherit));
  }

  .markdown-editor-root :global(.EasyMDEContainer .editor-toolbar) {
    background: transparent;
    border: none;
  }

  .markdown-editor-root.preview-active
    :global(.EasyMDEContainer .editor-preview),
  .markdown-editor-root.preview-active
    :global(.EasyMDEContainer .editor-preview-side) {
    border: 1px solid var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    border-radius: var(--underlay-radius-sm, var(--underlay-radius-sm, 0.35rem));
  }
</style>
