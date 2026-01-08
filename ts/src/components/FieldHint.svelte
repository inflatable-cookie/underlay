<script lang="ts">
  import Popover from "./Popover.svelte";

  interface Props {
    content: string;

    side?: "top" | "right" | "bottom" | "left";
    sideOffset?: number;
    align?: "start" | "center" | "end";
    alignOffset?: number;
    avoidCollisions?: boolean;
    collisionPadding?: number;
  }

  let {
    content,
    side = "top",
    sideOffset = 6,
    align = "center",
    alignOffset = 0,
    avoidCollisions = true,
    collisionPadding = 8
  }: Props = $props();
</script>

<Popover
  class="underlay-field-hint__trigger"
  triggerLabel="?"
  triggerAriaLabel={content}
  triggerType="button"
  contentClassName="underlay-field-hint__content"
  {side}
  {sideOffset}
  {align}
  {alignOffset}
  {avoidCollisions}
  {collisionPadding}
>
  <div role="tooltip">{content}</div>
</Popover>

<style>
  :global(.underlay-field-hint__trigger) {
    appearance: none;
    border: none;
    border-radius: var(--underlay-radius-pill, 999px);
    padding: 0;
    margin: 0;

    background: var(
      --underlay-color-field-bg,
      var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18))
    );

    color: var(--underlay-color-text, #fff);

    width: 1.5em;
    height: 1.5em;
    min-width: 0;
    min-height: 0;

    display: inline-flex;
    align-items: center;
    justify-content: center;

    cursor: pointer;
    font-size: 0.6em;
    font-weight: 700;
    line-height: 1;
  }

  /* If the Popover base trigger class is present, ensure our pill styles win. */
  :global(.underlay-popover-trigger.underlay-field-hint__trigger) {
    gap: 0;
    padding: 0;
    border: none;
    border-radius: var(--underlay-radius-pill, 999px);

    background: var(
      --underlay-color-field-bg,
      var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18))
    );

    width: 1.5em;
    height: 1.5em;
  }

  :global(.underlay-field-hint__trigger:hover) {
    background: var(
      --underlay-color-field-bg-hover,
      var(--underlay-color-field-bg-hover, rgba(148, 163, 184, 0.28))
    );
  }

  :global(.underlay-field-hint__content) {
    padding: 0.5rem 0.6rem;
    max-width: min(28rem, calc(100vw - 2rem));
    font-size: var(--underlay-font-size-sm, 0.85rem);
    line-height: 1.25;
  }
</style>
