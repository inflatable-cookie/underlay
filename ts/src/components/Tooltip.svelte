<script lang="ts">
  import { Tooltip as BitsTooltip } from "bits-ui";

  export let open = false;

  export let content: string;

  export let showTrigger = true;
  export let triggerLabel = "ⓘ";

  export let side: "top" | "right" | "bottom" | "left" = "top";
  export let sideOffset = 6;
  export let align: "start" | "center" | "end" = "center";
  export let alignOffset = 0;

  export let delayDuration = 500;
  export let disabled = false;
</script>

<BitsTooltip.Root bind:open {delayDuration} {disabled}>
  {#if showTrigger}
    <BitsTooltip.Trigger
      {...$$restProps}
      class={`underlay-tooltip-trigger ${$$restProps.class ?? ""}`}
      aria-label={content}
    >
      <slot name="trigger">{triggerLabel}</slot>
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
      <BitsTooltip.Arrow class="underlay-tooltip-arrow" />
    </BitsTooltip.Content>
  </BitsTooltip.Portal>
</BitsTooltip.Root>

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
    color: var(--underlay-color-text, var(--underlay-color-text, #e5e7eb));
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

  :global(.underlay-tooltip-content) {
    z-index: 80;
    border-radius: 0.6rem;
    border: 1px solid
      var(
        --underlay-color-border-subtle,
        var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.5))
      );
    background: var(
      --underlay-color-bg-surface,
      var(--underlay-color-bg-surface, #020617)
    );
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);
    padding: 0.4rem 0.55rem;
    max-width: min(28rem, calc(100vw - 2rem));
    font-size: 0.85rem;
    line-height: 1.25;
    color: var(--underlay-color-text, var(--underlay-color-text, #e5e7eb));
  }

  :global(.underlay-tooltip-arrow) {
    fill: var(
      --underlay-color-bg-surface,
      var(--underlay-color-bg-surface, #020617)
    );
    stroke: var(
      --underlay-color-border-subtle,
      var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.5))
    );
    stroke-width: 1;
  }
</style>
