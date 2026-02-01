<script lang="ts">
  import type { Snippet } from "svelte";
  import Trash2 from "lucide-svelte/icons/trash-2";

  interface Props {
    /** Primary text to display */
    label: string;
    /** Optional href for the item */
    href?: string | null;
    /** Click handler (used when no href) */
    onclick?: ((event: MouseEvent) => void) | null;
    /** Optional accent color for the indicator dot */
    accent?: string | null;
    /** Badge content rendered immediately after the label */
    badge?: Snippet;
    /** Trailing content (badges, pills, etc.) - pushed to the right */
    trailing?: Snippet;
    /** Whether to show delete button on hover */
    showDelete?: boolean;
    /** Delete handler */
    ondelete?: () => void;
  }

  let {
    label,
    href = null,
    onclick = null,
    accent = null,
    badge,
    trailing,
    showDelete = false,
    ondelete
  }: Props = $props();

  function handleDelete(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();
    ondelete?.();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      if (onclick) {
        onclick(e as unknown as MouseEvent);
      }
    }
  }
</script>

<li class="inline-list-item" class:inline-list-item--has-delete={showDelete && ondelete}>
  {#if href}
    <a
      class="inline-list-item__content"
      {href}
      onclick={onclick ?? undefined}
    >
      <span
        class="inline-list-item__dot"
        style:--inline-list-item-accent={accent}
      ></span>
      <span class="inline-list-item__label">{label}</span>
      {#if badge}
        <span class="inline-list-item__badge">
          {@render badge()}
        </span>
      {/if}
      {#if trailing}
        <span class="inline-list-item__trailing">
          {@render trailing()}
        </span>
      {/if}
    </a>
  {:else if onclick}
    <div
      class="inline-list-item__content inline-list-item__content--clickable"
      role="button"
      tabindex={0}
      onclick={onclick}
      onkeydown={handleKeydown}
    >
      <span
        class="inline-list-item__dot"
        style:--inline-list-item-accent={accent}
      ></span>
      <span class="inline-list-item__label">{label}</span>
      {#if badge}
        <span class="inline-list-item__badge">
          {@render badge()}
        </span>
      {/if}
      {#if trailing}
        <span class="inline-list-item__trailing">
          {@render trailing()}
        </span>
      {/if}
    </div>
  {:else}
    <div class="inline-list-item__content">
      <span
        class="inline-list-item__dot"
        style:--inline-list-item-accent={accent}
      ></span>
      <span class="inline-list-item__label">{label}</span>
      {#if badge}
        <span class="inline-list-item__badge">
          {@render badge()}
        </span>
      {/if}
      {#if trailing}
        <span class="inline-list-item__trailing">
          {@render trailing()}
        </span>
      {/if}
    </div>
  {/if}

  {#if showDelete && ondelete}
    <button
      type="button"
      class="inline-list-item__delete"
      onclick={handleDelete}
      aria-label="Delete {label}"
    >
      <Trash2 size={14} />
    </button>
  {/if}
</li>

<style>
  .inline-list-item {
    position: relative;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin: 0;
    min-width: 0;
    border-radius: var(--underlay-radius-sm, 0.375rem);
    transition: background-color 0.1s ease;
  }

  .inline-list-item:hover {
    background: var(--underlay-color-surface-hover, rgba(255, 255, 255, 0.04));
  }

  .inline-list-item__content {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 0.625rem;
    padding: 0.5rem 0.625rem;
    min-width: 0;
    text-decoration: none;
    color: inherit;
    border-radius: var(--underlay-radius-sm, 0.375rem);
  }

  .inline-list-item__content--clickable {
    cursor: pointer;
  }

  a.inline-list-item__content:focus-visible,
  .inline-list-item__content--clickable:focus-visible {
    outline: 2px solid var(--underlay-color-primary, #3b82f6);
    outline-offset: -2px;
  }

  .inline-list-item__dot {
    --inline-list-item-accent: var(--underlay-color-primary, #3b82f6);

    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--inline-list-item-accent);
    flex-shrink: 0;
  }

  .inline-list-item__label {
    flex: 1;
    min-width: 0;
    font-size: 0.9rem;
    font-weight: 500;
    color: var(--underlay-color-text, #e5e7eb);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .inline-list-item__badge {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    flex-shrink: 0;
  }

  .inline-list-item__trailing {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    flex-shrink: 0;
    margin-left: auto;
    transition: margin-right 0.1s ease;
  }

  /* Make room for delete button on hover when delete is enabled */
  .inline-list-item--has-delete:hover .inline-list-item__trailing {
    margin-right: 2rem;
  }

  .inline-list-item__delete {
    position: absolute;
    right: 0.375rem;
    top: 50%;
    transform: translateY(-50%);
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    padding: 0;
    border: none;
    border-radius: var(--underlay-radius-sm, 0.375rem);
    background: transparent;
    color: var(--underlay-color-text-muted, #9ca3af);
    cursor: pointer;
    opacity: 0;
    transition: opacity 0.1s ease, background-color 0.1s ease, color 0.1s ease;
  }

  .inline-list-item:hover .inline-list-item__delete {
    opacity: 1;
  }

  .inline-list-item__delete:hover {
    background: var(--underlay-color-danger-muted, rgba(239, 68, 68, 0.15));
    color: var(--underlay-color-danger, #ef4444);
  }

  .inline-list-item__delete:focus-visible {
    opacity: 1;
    outline: 2px solid var(--underlay-color-danger, #ef4444);
    outline-offset: -2px;
  }
</style>
