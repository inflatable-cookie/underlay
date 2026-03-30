<script lang="ts">
  import { onDestroy, onMount, untrack } from "svelte";
  import type { Snippet } from "svelte";
  import { lazyLoadEasyMde } from "./lazy-load-easymde";
  import type { MarkdownEditorContext } from "./markdown-editor-context";

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

  function emitContext(context: MarkdownEditorContext) {
    onContextChange?.(context);
  }

  function readTextareaContext(textarea: HTMLTextAreaElement): MarkdownEditorContext {
    const nextValue = textarea.value ?? "";
    const selectionStart = textarea.selectionStart ?? nextValue.length;
    const selectionEnd = textarea.selectionEnd ?? selectionStart;

    return {
      value: nextValue,
      selectionStart,
      selectionEnd
    };
  }

  function emitTextareaContext(textarea: HTMLTextAreaElement | null) {
    if (!textarea) {
      return;
    }

    emitContext(readTextareaContext(textarea));
  }

  function emitCodeMirrorContext() {
    const cm = editorInstance?.codemirror;
    if (!cm) {
      return;
    }

    const currentValue = editorInstance?.value?.() ?? "";
    const from = typeof cm.getCursor === "function" ? cm.getCursor("from") : null;
    const to = typeof cm.getCursor === "function" ? cm.getCursor("to") : null;
    const selectionStart =
      from && typeof cm.indexFromPos === "function"
        ? cm.indexFromPos(from)
        : currentValue.length;
    const selectionEnd =
      to && typeof cm.indexFromPos === "function"
        ? cm.indexFromPos(to)
        : selectionStart;

    emitContext({
      value: currentValue,
      selectionStart,
      selectionEnd
    });
  }

  function handleTextareaInput(event: Event) {
    const textarea = event.currentTarget as HTMLTextAreaElement;
    scheduleChange(textarea.value ?? "");
    emitTextareaContext(textarea);
  }

  function handleTextareaSelection(event: Event) {
    emitTextareaContext(event.currentTarget as HTMLTextAreaElement);
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
      autoDownloadFontAwesome: false,
      status: false,
      spellChecker: false,
      minHeight: "var(--underlay-markdown-editor-min-height, 5em)",
      initialValue: value ?? "",
      placeholder: placeholder ?? undefined,
      toolbar: [
        { name: "bold", action: EasyMDE.toggleBold, title: "Bold", icon: icon.bold },
        { name: "italic", action: EasyMDE.toggleItalic, title: "Italic", icon: icon.italic },
        { name: "heading", action: EasyMDE.toggleHeadingSmaller, title: "Heading", icon: icon.heading },
        "|",
        { name: "quote", action: EasyMDE.toggleBlockquote, title: "Quote", icon: icon.quote },
        { name: "unordered-list", action: EasyMDE.toggleUnorderedList, title: "Bulleted list", icon: icon.list },
        { name: "ordered-list", action: EasyMDE.toggleOrderedList, title: "Numbered list", icon: icon.listOrdered },
        "|",
        { name: "link", action: EasyMDE.drawLink, title: "Link", icon: icon.link },
        { name: "image", action: EasyMDE.drawImage, title: "Image", icon: icon.image },
        "|",
        { name: "preview", action: EasyMDE.togglePreview, title: "Preview", icon: icon.preview },
        { name: "side-by-side", action: EasyMDE.toggleSideBySide, title: "Side by side", icon: icon.sideBySide },
        { name: "fullscreen", action: EasyMDE.toggleFullScreen, title: "Fullscreen", icon: icon.fullscreen }
      ]
    });

    editorInstance.codemirror.on("change", () => {
      const next = editorInstance?.value?.() ?? "";
      scheduleChange(next);
      emitCodeMirrorContext();
    });

    editorInstance.codemirror.on("cursorActivity", () => {
      emitCodeMirrorContext();
    });

    editorReady = true;
    emitCodeMirrorContext();
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
    const shouldInit = mounted && showPreview && !loading;

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
  class={`underlay-markdown-editor-root ${showPreview ? "" : "underlay-preview-hidden"} ${className}`}
>
  {#if loading || (showPreview && !editorReady)}
    <div class="underlay-markdown-editor-spinner">
      <span class="underlay-spinner-dot" aria-hidden="true"></span>
      <span>Loading markdown editor...</span>
    </div>
  {/if}

  {#if label}
    <label class="underlay-markdown-editor-label">
      <span class="underlay-markdown-editor-label__text">{label}</span>
      <textarea
        bind:this={textareaElement}
        class="underlay-markdown-editor-textarea"
        class:underlay-is-hidden={showPreview && editorReady}
        name={name ?? undefined}
        bind:value
        required={required}
        placeholder={placeholder ?? undefined}
        oninput={handleTextareaInput}
        onclick={handleTextareaSelection}
        onkeyup={handleTextareaSelection}
        onselect={handleTextareaSelection}
      ></textarea>
    </label>
  {:else}
    <textarea
      bind:this={textareaElement}
      class="underlay-markdown-editor-textarea"
      class:underlay-is-hidden={showPreview && editorReady}
      name={name ?? undefined}
      bind:value
      required={required}
      placeholder={placeholder ?? undefined}
      oninput={handleTextareaInput}
      onclick={handleTextareaSelection}
      onkeyup={handleTextareaSelection}
      onselect={handleTextareaSelection}
    ></textarea>
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

  .underlay-markdown-editor-textarea {
    width: 100%;
    min-height: var(--underlay-markdown-editor-min-height, 5em);
    resize: vertical;
  }

  .underlay-is-hidden {
    display: none;
  }

  .underlay-markdown-editor-help {
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.8));
    font-size: 0.875rem;
  }

  .underlay-markdown-editor-spinner {
    display: inline-flex;
    align-items: center;
    gap: var(--underlay-space-2, 0.5rem);
  }

  .underlay-spinner-dot {
    width: 0.625rem;
    height: 0.625rem;
    border-radius: 999px;
    background: currentColor;
    opacity: 0.7;
  }
</style>
