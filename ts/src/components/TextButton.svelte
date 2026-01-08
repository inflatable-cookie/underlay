<script lang="ts">
  import { Button as BitsButton } from "bits-ui";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher<{ click: MouseEvent }>();

  function handleClick(event: MouseEvent) {
    dispatch("click", event);
  }

  export let type: "button" | "submit" | "reset" = "button";
  export let variant: "default" | "danger" = "default";
  export let className: string = "";
</script>

<BitsButton.Root
  {...$$restProps}
  class={`underlay-text-button underlay-text-button--${variant} ${className} ${$$restProps.class ?? ""}`}
  {type}
  onclick={handleClick}
>
  <slot />
</BitsButton.Root>

<style>
  :global(.underlay-text-button) {
    border: none;
    background: transparent;
    color: var(--underlay-color-text-muted, var(--underlay-color-text-muted, inherit));
    font-size: calc(1em * var(--underlay-font-scale-sm, 0.85));
    cursor: pointer;
    padding: 0;
  }

  :global(.underlay-text-button:focus-visible) {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--underlay-focus-ring-offset, 2px));
  }

  :global(.underlay-text-button:hover) {
    color: var(--underlay-color-text, var(--underlay-color-text, inherit));
    text-decoration: underline;
  }

  :global(.underlay-text-button--danger) {
    color: var(--underlay-color-error, #ef4444);
  }

  :global(.underlay-text-button--danger:hover) {
    color: var(--underlay-color-error, #ef4444);
    text-decoration: underline;
    text-decoration-thickness: 2px;
  }

  :global(.underlay-text-button:disabled),
  :global(.underlay-text-button[aria-disabled="true"]) {
    opacity: 0.6;
    cursor: default;
    text-decoration: none;
  }
</style>
