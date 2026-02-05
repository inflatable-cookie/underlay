<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

  interface Props extends Omit<HTMLButtonAttributes, "class"> {
    variant?: "primary" | "secondary" | "subtle" | "danger" | "danger-subtle";
    /** Size of the button. Use "icon" or "icon-sm" for icon-only buttons. */
    size?: "sm" | "md" | "lg" | "icon" | "icon-sm";
    type?: "button" | "submit" | "reset";
    pill?: boolean;
    class?: string;
    onclick?: (event: MouseEvent) => void;
    disabled?: boolean;
    children?: Snippet;
  }

  let {
    variant = "primary",
    size = "md",
    type = "button",
    pill = true,
    class: className = "",
    onclick,
    disabled = false,
    children,
    ...restProps
  }: Props = $props();
</script>

<button
  {...restProps}
  {onclick}
  {disabled}
  class={`underlay-button ${pill ? "underlay-button--pill" : "underlay-button--square"} underlay-button--${variant} underlay-button--${size} ${className}`}
  {type}
>
  {@render children?.()}
</button>

<style>
  :global(.underlay-button) {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--underlay-button-gap, var(--underlay-button-gap, 0.35em));
    box-sizing: border-box;
    border: none;
    padding: var(--underlay-button-padding-block, var(--underlay-button-padding-block, 0.6em))
      var(--underlay-button-padding-inline, var(--underlay-button-padding-inline, 1.2em));
    font-size: var(--underlay-button-font-size, calc(1em * 0.95));
    font-weight: 500;
    cursor: pointer;
    background: var(
      --underlay-color-button-neutral-bg,
      var(--underlay-color-button-neutral-bg, rgba(255, 255, 255, 0.03))
    );
    color: var(--underlay-color-text, var(--underlay-color-text, inherit));
    transition:
      background-color 0.12s ease-out,
      border-color 0.12s ease-out,
      color 0.12s ease-out,
      box-shadow 0.12s ease-out;
  }

  :global(.underlay-button--pill) {
    border-radius: var(--underlay-radius-pill, var(--underlay-radius-pill, 999px));
  }

  :global(.underlay-button--square) {
    border-radius: var(--underlay-radius-sm, var(--underlay-radius-sm, 0.35rem));
  }

  /* Sizes */
  :global(.underlay-button--sm) {
    padding: 0.4em 0.8em;
    font-size: 0.8em;
  }

  /* md size uses base styles, no overrides needed */

  :global(.underlay-button--lg) {
    padding: 0.75em 1.5em;
    font-size: 1.1em;
  }

  :global(.underlay-button--icon) {
    width: 2.25rem;
    height: 2.25rem;
    padding: 0;
    border-radius: var(--underlay-radius-control, 0.5rem);
  }

  :global(.underlay-button--icon-sm) {
    width: 1.75rem;
    height: 1.75rem;
    padding: 0;
    font-size: 0.875rem;
    border-radius: var(--underlay-radius-control, 0.5rem);
  }

  :global(.underlay-button--primary) {
    background-color: var(
      --underlay-color-primary,
      var(--underlay-color-primary, #2563eb)
    );
    color: var(--underlay-color-on-primary, var(--underlay-color-on-primary, white));
    box-shadow: var(--underlay-shadow-md, var(--underlay-shadow-md, none));
  }

  :global(.underlay-button--primary:hover) {
    background-color: var(
      --underlay-color-primary-strong,
      var(--underlay-color-primary-strong, #1d4ed8)
    );
  }

  :global(.underlay-button--secondary) {
    background-color: var(
      --underlay-color-button-secondary,
      var(--underlay-color-button-secondary, #ea580c)
    );
    color: var(--underlay-color-on-primary, var(--underlay-color-on-primary, white));
  }

  :global(.underlay-button--secondary:hover) {
    background-color: var(
      --underlay-color-button-secondary-strong,
      var(--underlay-color-button-secondary-strong, #f97316)
    );
  }

  :global(.underlay-button--subtle) {
    background-color: var(
      --underlay-color-field-bg,
      var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18))
    );
    color: var(--underlay-color-text-muted, var(--underlay-color-text-muted, inherit));
    box-shadow: none;
    border: 1px solid transparent;
  }

  :global(.underlay-button--subtle:hover) {
    background-color: var(
      --underlay-color-field-bg-hover,
      var(--underlay-color-field-bg-hover, rgba(148, 163, 184, 0.25))
    );
    color: var(--underlay-color-text, var(--underlay-color-text, inherit));
  }

  :global(.underlay-button--danger) {
    background-color: var(
      --underlay-color-danger,
      var(--underlay-color-danger, #dc2626)
    );
    color: var(--underlay-color-on-danger, var(--underlay-color-on-danger, white));
    box-shadow: var(--underlay-shadow-md, var(--underlay-shadow-md, none));
  }

  :global(.underlay-button--danger:hover) {
    background-color: var(
      --underlay-color-danger-strong,
      var(--underlay-color-danger-strong, #b91c1c)
    );
  }

  :global(.underlay-button--danger-subtle) {
    background-color: var(
      --underlay-color-field-bg,
      var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18))
    );
    color: color-mix(in srgb, var(--underlay-color-danger, #ef4444) 70%, var(--underlay-color-text-muted, #9ca3af));
    box-shadow: none;
    border: 1px solid transparent;
  }

  :global(.underlay-button--danger-subtle:hover) {
    background-color: var(
      --underlay-color-danger,
      var(--underlay-color-danger, #dc2626)
    );
    color: var(--underlay-color-on-danger, var(--underlay-color-on-danger, white));
  }

  :global(.underlay-button:focus-visible) {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--underlay-focus-ring-offset, 2px));
  }

  :global(.underlay-button:disabled),
  :global(.underlay-button[aria-disabled="true"]) {
    opacity: 0.6;
    cursor: default;
    box-shadow: none;
  }
</style>
