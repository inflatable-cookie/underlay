<script lang="ts">
  import type { Snippet } from "svelte";
  import { tick } from "svelte";

  interface Props {
    /** Whether the drawer is open (bindable) */
    open?: boolean;
    /** Panel header title */
    title?: string;
    /** Slide direction */
    position?: "right" | "left";
    /** Panel width (CSS value) */
    width?: string;
    /** Whether to show backdrop overlay (default: auto — true on mobile, false on desktop) */
    overlay?: boolean | "auto";
    /** Close callback */
    onclose?: () => void;
    /** Panel content */
    children?: Snippet;
    /** Extra header content (e.g., action buttons) */
    headerActions?: Snippet;
    /** Additional CSS class */
    class?: string;
  }

  let {
    open = $bindable(false),
    title = undefined,
    position = "right",
    width = "28rem",
    overlay = "auto",
    onclose = undefined,
    children,
    headerActions,
    class: className = ""
  }: Props = $props();

  let panelEl = $state<HTMLDivElement | null>(null);
  let previousFocusEl = $state<HTMLElement | null>(null);

  function close() {
    open = false;
    onclose?.();
  }

  function handleBackdropClick() {
    close();
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape" && open) {
      event.preventDefault();
      close();
    }
  }

  // Focus management
  $effect(() => {
    if (open) {
      previousFocusEl = document.activeElement as HTMLElement;
      tick().then(() => {
        panelEl?.focus();
      });
    } else if (previousFocusEl) {
      previousFocusEl.focus();
      previousFocusEl = null;
    }
  });
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- Backdrop (mobile overlay or explicit overlay mode) -->
  <div
    class="underlay-drawer-backdrop {overlay === true ? 'underlay-drawer-backdrop--force' : ''}"
    class:underlay-drawer-backdrop--visible={open}
    onclick={handleBackdropClick}
    role="presentation"
  ></div>
{/if}

<div
  bind:this={panelEl}
  class="underlay-drawer underlay-drawer--{position} {className}"
  class:underlay-drawer--open={open}
  class:underlay-drawer--overlay-force={overlay === true}
  style:--drawer-width={width}
  role="complementary"
  aria-label={title ?? "Side panel"}
  tabindex="-1"
>
  {#if title || headerActions}
    <div class="underlay-drawer__header">
      {#if title}
        <h3 class="underlay-drawer__title">{title}</h3>
      {/if}
      <div class="underlay-drawer__header-actions">
        {#if headerActions}
          {@render headerActions()}
        {/if}
        <button
          class="underlay-drawer__close"
          onclick={close}
          aria-label="Close panel"
          type="button"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M18 6L6 18" /><path d="M6 6l12 12" />
          </svg>
        </button>
      </div>
    </div>
  {/if}

  <div class="underlay-drawer__content">
    {#if children}
      {@render children()}
    {/if}
  </div>
</div>

<style>
  /* Backdrop */
  :global(.underlay-drawer-backdrop) {
    display: none;
    position: fixed;
    inset: 0;
    background-color: rgba(0, 0, 0, 0.4);
    z-index: 105;
    opacity: 0;
    transition: opacity 0.25s ease;
  }

  :global(.underlay-drawer-backdrop--visible) {
    opacity: 1;
  }

  /* Show backdrop on mobile by default */
  @media (max-width: 900px) {
    :global(.underlay-drawer-backdrop) {
      display: block;
    }
  }

  /* Force backdrop on all sizes when overlay="true" */
  :global(.underlay-drawer-backdrop--force) {
    display: block;
  }

  /* Panel */
  :global(.underlay-drawer) {
    position: absolute;
    top: 0;
    bottom: 0;
    width: var(--drawer-width, 28rem);
    max-width: 90vw;
    background-color: var(--underlay-color-surface, #1a1a2e);
    display: flex;
    flex-direction: column;
    transition: transform 0.25s ease;
    outline: none;
    overflow: hidden;
  }

  :global(.underlay-drawer--right) {
    right: 0;
    border-left: 1px solid var(--underlay-color-border, rgba(148, 163, 184, 0.12));
    transform: translateX(100%);
    box-shadow: -4px 0 12px rgba(0, 0, 0, 0.15);
  }

  :global(.underlay-drawer--left) {
    left: 0;
    border-right: 1px solid var(--underlay-color-border, rgba(148, 163, 184, 0.12));
    transform: translateX(-100%);
    box-shadow: 4px 0 12px rgba(0, 0, 0, 0.15);
  }

  :global(.underlay-drawer--open.underlay-drawer--right),
  :global(.underlay-drawer--open.underlay-drawer--left) {
    transform: translateX(0);
  }

  /* Mobile: fixed overlay */
  @media (max-width: 900px) {
    :global(.underlay-drawer) {
      position: fixed;
      z-index: 110;
    }
  }

  /* Forced overlay mode */
  :global(.underlay-drawer--overlay-force) {
    position: fixed;
    z-index: 110;
  }

  /* Header */
  :global(.underlay-drawer__header) {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.75rem 1rem;
    border-bottom: 1px solid var(--underlay-color-border, rgba(148, 163, 184, 0.12));
    flex-shrink: 0;
  }

  :global(.underlay-drawer__title) {
    margin: 0;
    font-size: var(--underlay-font-size-sm, 0.875rem);
    font-weight: 600;
    color: var(--underlay-color-text, inherit);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  :global(.underlay-drawer__header-actions) {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  :global(.underlay-drawer__close) {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.75rem;
    height: 1.75rem;
    padding: 0;
    border: none;
    border-radius: var(--underlay-radius-sm, 0.25rem);
    background: transparent;
    color: var(--underlay-color-text-muted, #64748b);
    cursor: pointer;
    transition: background-color 0.12s ease, color 0.12s ease;
  }

  :global(.underlay-drawer__close:hover) {
    background-color: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
    color: var(--underlay-color-text, inherit);
  }

  :global(.underlay-drawer__close:focus-visible) {
    outline: 2px solid var(--underlay-color-primary, #2563eb);
    outline-offset: 2px;
  }

  /* Content */
  :global(.underlay-drawer__content) {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
  }

  /* Reduce motion */
  @media (prefers-reduced-motion: reduce) {
    :global(.underlay-drawer) {
      transition: none;
    }
    :global(.underlay-drawer-backdrop) {
      transition: none;
    }
  }
</style>
