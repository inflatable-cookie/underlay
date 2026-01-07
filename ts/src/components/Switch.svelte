<script lang="ts">
  export let name: string | undefined = undefined;
  export let initialChecked = false;
  export let leftLabel = "Off";
  export let rightLabel = "On";
  export let disabled = false;

  let checked = initialChecked;
  export { checked };

  function toggle() {
    if (disabled) return;
    checked = !checked;
  }
</script>

<button
  type="button"
  class={`underlay-switch ${checked ? "underlay-switch--on" : "underlay-switch--off"}`}
  on:click={toggle}
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
  name={name}
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
    gap: var(--underlay-space-2, var(--froyo-space-2, 0.5rem));
    padding: 0;
    border: none;
    background: transparent;
    cursor: pointer;
    font: inherit;
    color: inherit;
  }

  .underlay-switch:focus-visible {
    outline: var(--underlay-focus-ring-width, var(--froyo-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--froyo-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--froyo-focus-ring-offset, 2px));
  }

  .underlay-switch[disabled] {
    cursor: default;
    opacity: 0.6;
  }

  .underlay-switch__label {
    font-size: var(--underlay-font-size-xs, var(--froyo-font-size-xs, 0.75rem));
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .underlay-switch__label--left,
  .underlay-switch__label--right {
    color: var(--underlay-color-text-muted, var(--froyo-color-text-muted, #9ca3af));
  }

  .underlay-switch--on .underlay-switch__label--right {
    color: var(--underlay-color-text, var(--froyo-color-text, inherit));
  }

  .underlay-switch--off .underlay-switch__label--left {
    color: var(--underlay-color-text, var(--froyo-color-text, inherit));
  }

  .underlay-switch__track {
    position: relative;
    width: var(--underlay-switch-track-width, var(--froyo-switch-track-width, 2.25rem));
    height: var(--underlay-switch-track-height, var(--froyo-switch-track-height, 1.25rem));
    border-radius: var(--underlay-radius-pill, var(--froyo-radius-pill, 999px));
    background: var(
      --underlay-color-field-bg,
      var(--froyo-color-field-bg, rgba(148, 163, 184, 0.18))
    );
    box-sizing: border-box;
  }

  .underlay-switch__thumb {
    position: absolute;
    top: var(--underlay-switch-thumb-inset, var(--froyo-switch-thumb-inset, 2px));
    left: var(--underlay-switch-thumb-inset, var(--froyo-switch-thumb-inset, 2px));
    width: var(--underlay-switch-thumb-size, var(--froyo-switch-thumb-size, 0.9rem));
    height: var(--underlay-switch-thumb-size, var(--froyo-switch-thumb-size, 0.9rem));
    border-radius: var(--underlay-radius-pill, var(--froyo-radius-pill, 999px));
    background: var(--underlay-color-text, var(--froyo-color-text, #e5e7eb));
    transition: transform 0.15s ease-out;
  }

  .underlay-switch--on .underlay-switch__track {
    background: var(--underlay-color-primary, var(--froyo-color-primary, #2563eb));
  }

  .underlay-switch--on .underlay-switch__thumb {
    transform: translateX(
      var(--underlay-switch-thumb-shift, var(--froyo-switch-thumb-shift, 0.9rem))
    );
  }
</style>
