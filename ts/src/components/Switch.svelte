<script lang="ts">
  type SwitchVariant = "default" | "danger-off";
  type SwitchStateVariant = "default" | "primary" | "success" | "warning" | "danger";

  interface Props {
    name?: string;
    initialChecked?: boolean;
    leftLabel?: string;
    rightLabel?: string;
    disabled?: boolean;
    checked?: boolean;
    /** @deprecated Legacy variant style. Use leftVariant and rightVariant instead. */
    variant?: SwitchVariant;
    /** Color variant for left/off state */
    leftVariant?: SwitchStateVariant;
    /** Color variant for right/on state */
    rightVariant?: SwitchStateVariant;
  }

  let {
    name,
    initialChecked = false,
    leftLabel = "Off",
    rightLabel = "On",
    disabled = false,
    checked = $bindable(initialChecked),
    variant = "default",
    leftVariant = "default",
    rightVariant = "primary"
  }: Props = $props();

  function toggle() {
    if (disabled) return;
    checked = !checked;
  }

  // Build variant classes
  const variantClass = $derived(() => {
    const classes: string[] = [];

    // Legacy variant support
    if (variant === "danger-off") {
      classes.push("underlay-switch--danger-off");
    }

    // New flexible variants
    if (leftVariant !== "default") {
      classes.push(`underlay-switch--left-${leftVariant}`);
    }
    if (rightVariant !== "default" && rightVariant !== "primary") {
      classes.push(`underlay-switch--right-${rightVariant}`);
    }

    return classes.join(" ");
  });
</script>

<button
  type="button"
  class={`underlay-switch ${checked ? "underlay-switch--on" : "underlay-switch--off"} ${variantClass()}`}
  onclick={toggle}
  role="switch"
  aria-checked={checked}
  aria-label={`${checked ? rightLabel : leftLabel}`}
  {disabled}
>
  <span class="underlay-switch__label underlay-switch__label--left">{leftLabel}</span>
  <span class="underlay-switch__track">
    <span class="underlay-switch__thumb"></span>
  </span>
  <span class="underlay-switch__label underlay-switch__label--right">{rightLabel}</span>
</button>

<input
  class="underlay-switch__input"
  type="checkbox"
  {name}
  bind:checked
  value="on"
  hidden
  aria-hidden="true"
  tabindex="-1"
/>

<style>
  .underlay-switch {
    display: inline-flex;
    align-items: center;
    gap: var(--underlay-space-2, var(--underlay-space-2, 0.5rem));
    width: max-content;
    min-height: var(--underlay-input-height, 2em);
    padding: 0;
    border: none;
    background: transparent;
    cursor: pointer;
    font: inherit;
    color: inherit;
  }

  .underlay-switch:focus-visible {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--underlay-focus-ring-offset, 2px));
  }

  .underlay-switch[disabled] {
    cursor: default;
    opacity: 0.6;
  }

  .underlay-switch__label {
    font-size: var(--underlay-font-size-sm, 0.8rem);
  }

  .underlay-switch__label--left,
  .underlay-switch__label--right {
    color: var(--underlay-color-text-muted, var(--underlay-color-text-muted, #9ca3af));
  }

  .underlay-switch--on .underlay-switch__label--right {
    color: var(--underlay-color-text, var(--underlay-color-text, inherit));
  }

  .underlay-switch--off .underlay-switch__label--left {
    color: var(--underlay-color-text, var(--underlay-color-text, inherit));
  }

  .underlay-switch__track {
    position: relative;
    width: var(--underlay-switch-track-width, var(--underlay-switch-track-width, 2.25rem));
    height: var(--underlay-switch-track-height, var(--underlay-switch-track-height, 1.25rem));
    border-radius: var(--underlay-radius-pill, var(--underlay-radius-pill, 999px));
    background: var(
      --underlay-color-field-bg,
      var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18))
    );
    box-sizing: border-box;
  }

  .underlay-switch__thumb {
    position: absolute;
    top: 50%;
    left: var(--underlay-switch-thumb-inset, 2px);
    width: var(--underlay-switch-thumb-size, 0.9rem);
    height: var(--underlay-switch-thumb-size, 0.9rem);
    border-radius: var(--underlay-radius-pill, 999px);
    background: var(--underlay-color-text, #e5e7eb);
    transform: translateY(-50%);
    transition: transform 0.15s ease-out, left 0.15s ease-out;
  }

  .underlay-switch--on .underlay-switch__track {
    background: var(--underlay-color-primary, #2563eb);
  }

  .underlay-switch--on .underlay-switch__thumb {
    left: calc(100% - var(--underlay-switch-thumb-size, 0.9rem) - var(--underlay-switch-thumb-inset, 2px));
  }

  /* Legacy danger-off variant: red track and label when off */
  .underlay-switch--danger-off.underlay-switch--off .underlay-switch__track {
    background: var(--underlay-color-error, #ef4444);
  }

  .underlay-switch--danger-off.underlay-switch--off .underlay-switch__label--left {
    color: var(--underlay-color-error, #ef4444);
  }

  /* Flexible left state variants (when switch is OFF) */
  .underlay-switch--left-primary.underlay-switch--off .underlay-switch__track {
    background: var(--underlay-color-primary, #2563eb);
  }
  .underlay-switch--left-primary.underlay-switch--off .underlay-switch__label--left {
    color: var(--underlay-color-primary, #2563eb);
  }

  .underlay-switch--left-success.underlay-switch--off .underlay-switch__track {
    background: var(--underlay-color-success, #22c55e);
  }
  .underlay-switch--left-success.underlay-switch--off .underlay-switch__label--left {
    color: var(--underlay-color-success, #22c55e);
  }

  .underlay-switch--left-warning.underlay-switch--off .underlay-switch__track {
    background: var(--underlay-color-warning, #de8d04);
  }
  .underlay-switch--left-warning.underlay-switch--off .underlay-switch__label--left {
    color: var(--underlay-color-warning, #de8d04);
  }

  .underlay-switch--left-danger.underlay-switch--off .underlay-switch__track {
    background: var(--underlay-color-danger, #ef4444);
  }
  .underlay-switch--left-danger.underlay-switch--off .underlay-switch__label--left {
    color: var(--underlay-color-danger, #ef4444);
  }

  /* Flexible right state variants (when switch is ON) */
  .underlay-switch--right-success.underlay-switch--on .underlay-switch__track {
    background: var(--underlay-color-success, #22c55e);
  }
  .underlay-switch--right-success.underlay-switch--on .underlay-switch__label--right {
    color: var(--underlay-color-success, #22c55e);
  }

  .underlay-switch--right-warning.underlay-switch--on .underlay-switch__track {
    background: var(--underlay-color-warning, #de8d04);
  }
  .underlay-switch--right-warning.underlay-switch--on .underlay-switch__label--right {
    color: var(--underlay-color-warning, #de8d04);
  }

  .underlay-switch--right-danger.underlay-switch--on .underlay-switch__track {
    background: var(--underlay-color-danger, #ef4444);
  }
  .underlay-switch--right-danger.underlay-switch--on .underlay-switch__label--right {
    color: var(--underlay-color-danger, #ef4444);
  }
</style>
