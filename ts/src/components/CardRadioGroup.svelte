<script lang="ts">
  import type { Component, SvelteComponent } from "svelte";

  type IconComponent =
    | Component<{ size?: number; strokeWidth?: number }>
    | (new (...args: any[]) => SvelteComponent);

  interface Option {
    value: string;
    title: string;
    description?: string;
    icon?: IconComponent | IconComponent[];
    tone?: "neutral" | "success" | "warning" | "danger";
    disabled?: boolean;
  }

  interface Props {
    name?: string;
    options: Option[];
    value?: string;
    disabled?: boolean;
    required?: boolean;
    ariaLabel?: string;
    minColumnWidth?: string;
    onchange?: (value: string) => void;
  }

  let {
    name,
    options,
    value = $bindable(options[0]?.value ?? ""),
    disabled = false,
    required = false,
    ariaLabel,
    minColumnWidth = "12rem",
    onchange,
  }: Props = $props();

  const enabledOptions = $derived(options.filter((option) => !option.disabled));
  const selectedIndex = $derived(options.findIndex((option) => option.value === value));

  function select(next: string) {
    if (disabled || value === next) return;
    value = next;
    onchange?.(next);
  }

  function focusOption(index: number) {
    const node = document.querySelector<HTMLElement>(
      `.underlay-card-radio-group__option[data-option-index="${index}"]`,
    );
    node?.focus();
  }

  function handleKeyDown(event: KeyboardEvent, index: number) {
    if (disabled || enabledOptions.length <= 1) {
      return;
    }

    const currentOption = options[index];
    if (!currentOption || currentOption.disabled) {
      return;
    }

    const navigableValues = enabledOptions.map((option) => option.value);
    const activeIndex = navigableValues.indexOf(currentOption.value);
    if (activeIndex === -1) {
      return;
    }

    const moveNext = ["ArrowRight", "ArrowDown"];
    const movePrev = ["ArrowLeft", "ArrowUp"];

    if (!moveNext.includes(event.key) && !movePrev.includes(event.key)) {
      return;
    }

    event.preventDefault();
    const delta = moveNext.includes(event.key) ? 1 : -1;
    const nextIndex =
      (activeIndex + delta + navigableValues.length) % navigableValues.length;
    const nextValue = navigableValues[nextIndex];
    const optionIndex = options.findIndex((option) => option.value === nextValue);
    if (nextValue && optionIndex >= 0) {
      select(nextValue);
      focusOption(optionIndex);
    }
  }
</script>

{#if name}
  <input type="hidden" {name} {value} />
{/if}

<div
  class="underlay-card-radio-group"
  class:underlay-card-radio-group--disabled={disabled}
  role="radiogroup"
  aria-label={ariaLabel}
  style={`--underlay-card-radio-group-min:${minColumnWidth};`}
>
  {#each options as option, index}
    {@const selected = option.value === value}
    {@const icons = option.icon ? (Array.isArray(option.icon) ? option.icon : [option.icon]) : []}
    <button
      type="button"
      class="underlay-card-radio-group__option"
      class:underlay-card-radio-group__option--selected={selected}
      class:underlay-card-radio-group__option--tone-success={option.tone === "success"}
      class:underlay-card-radio-group__option--tone-warning={option.tone === "warning"}
      class:underlay-card-radio-group__option--tone-danger={option.tone === "danger"}
      role="radio"
      aria-checked={selected}
      aria-disabled={disabled || option.disabled}
      disabled={disabled || option.disabled}
      data-option-index={index}
      onclick={() => select(option.value)}
      onkeydown={(event) => handleKeyDown(event, index)}
    >
      {#if icons.length > 0}
        <div
          class="underlay-card-radio-group__icon-group"
          class:underlay-card-radio-group__icon-group--stacked={icons.length > 1}
        >
          {#each icons as Icon}
            <span class="underlay-card-radio-group__icon-tile">
              <Icon size={24} strokeWidth={2.1} />
            </span>
          {/each}
        </div>
      {/if}
      <div class="underlay-card-radio-group__body">
        <span class="underlay-card-radio-group__title">{option.title}</span>
        {#if option.description}
          <span class="underlay-card-radio-group__description">
            {option.description}
          </span>
        {/if}
      </div>
    </button>
  {/each}
</div>

<style>
  .underlay-card-radio-group {
    display: grid;
    grid-template-columns: repeat(
      auto-fit,
      minmax(min(var(--underlay-card-radio-group-min, 15rem), 100%), 1fr)
    );
    gap: 0.85rem;
    width: 100%;
  }

  .underlay-card-radio-group--disabled {
    opacity: 0.7;
  }

  .underlay-card-radio-group__option {
    --underlay-card-radio-group-accent: var(--underlay-color-primary, #14b8a6);
    appearance: none;
    -webkit-appearance: none;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
    gap: 0.85rem;
    min-height: 8rem;
    width: 100%;
    padding: 1rem 1.05rem;
    border: 1px solid
      var(
        --underlay-color-border-subtle,
        rgba(148, 163, 184, 0.28)
      );
    border-radius: 0.75rem;
    background:
      linear-gradient(
        180deg,
        color-mix(in srgb, var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18)) 82%, transparent),
        color-mix(in srgb, var(--underlay-color-bg-surface, #020617) 92%, transparent)
      );
    background-clip: padding-box;
    color: var(--underlay-color-text, inherit);
    text-align: center;
    cursor: pointer;
    transition:
      border-color 0.16s ease,
      background-color 0.16s ease,
      transform 0.16s ease,
      box-shadow 0.16s ease;
  }

  .underlay-card-radio-group__option:hover:not(:disabled) {
    border-color: color-mix(
      in srgb,
      var(--underlay-card-radio-group-accent) 34%,
      var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.28))
    );
    transform: translateY(-1px);
  }

  .underlay-card-radio-group__option--selected {
    border-color: color-mix(
      in srgb,
      var(--underlay-card-radio-group-accent) 55%,
      white 0%
    );
    background:
      linear-gradient(
        180deg,
        color-mix(in srgb, var(--underlay-card-radio-group-accent) 14%, transparent),
        color-mix(in srgb, var(--underlay-color-bg-surface, #020617) 94%, transparent)
      );
    box-shadow: 0 0 0 1px
      color-mix(in srgb, var(--underlay-card-radio-group-accent) 28%, transparent);
  }

  .underlay-card-radio-group__option:focus-visible {
    outline: 2px solid var(--underlay-card-radio-group-accent);
    outline-offset: 2px;
  }

  .underlay-card-radio-group__option--tone-success {
    --underlay-card-radio-group-accent: var(--underlay-color-success, #16a34a);
  }

  .underlay-card-radio-group__option--tone-warning {
    --underlay-card-radio-group-accent: var(--underlay-color-warning, #f59e0b);
  }

  .underlay-card-radio-group__option--tone-danger {
    --underlay-card-radio-group-accent: var(--underlay-color-danger, #ef4444);
  }

  .underlay-card-radio-group__option:disabled {
    cursor: default;
    opacity: 0.6;
    transform: none;
  }

  .underlay-card-radio-group__icon-group {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.25rem;
  }

  .underlay-card-radio-group__icon-group--stacked {
    justify-content: center;
  }

  .underlay-card-radio-group__icon-tile {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 3.2rem;
    height: 3.2rem;
    flex: 0 0 3.2rem;
    border-radius: 0.7rem;
    border: 1px solid
      color-mix(
        in srgb,
        var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.28)) 92%,
        transparent
      );
    background: color-mix(
      in srgb,
      var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18)) 88%,
      transparent
    );
    color: var(--underlay-color-text-muted, #94a3b8);
  }

  .underlay-card-radio-group__icon-group--stacked .underlay-card-radio-group__icon-tile:first-child {
    background: color-mix(
      in srgb,
      var(--underlay-card-radio-group-accent) 8%,
      var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18))
    );
    border-color: color-mix(
      in srgb,
      var(--underlay-card-radio-group-accent) 22%,
      var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.28))
    );
    color: color-mix(
      in srgb,
      var(--underlay-card-radio-group-accent) 68%,
      var(--underlay-color-text, #e2e8f0)
    );
  }

  .underlay-card-radio-group__icon-group--stacked .underlay-card-radio-group__icon-tile:last-child {
    opacity: 0.82;
  }

  .underlay-card-radio-group__option--selected .underlay-card-radio-group__icon-tile {
    background: color-mix(
      in srgb,
      var(--underlay-card-radio-group-accent) 18%,
      transparent
    );
    border-color: color-mix(
      in srgb,
      var(--underlay-card-radio-group-accent) 38%,
      transparent
    );
    color: var(--underlay-card-radio-group-accent);
  }

  .underlay-card-radio-group__option--selected
    .underlay-card-radio-group__icon-group--stacked
    .underlay-card-radio-group__icon-tile:first-child {
    background: color-mix(
      in srgb,
      var(--underlay-card-radio-group-accent) 26%,
      transparent
    );
    border-color: color-mix(
      in srgb,
      var(--underlay-card-radio-group-accent) 52%,
      transparent
    );
  }

  .underlay-card-radio-group__option--selected
    .underlay-card-radio-group__icon-group--stacked
    .underlay-card-radio-group__icon-tile:last-child {
    opacity: 0.92;
  }

  .underlay-card-radio-group__body {
    display: flex;
    flex-direction: column;
    gap: 0.28rem;
    min-width: 0;
    width: 100%;
    align-items: center;
  }

  .underlay-card-radio-group__title {
    font-size: 0.95rem;
    font-weight: 700;
    line-height: 1.25;
    color: var(--underlay-color-text, inherit);
  }

  .underlay-card-radio-group__description {
    font-size: 0.82rem;
    line-height: 1.35;
    color: var(--underlay-color-text-muted, #94a3b8);
  }
</style>
