<script lang="ts">
  import { tick, getContext } from "svelte";
  import { Select as BitsSelect } from "bits-ui";
  import type { Snippet } from "svelte";
  import X from "lucide-svelte/icons/x";
  import ChevronDown from "lucide-svelte/icons/chevron-down";
  import {
    FIELD_A11Y_CONTEXT_KEY,
    mergeAriaDescribedBy,
    type FieldA11yContext
  } from "./field/a11y-context";

  type SelectItem = {
    value: string;
    label: string;
    disabled?: boolean;
  };

  type SelectGroup = {
    label: string;
    items?: SelectItem[];
    groups?: SelectGroup[];
  };

  type GroupRow =
    | { kind: "heading"; key: string; label: string; level: number }
    | { kind: "item"; key: string; item: SelectItem; level: number };

  interface Props {
    value?: string;
    open?: boolean;
    items?: SelectItem[] | null;
    groups?: SelectGroup[] | null;
    placeholder?: string;
    id?: string;
    name?: string;
    required?: boolean;
    disabled?: boolean;
    triggerType?: "button" | "submit" | "reset";
    triggerAriaLabel?: string | null;
    contentClassName?: string;
    returnFocusOnClose?: boolean;
    side?: "top" | "right" | "bottom" | "left";
    sideOffset?: number;
    align?: "start" | "center" | "end";
    alignOffset?: number;
    avoidCollisions?: boolean;
    collisionPadding?: number;
    onchange?: (value: string) => void;
    oninput?: (value: string) => void;
    children?: Snippet;
    class?: string;
    /** Show a clear button to reset to default value */
    clearable?: boolean;
    /** Value to reset to when cleared (default: "") */
    defaultValue?: string;
    /** Async function to load items on first open. When provided, items are fetched lazily. */
    loadItems?: () => Promise<SelectItem[]>;
    /** Async function to load grouped items on first open. */
    loadGroups?: () => Promise<SelectGroup[]>;
    /** Label to display for the current value before items are loaded */
    valueLabel?: string;
    /** Key to invalidate cached loaded items (e.g. parent filter value for cascading dropdowns) */
    loadKey?: string;
  }

  let {
    value = $bindable(""),
    open = $bindable(false),
    items = undefined,
    groups = undefined,
    placeholder = "Select…",
    id = undefined,
    name = undefined,
    required = false,
    disabled = false,
    triggerType = "button",
    triggerAriaLabel = null,
    contentClassName = "",
    returnFocusOnClose = true,
    side = "bottom",
    sideOffset = 6,
    align = "end",
    alignOffset = 0,
    avoidCollisions = true,
    collisionPadding = 8,
    onchange,
    oninput,
    children,
    class: className,
    clearable = false,
    defaultValue = "",
    loadItems = undefined,
    loadGroups = undefined,
    valueLabel = undefined,
    loadKey = undefined,
  }: Props = $props();

  const fieldA11y = getContext<FieldA11yContext | undefined>(FIELD_A11Y_CONTEXT_KEY);

  const controlId = $derived(id ?? fieldA11y?.controlId());

  const matchesField = $derived(fieldA11y?.matchesControl(controlId) ?? false);
  const hasFieldError = $derived(matchesField && (fieldA11y?.hasError() ?? false));
  const fieldErrorId = $derived(matchesField ? fieldA11y?.errorId() : undefined);

  const ariaInvalid = $derived(hasFieldError ? true : undefined);
  const ariaDescribedBy = $derived(mergeAriaDescribedBy(fieldErrorId));
  const ariaErrorMessage = $derived(hasFieldError ? fieldErrorId : undefined);

  let triggerRef: HTMLElement | null = $state(null);
  let lastOpen = $state(open);
  let lastDispatchedValue = $state(value);

  function flattenGroupItems(input: SelectGroup[] | null | undefined): SelectItem[] {
    if (!input?.length) return [];
    const acc: SelectItem[] = [];
    for (const group of input) {
      if (group.items?.length) {
        acc.push(...group.items);
      }
      if (group.groups?.length) {
        acc.push(...flattenGroupItems(group.groups));
      }
    }
    return acc;
  }

  function buildGroupRows(
    input: SelectGroup[] | null | undefined,
    level = 1,
    parentKey = "group"
  ): GroupRow[] {
    if (!input?.length) return [];
    const rows: GroupRow[] = [];

    input.forEach((group, groupIndex) => {
      const groupKey = `${parentKey}-${groupIndex}`;
      rows.push({
        kind: "heading",
        key: `${groupKey}-heading`,
        label: group.label,
        level
      });

      for (const item of group.items ?? []) {
        rows.push({
          kind: "item",
          key: `${groupKey}-item-${item.value}`,
          item,
          level
        });
      }

      if (group.groups?.length) {
        rows.push(...buildGroupRows(group.groups, level + 1, `${groupKey}-sub`));
      }
    });

    return rows;
  }

  // --- Lazy loading ---
  const isLazy = $derived(Boolean(loadItems || loadGroups));
  let loadedItems = $state<SelectItem[] | null>(null);
  let loadedGroups = $state<SelectGroup[] | null>(null);
  let loadState = $state<"idle" | "loading" | "loaded" | "error">("idle");
  let loadError = $state<string | null>(null);
  let lastLoadKey = $state<string | undefined>(undefined);

  // Effective items/groups: loaded data takes precedence, then fall back to props
  const effectiveItems = $derived(loadedItems ?? items ?? null);
  const effectiveGroups = $derived(loadedGroups ?? groups ?? null);

  // Invalidate cache when loadKey changes
  $effect(() => {
    if (loadKey !== lastLoadKey) {
      lastLoadKey = loadKey;
      if (loadedItems || loadedGroups) {
        loadedItems = null;
        loadedGroups = null;
        loadState = "idle";
        loadError = null;
      }
    }
  });

  // Trigger load on first open
  $effect(() => {
    if (open && isLazy && loadState === "idle") {
      loadState = "loading";
      const loader = loadGroups
        ? loadGroups().then((g) => { loadedGroups = g; })
        : loadItems!().then((i) => { loadedItems = i; });
      loader
        .then(() => { loadState = "loaded"; })
        .catch((e) => {
          loadState = "error";
          loadError = e?.message ?? "Failed to load options";
        });
    }
  });

  function retryLoad() {
    loadState = "idle";
    loadError = null;
    // Re-trigger by opening again (already open)
    if (open && isLazy) {
      loadState = "loading";
      const loader = loadGroups
        ? loadGroups().then((g) => { loadedGroups = g; })
        : loadItems!().then((i) => { loadedItems = i; });
      loader
        .then(() => { loadState = "loaded"; })
        .catch((e) => {
          loadState = "error";
          loadError = e?.message ?? "Failed to load options";
        });
    }
  }

  $effect(() => {
    if (lastOpen && !open && returnFocusOnClose && typeof window !== "undefined") {
      void tick().then(() => triggerRef?.focus());
    }
    lastOpen = open;
  });

  // BitsSelect doesn't emit native "change" events in a way we can forward
  // without opting into internal types, so we dispatch when bound value changes.
  let allItems = $derived(
    effectiveGroups?.length
      ? flattenGroupItems(effectiveGroups)
      : (effectiveItems ?? [])
  );

  let hasGroups = $derived(Boolean(effectiveGroups?.length));
  let groupRows = $derived(buildGroupRows(effectiveGroups));

  $effect(() => {
    if (allItems.length && value !== lastDispatchedValue) {
      lastDispatchedValue = value;
      onchange?.(value);
      oninput?.(value);
    }
  });

  let selectedLabel = $derived(
    allItems.find((item) => item.value === value)?.label
    ?? (isLazy && loadState !== "loaded" ? valueLabel : undefined)
  );
  let hasSelection = $derived(typeof selectedLabel === "string" && selectedLabel.length > 0);
  let isDefaultValue = $derived(value === defaultValue || value === "");
  let showClearButton = $derived(clearable && hasSelection && value !== defaultValue && !disabled);

  function handleClear(event: MouseEvent) {
    event.stopPropagation();
    event.preventDefault();
    value = defaultValue;
    lastDispatchedValue = defaultValue;
    onchange?.(defaultValue);
    oninput?.(defaultValue);
  }

  let alignShift = $derived(
    align === "end"
      ? "calc((var(--bits-floating-anchor-width) - 100%) / 2)"
      : align === "start"
        ? "calc((100% - var(--bits-floating-anchor-width)) / 2)"
        : "0px"
  );

  let alignShiftRtl = $derived(
    align === "end"
      ? "calc((100% - var(--bits-floating-anchor-width)) / 2)"
      : align === "start"
        ? "calc((var(--bits-floating-anchor-width) - 100%) / 2)"
        : "0px"
  );

  function handleNativeChange(event: Event) {
    const target = event.currentTarget as HTMLSelectElement | null;
    const next = target ? target.value : value;
    value = next;
    onchange?.(next);
    oninput?.(next);
  }
</script>

{#if allItems.length || isLazy}
  <BitsSelect.Root
    type="single"
    items={allItems}
    bind:value={value as never}
    bind:open
    {name}
    {required}
    {disabled}
  >
    <BitsSelect.Trigger
      id={controlId}
      bind:ref={triggerRef}
      type={triggerType}
      class={`underlay-select-trigger ${className ?? ""}`}
      aria-label={triggerAriaLabel ?? placeholder}
      aria-invalid={ariaInvalid}
      aria-describedby={ariaDescribedBy}
      aria-errormessage={ariaErrorMessage}
    >
      <span class="underlay-select-trigger__text" class:underlay-placeholder={!hasSelection} class:underlay-default-value={hasSelection && isDefaultValue}>
        {hasSelection ? selectedLabel : placeholder}
      </span>
      <span class="underlay-select-trigger__controls">
        {#if showClearButton}
          <button
            type="button"
            class="underlay-select-trigger__clear"
            aria-label="Clear selection"
            onclick={handleClear}
            onpointerdown={(e) => e.stopPropagation()}
            onmousedown={(e) => e.stopPropagation()}
          >
            <X size="1em" strokeWidth={2.5} />
          </button>
        {/if}
        <ChevronDown size="1em" strokeWidth={2.5} class="underlay-select-trigger__chevron" />
      </span>
    </BitsSelect.Trigger>

    <BitsSelect.Portal>
      <BitsSelect.Content
        class={`underlay-select-content ${contentClassName}`}
        style={`--underlay-select-align-shift: ${alignShift}; --underlay-select-align-shift-rtl: ${alignShiftRtl};`}
        {side}
        {sideOffset}
        {align}
        {alignOffset}
        {avoidCollisions}
        {collisionPadding}
      >
        <BitsSelect.Viewport class="underlay-select-viewport">
          {#if isLazy && loadState === "loading"}
            <div class="underlay-select-loading">Loading…</div>
          {:else if isLazy && loadState === "error"}
            <div class="underlay-select-error">
              <span>{loadError}</span>
              <button class="underlay-select-error__retry" onclick={retryLoad}>Retry</button>
            </div>
          {:else if hasGroups}
            {#each groupRows as row (row.key)}
              {#if row.kind === "heading"}
                <div
                  class="underlay-select-group-heading"
                  data-level={row.level}
                  style={`--underlay-select-level: ${row.level};`}
                >
                  {row.label}
                </div>
              {:else}
                {@const isSelected = row.item.value === value}
                <BitsSelect.Item
                  value={row.item.value}
                  label={row.item.label}
                  disabled={row.item.disabled}
                  class="underlay-select-item"
                  data-level={row.level}
                  style={`--underlay-select-level: ${row.level};`}
                >
                  <span class="underlay-select-item__label">{row.item.label}</span>
                  {#if isSelected}
                    <span class="underlay-select-item__check" aria-hidden="true">✓</span>
                  {/if}
                </BitsSelect.Item>
              {/if}
            {/each}
          {:else}
            {#each effectiveItems ?? [] as item (item.value)}
              {@const isSelected = item.value === value}
              <BitsSelect.Item
                value={item.value}
                label={item.label}
                disabled={item.disabled}
                class="underlay-select-item"
              >
                <span class="underlay-select-item__label">{item.label}</span>
                {#if isSelected}
                  <span class="underlay-select-item__check" aria-hidden="true">✓</span>
                {/if}
              </BitsSelect.Item>
            {/each}
          {/if}
        </BitsSelect.Viewport>
      </BitsSelect.Content>
    </BitsSelect.Portal>
  </BitsSelect.Root>
{:else}
  <select
    id={controlId}
    class={`underlay-select ${className ?? ""}`}
    bind:value
    {name}
    {required}
    {disabled}
    aria-invalid={ariaInvalid}
    aria-describedby={ariaDescribedBy}
    aria-errormessage={ariaErrorMessage}
    onchange={handleNativeChange}
  >
    {#if children}
      {@render children()}
    {/if}
  </select>
{/if}

<style>
  .underlay-select {
    width: 100%;
    min-width: min(var(--underlay-select-min-width, 12rem), 100%);
    box-sizing: border-box;
    padding: 0.55em 0.7em;
    border-radius: 0.35rem;
    border: none;
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
    color: var(--underlay-color-text, #e5e7eb);
    font-size: 0.85rem;
  }

  .underlay-select-group-heading {
    position: sticky;
    top: 0;
    z-index: 1;
    display: block;
    padding: 0.35rem 0.7rem;
    padding-left: calc(0.7rem + (var(--underlay-select-level, 1) - 1) * 0.35rem);
    margin: 0.2rem 0 0.15rem;
    font-size: 0.72rem;
    font-weight: 700;
    letter-spacing: 0.02em;
    text-transform: uppercase;
    color: var(--underlay-color-text-subtle, var(--underlay-color-text-muted, #64748b));
    background: var(--underlay-color-bg-surface, #fff);
  }

  .underlay-select-group-heading:not([data-level="1"]) {
    font-size: 0.65rem;
    text-transform: none;
    letter-spacing: 0;
    opacity: 0.85;
  }

  .underlay-select:focus,
  .underlay-select:focus-visible {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--underlay-focus-ring-offset, 2px));
  }

  /* Disabled state for native select */
  .underlay-select:disabled {
    color: var(--underlay-color-text-disabled, rgba(156, 163, 175, 0.5));
    cursor: not-allowed;
    opacity: 0.6;
  }

  :global(.underlay-select-trigger) {
    width: 100%;
    min-width: min(var(--underlay-select-min-width, 12rem), 100%);
    display: inline-flex;
    align-items: center;
    justify-content: space-between;
    box-sizing: border-box;
    padding: 0.55em 0.7em;
    border-radius: 0.35rem;
    border: none;
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
    color: var(--underlay-color-text, #e5e7eb);
    font-size: 0.85rem;
    cursor: pointer;
    container-type: inline-size;
  }

  :global(.underlay-select-trigger:focus-visible) {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--underlay-focus-ring-offset, 2px));
  }

  /* Disabled state for trigger */
  :global(.underlay-select-trigger:disabled),
  :global(.underlay-select-trigger[data-disabled]) {
    color: var(--underlay-color-text-disabled, rgba(156, 163, 175, 0.5));
    cursor: not-allowed;
    opacity: 0.6;
  }

  :global(.underlay-select-trigger__text) {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    min-width: 0;
  }

  .underlay-placeholder {
    color: var(--underlay-color-text-muted, var(--underlay-color-text-muted, #9ca3af));
  }

  .underlay-default-value {
    opacity: 0.6;
  }

  :global(.underlay-select-trigger__controls) {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    margin-left: 0.5rem;
    flex-shrink: 0;
  }

  @container (max-width: 6rem) {
    :global(.underlay-select-trigger__controls) {
      gap: 0.15rem;
      margin-left: 0.25rem;
    }
  }

  :global(.underlay-select-trigger__clear) {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    margin: 0;
    border: none;
    background: transparent;
    color: var(--underlay-color-text-muted, #9ca3af);
    cursor: pointer;
    border-radius: 0.2rem;
    opacity: 0.7;
    transition: opacity 0.15s ease, color 0.15s ease;
  }

  :global(.underlay-select-trigger__clear:hover) {
    opacity: 1;
    color: var(--underlay-color-danger, #ef4444);
  }

  :global(.underlay-select-trigger__clear:focus-visible) {
    outline: 2px solid var(--underlay-color-primary, #2563eb);
    outline-offset: 1px;
  }

  :global(.underlay-select-trigger__clear svg) {
    margin-top: 1px;
  }

  :global(.underlay-select-trigger__chevron) {
    color: var(--underlay-color-text, #e5e7eb);
    opacity: 0.5;
    margin-top: 1px;
  }

  :global(.underlay-select-content) {
    z-index: 200;
    border-radius: 0.35rem;
    border: 1px solid
      var(
        --underlay-color-border-subtle,
        var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.5))
      );

    background: var(
      --underlay-color-menu-bg,
      var(--underlay-color-bg-surface, #020617)
    );

    box-shadow: var(
      --underlay-shadow-menu,
      0 8px 16px rgba(0, 0, 0, 0.5)
    );

    padding: 0.25rem;

    min-width: max(var(--underlay-select-min-content-width, 8rem), min(var(--bits-floating-anchor-width), calc(100vw - 1.5rem)));
    width: max(var(--underlay-select-min-content-width, 8rem), var(--bits-floating-anchor-width));
    max-width: min(26rem, calc(100vw - 1.5rem));
  }

  :global(.underlay-select-content[data-align="center"]) {
    transform: translateX(var(--underlay-select-align-shift, 0px));
  }

  :global([dir="rtl"] .underlay-select-content[data-align="center"]) {
    transform: translateX(var(--underlay-select-align-shift-rtl, 0px));
  }

  :global(.underlay-select-viewport) {
    display: flex;
    flex-direction: column;
    gap: 0.1rem;
    max-height: min(18rem, var(--bits-select-content-available-height));
    overflow: auto;
  }

  :global(.underlay-select-item) {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    padding: 0.35rem 0.55rem;
    padding-left: calc(0.55rem + (var(--underlay-select-level, 1) - 1) * 0.35rem);
    border-radius: 0.25rem;
    cursor: pointer;
    user-select: none;
    font-size: 0.85rem;
    color: var(--underlay-color-text, var(--underlay-color-text, #e5e7eb));
  }

  :global(.underlay-select-item[data-highlighted]) {
    background: var(
      --underlay-color-hover-bg,
      var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.2))
    );
  }

  :global(.underlay-select-item[data-disabled]) {
    opacity: 0.6;
    cursor: default;
  }

  :global(.underlay-select-item__check) {
    opacity: 0.9;
  }

  /* Override bits-ui floating wrapper z-index to ensure Select appears above popovers */
  :global([data-bits-floating-content-wrapper]:has(.underlay-select-content)) {
    z-index: 200 !important;
  }

  /* Lazy-loading states */
  .underlay-select-loading {
    padding: 0.55rem 0.7rem;
    font-size: 0.8rem;
    color: var(--underlay-color-text-muted, #9ca3af);
    text-align: center;
  }

  .underlay-select-error {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.35rem;
    padding: 0.55rem 0.7rem;
    font-size: 0.8rem;
    color: var(--underlay-color-danger, #ef4444);
    text-align: center;
  }

  .underlay-select-error__retry {
    padding: 0.2rem 0.5rem;
    border: none;
    border-radius: 0.25rem;
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
    color: var(--underlay-color-text, #e5e7eb);
    font-size: 0.75rem;
    cursor: pointer;
  }

  .underlay-select-error__retry:hover {
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.3));
  }
</style>
