<script lang="ts">
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher<{ click: MouseEvent }>();

  function handleClick(event: MouseEvent) {
    dispatch("click", event);
  }

  export let label: string;
  export let type: "button" | "submit" | "reset" = "button";
  export let disabled = false;
  export let sizeRem = 2;
  export let variant: "neutral" | "danger" = "neutral";
</script>

<button
  {...$$restProps}
  {type}
  {disabled}
  aria-label={label}
  onclick={handleClick}
  class={`underlay-icon-button underlay-icon-button--${variant} ${$$restProps.class ?? ""}`}
  style={`--underlay-icon-button-size: ${sizeRem}rem;`}
>
  <slot />
</button>

<style>
  :global(.underlay-icon-button) {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--underlay-icon-button-size, 2rem);
    height: var(--underlay-icon-button-size, 2rem);
    border-radius: 0.5rem;
    border: 1px solid rgba(148, 163, 184, 0.35);
    background: rgba(255, 255, 255, 0.03);
    color: var(--underlay-color-text, var(--underlay-color-text, #e5e7eb));
    cursor: pointer;
    padding: 0;
    line-height: 1;
  }

  :global(.underlay-icon-button:hover) {
    background: rgba(148, 163, 184, 0.08);
  }

  :global(.underlay-icon-button:focus-visible) {
    outline: 2px solid rgba(59, 130, 246, 0.9);
    outline-offset: 2px;
  }

  :global(.underlay-icon-button:disabled) {
    opacity: 0.55;
    cursor: default;
  }

  :global(.underlay-icon-button--danger) {
    border-color: rgba(239, 68, 68, 0.5);
    background: rgba(239, 68, 68, 0.12);
  }

  :global(.underlay-icon-button--danger:hover) {
    background: rgba(239, 68, 68, 0.18);
  }
</style>
