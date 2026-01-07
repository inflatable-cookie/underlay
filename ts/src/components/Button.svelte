<script lang="ts">
  import { Button as BitsButton } from "bits-ui";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher<{ click: MouseEvent }>();

  function handleClick(event: MouseEvent) {
    dispatch("click", event);
  }

  export let variant: "primary" | "secondary" | "subtle" = "primary";
  export let type: "button" | "submit" | "reset" = "button";
  export let pill: boolean = true;

  export let className: string = "";
</script>

<BitsButton.Root
  {...$$restProps}
  onclick={handleClick}
  class={`underlay-button ${pill ? "underlay-button--pill" : "underlay-button--square"} underlay-button--${variant} ${className} ${$$restProps.class ?? ""}`}
  {type}
>
  <slot />
</BitsButton.Root>

<style>
  :global(.underlay-button) {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--underlay-button-gap, var(--froyo-button-gap, 0.35em));
    box-sizing: border-box;
    border: none;
    padding: var(--underlay-button-padding-block, var(--froyo-button-padding-block, 0.6em))
      var(--underlay-button-padding-inline, var(--froyo-button-padding-inline, 1.2em));
    font-size: var(--underlay-button-font-size, calc(1em * 0.95));
    font-weight: 500;
    cursor: pointer;
    background: var(
      --underlay-color-button-neutral-bg,
      var(--froyo-color-button-neutral-bg, rgba(255, 255, 255, 0.03))
    );
    color: var(--underlay-color-text, var(--froyo-color-text, inherit));
    transition:
      background-color 0.12s ease-out,
      border-color 0.12s ease-out,
      color 0.12s ease-out,
      box-shadow 0.12s ease-out;
  }

  :global(.underlay-button--pill) {
    border-radius: var(--underlay-radius-pill, var(--froyo-radius-pill, 999px));
  }

  :global(.underlay-button--square) {
    border-radius: var(--underlay-radius-sm, var(--froyo-radius-sm, 0.35rem));
  }

  :global(.underlay-button--primary) {
    background-color: var(
      --underlay-color-primary,
      var(--froyo-color-primary, #2563eb)
    );
    color: var(--underlay-color-on-primary, var(--froyo-color-on-primary, white));
    box-shadow: var(--underlay-shadow-md, var(--froyo-shadow-md, none));
  }

  :global(.underlay-button--primary:hover) {
    background-color: var(
      --underlay-color-primary-strong,
      var(--froyo-color-primary-strong, #1d4ed8)
    );
  }

  :global(.underlay-button--secondary) {
    background-color: var(
      --underlay-color-button-secondary,
      var(--froyo-color-button-secondary, #ea580c)
    );
    color: var(--underlay-color-on-primary, var(--froyo-color-on-primary, white));
  }

  :global(.underlay-button--secondary:hover) {
    background-color: var(
      --underlay-color-button-secondary-strong,
      var(--froyo-color-button-secondary-strong, #f97316)
    );
  }

  :global(.underlay-button--subtle) {
    background-color: var(
      --underlay-color-field-bg,
      var(--froyo-color-field-bg, rgba(148, 163, 184, 0.18))
    );
    color: var(--underlay-color-text-muted, var(--froyo-color-text-muted, inherit));
    box-shadow: none;
    border: 1px solid transparent;
  }

  :global(.underlay-button--subtle:hover) {
    background-color: var(
      --underlay-color-field-bg-hover,
      var(--froyo-color-field-bg-hover, rgba(148, 163, 184, 0.25))
    );
    color: var(--underlay-color-text, var(--froyo-color-text, inherit));
  }

  :global(.underlay-button:focus-visible) {
    outline: var(--underlay-focus-ring-width, var(--froyo-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--froyo-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--froyo-focus-ring-offset, 2px));
  }

  :global(.underlay-button:disabled),
  :global(.underlay-button[aria-disabled="true"]) {
    opacity: 0.6;
    cursor: default;
    box-shadow: none;
  }
</style>
