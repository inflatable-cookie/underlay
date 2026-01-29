<script lang="ts">
  import type { Snippet } from "svelte";
  import { Tooltip as BitsTooltip } from "bits-ui";

  interface Props {
    open?: boolean;
    content: string;
    showTrigger?: boolean;
    triggerLabel?: string;
    side?: "top" | "right" | "bottom" | "left";
    sideOffset?: number;
    align?: "start" | "center" | "end";
    alignOffset?: number;
    delayDuration?: number;
    disabled?: boolean;
    /** Use inline mode for text content (inherits font size, no fixed dimensions) */
    inline?: boolean;
    trigger?: Snippet;
    class?: string;
  }

  let {
    open = $bindable(false),
    content,
    showTrigger = true,
    triggerLabel = "ⓘ",
    side = "top",
    sideOffset = 0,
    align = "center",
    alignOffset = 0,
    delayDuration = 500,
    disabled = false,
    inline = false,
    trigger,
    class: className,
  }: Props = $props();

  const triggerClass = $derived(
    inline || trigger
      ? `underlay-tooltip-trigger--inline ${className ?? ""}`
      : `underlay-tooltip-trigger ${className ?? ""}`
  );
</script>

<BitsTooltip.Provider>
  <BitsTooltip.Root bind:open {delayDuration} {disabled}>
    {#if showTrigger}
      <BitsTooltip.Trigger
        class={triggerClass}
        aria-label={content}
      >
        {#if trigger}
          {@render trigger()}
        {:else}
          {triggerLabel}
        {/if}
      </BitsTooltip.Trigger>
    {/if}

    <BitsTooltip.Portal>
      <BitsTooltip.Content
        class="underlay-tooltip-content"
        {side}
        {sideOffset}
        {align}
        {alignOffset}
      >
        {content}
        <BitsTooltip.Arrow class="underlay-tooltip-arrow" width={8} height={6} />
      </BitsTooltip.Content>
    </BitsTooltip.Portal>
  </BitsTooltip.Root>
</BitsTooltip.Provider>

<style>
  :global(.underlay-tooltip-trigger) {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.7rem;
    height: 1.7rem;
    border-radius: 999px;
    border: 1px solid rgba(148, 163, 184, 0.35);
    background: rgba(255, 255, 255, 0.03);
    color: var(--underlay-color-text, #e5e7eb);
    cursor: pointer;
    font-size: 0.85rem;
    line-height: 1;
  }

  :global(.underlay-tooltip-trigger:hover) {
    background: rgba(148, 163, 184, 0.08);
  }

  :global(.underlay-tooltip-trigger:focus-visible) {
    outline: 2px solid rgba(59, 130, 246, 0.9);
    outline-offset: 2px;
  }

  :global(.underlay-tooltip-trigger--inline) {
    display: inline;
    background: none;
    border: none;
    padding: 0;
    margin: 0;
    width: auto;
    height: auto;
    border-radius: 0;
    cursor: help;
    color: inherit;
    font: inherit;
  }

  :global(.underlay-tooltip-trigger--inline:focus-visible) {
    outline: 2px solid rgba(59, 130, 246, 0.9);
    outline-offset: 2px;
    border-radius: 2px;
  }

  :global(.underlay-tooltip-content) {
    z-index: 80;
    border-radius: 0.4rem;
    border: 1px solid rgba(148, 163, 184, 0.5);
    background: #020617;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);
    padding: 0.25rem 0.5rem;
    max-width: min(28rem, calc(100vw - 2rem));
    font-size: 0.875rem;
    line-height: 1.35;
    color: #e5e7eb;
  }

  :global(.underlay-tooltip-arrow) {
    fill: #020617;
    stroke: rgba(148, 163, 184, 0.5);
    stroke-width: 1;
  }
</style>
