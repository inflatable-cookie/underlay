<script lang="ts">
  import { NightfireRenderer, type NightfireValue } from "../nightfire";
  import { marked } from "marked";

  interface Props {
    title: string;
    value?: NightfireValue | string | null;
    /** When true, string values are parsed as markdown instead of raw HTML */
    markdown?: boolean;
    emptyMessage?: string;
    /** Max height in pixels when collapsed. Set to 0 to disable collapsing. */
    maxHeight?: number;
  }

  let {
    title,
    value = null,
    markdown = false,
    emptyMessage = "No content set.",
    maxHeight = 200
  }: Props = $props();

  const isNightfire = $derived(value !== null && typeof value === "object");

  const hasContent = $derived(
    value &&
      (typeof value === "string"
        ? value.trim().length > 0
        : (Array.isArray(value.blocks) && value.blocks.length > 0) || value.block)
  );

  const renderedHtml = $derived(
    typeof value === "string" && markdown
      ? marked.parse(value, { async: false }) as string
      : value
  );

  let isExpanded = $state(false);
  let contentEl: HTMLDivElement | null = $state(null);
  let isOverflowing = $state(false);

  $effect(() => {
    if (!contentEl || maxHeight <= 0) {
      isOverflowing = false;
      return;
    }

    // Check if content overflows the max height
    const checkOverflow = () => {
      if (contentEl) {
        isOverflowing = contentEl.scrollHeight > maxHeight;
      }
    };

    checkOverflow();

    // Recheck on resize
    const observer = new ResizeObserver(checkOverflow);
    observer.observe(contentEl);

    return () => observer.disconnect();
  });

  function toggleExpanded() {
    isExpanded = !isExpanded;
  }
</script>

<div class="content-card">
  <h4 class="content-card__title">{title}</h4>
  <div
    class="content-card__body"
    class:content-card__body--collapsed={!isExpanded && isOverflowing && maxHeight > 0}
    style:--content-card-max-height="{maxHeight}px"
    bind:this={contentEl}
  >
    {#if hasContent}
      {#if isNightfire}
        <NightfireRenderer value={value as NightfireValue} />
      {:else}
        {@html renderedHtml}
      {/if}
    {:else}
      <p class="content-card__empty">{emptyMessage}</p>
    {/if}
  </div>
  {#if isOverflowing && maxHeight > 0}
    <button
      type="button"
      class="content-card__toggle"
      onclick={toggleExpanded}
    >
      {isExpanded ? "Show less" : "Show more"}
    </button>
  {/if}
</div>

<style>
  .content-card {
    max-width: 65ch;
    padding: 1.25rem 1.5rem;
    background: var(--underlay-color-surface-muted, rgba(255, 255, 255, 0.02));
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.2));
    border-radius: var(--underlay-radius-lg, 0.75rem);
  }

  .content-card__title {
    margin: 0 0 0.75rem;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
  }

  .content-card__body {
    margin: 0;
    line-height: 1.6;
    color: var(--underlay-color-text, #e5e7eb);
  }

  .content-card__body--collapsed {
    max-height: var(--content-card-max-height, 200px);
    overflow: hidden;
    mask-image: linear-gradient(to bottom, black 60%, transparent 100%);
    -webkit-mask-image: linear-gradient(to bottom, black 60%, transparent 100%);
  }

  .content-card__body :global(p:first-child) {
    margin-top: 0;
  }

  .content-card__body :global(p:last-child) {
    margin-bottom: 0;
  }

  .content-card__body :global(h1),
  .content-card__body :global(h2),
  .content-card__body :global(h3),
  .content-card__body :global(h4),
  .content-card__body :global(h5),
  .content-card__body :global(h6) {
    margin-top: 1.25em;
    margin-bottom: 0.5em;
    font-weight: 600;
    line-height: 1.3;
  }

  .content-card__body :global(h1:first-child),
  .content-card__body :global(h2:first-child),
  .content-card__body :global(h3:first-child),
  .content-card__body :global(h4:first-child),
  .content-card__body :global(h5:first-child),
  .content-card__body :global(h6:first-child) {
    margin-top: 0;
  }

  .content-card__body :global(ul),
  .content-card__body :global(ol) {
    padding-left: 1.5em;
    margin: 0.75em 0;
  }

  .content-card__body :global(li) {
    margin: 0.25em 0;
  }

  .content-card__body :global(code) {
    font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Monaco, Consolas, monospace;
    font-size: 0.9em;
    background: rgba(148, 163, 184, 0.1);
    border: 1px solid rgba(148, 163, 184, 0.15);
    border-radius: 0.25rem;
    padding: 0.1em 0.4em;
  }

  .content-card__body :global(pre) {
    background: rgba(0, 0, 0, 0.2);
    border-radius: 0.5rem;
    padding: 1rem;
    overflow-x: auto;
    margin: 0.75em 0;
  }

  .content-card__body :global(pre code) {
    background: none;
    border: none;
    padding: 0;
  }

  .content-card__body :global(blockquote) {
    border-left: 3px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.3));
    padding-left: 1rem;
    margin: 0.75em 0;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
  }

  .content-card__empty {
    margin: 0;
    font-size: 0.9em;
    font-style: italic;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.7));
  }

  .content-card__toggle {
    display: block;
    margin-top: 0.75rem;
    padding: 0;
    background: none;
    border: none;
    font-size: 0.85rem;
    font-weight: 500;
    color: var(--underlay-color-primary, #3b82f6);
    cursor: pointer;
    transition: color 0.15s ease;
  }

  .content-card__toggle:hover {
    color: var(--underlay-color-primary-hover, #2563eb);
  }

  .content-card__toggle:focus-visible {
    outline: 2px solid var(--underlay-color-primary, #3b82f6);
    outline-offset: 2px;
    border-radius: 2px;
  }
</style>
