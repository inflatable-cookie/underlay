<script lang="ts">
  interface Option {
    label: string;
    value: string;
    disabled?: boolean;
    title?: string;
  }

  interface Props {
    /** Form input name for hidden field submission */
    name?: string;
    /** Available segments */
    options: Option[];
    /** Currently selected value (bindable) */
    value?: string;
    /** Disable all segments */
    disabled?: boolean;
    /** Size of each segment */
    size?: "md" | "sm" | "xs";
    /** When false, each segment sizes to content instead of equal width. */
    equalWidth?: boolean;
    /** Accessible label for the radiogroup */
    ariaLabel?: string;
    /** Callback when selected value changes */
    onchange?: (value: string) => void;
  }

  let {
    name,
    options,
    value = $bindable(options[0]?.value ?? ""),
    disabled = false,
    size = "md",
    equalWidth = true,
    ariaLabel,
    onchange
  }: Props = $props();

  function select(v: string) {
    if (disabled) return;
    if (value === v) return;
    value = v;
    onchange?.(v);
  }
</script>

{#if name}
  <input type="hidden" {name} {value} />
{/if}

<div
  class="underlay-segmented"
  class:underlay-segmented--disabled={disabled}
  role="radiogroup"
  aria-label={ariaLabel}
>
  {#each options as opt, i}
    {@const isActive = value === opt.value}
    {@const nextActive = i < options.length - 1 && value === options[i + 1]?.value}
    <button
      type="button"
      class="underlay-segmented__option"
      class:underlay-segmented__option--active={isActive}
      class:underlay-segmented__option--before-active={nextActive}
      class:underlay-segmented__option--fit={!equalWidth}
      class:underlay-segmented__option--sm={size === "sm"}
      class:underlay-segmented__option--xs={size === "xs"}
      role="radio"
      aria-checked={isActive}
      disabled={disabled || !!opt.disabled}
      title={opt.title}
      onclick={() => select(opt.value)}
    >
      {opt.label}
    </button>
  {/each}
</div>

<style>
  .underlay-segmented {
    display: flex;
    border: 1px solid
      var(
        --underlay-color-border-subtle,
        var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.25))
      );
    border-radius: var(--underlay-radius-sm, var(--underlay-radius-sm, 0.35rem));
    overflow: hidden;
    background: var(
      --underlay-color-field-bg,
      var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18))
    );
  }

  .underlay-segmented--disabled {
    opacity: 0.6;
    cursor: default;
    pointer-events: none;
  }

  .underlay-segmented__option {
    flex: 1 1 0%;
    padding: var(--underlay-field-padding-block, var(--underlay-field-padding-block, 0.55em))
      var(--underlay-field-padding-inline, var(--underlay-field-padding-inline, 0.7em));
    font: inherit;
    font-size: var(--underlay-font-size-sm, 0.8rem);
    font-weight: 500;
    line-height: 1.4;
    text-align: center;
    cursor: pointer;
    border: none;
    border-right: 1px solid
      var(
        --underlay-color-border-subtle,
        var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.25))
      );
    background: transparent;
    color: var(--underlay-color-text-muted, var(--underlay-color-text-muted, #9ca3af));
    transition: background-color 0.15s ease, color 0.15s ease;
  }

  .underlay-segmented__option--fit {
    flex: 0 0 auto;
  }

  .underlay-segmented__option--sm {
    padding: 0.4em 0.58em;
    font-size: 0.74rem;
    line-height: 1.3;
  }

  .underlay-segmented__option--xs {
    padding: 0.3em 0.48em;
    font-size: 0.68rem;
    line-height: 1.2;
  }

  .underlay-segmented__option:last-child {
    border-right: none;
  }

  /* Suppress internal border adjacent to active segment */
  .underlay-segmented__option--active {
    border-right-color: transparent;
  }

  .underlay-segmented__option--before-active {
    border-right-color: transparent;
  }

  .underlay-segmented__option:hover:not(.underlay-segmented__option--active) {
    background: var(
      --underlay-color-field-bg-hover,
      var(--underlay-color-field-bg-hover, rgba(148, 163, 184, 0.28))
    );
    color: var(--underlay-color-text, var(--underlay-color-text, inherit));
  }

  .underlay-segmented__option--active {
    background: var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    color: #fff;
  }

  .underlay-segmented__option:focus-visible {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    outline-offset: var(
      --underlay-focus-ring-offset,
      var(--underlay-focus-ring-offset, 2px)
    );
    z-index: 1;
  }

  .underlay-segmented__option:disabled {
    opacity: 0.55;
    cursor: default;
  }
</style>
