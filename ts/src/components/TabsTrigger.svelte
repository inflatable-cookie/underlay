<script lang="ts">
  import { Tabs as BitsTabs } from "bits-ui";
  import type { Snippet } from "svelte";
  import { getContext, onMount, onDestroy } from "svelte";
  import type { TabsVariant, TabsSize } from "./TabsRoot.svelte";
  import type { FormTabsSectionRegistryContext } from "./form-tabs/types";

  interface Props {
    value: string;
    disabled?: boolean;
    children?: Snippet;
    class?: string;
    /** Optional count to display as a badge */
    count?: number | null;
    /**
     * Optional label for collapsible dropdown.
     * If not provided, text content of the trigger is used.
     * Falls back to value if neither is available.
     */
    label?: string;
  }

  let { value, disabled = false, children, class: className, count, label }: Props = $props();

  const getVariant = getContext<() => TabsVariant>("underlay-tabs-variant");
  const getSize = getContext<() => TabsSize>("underlay-tabs-size");
  const registry = getContext<FormTabsSectionRegistryContext | undefined>("underlay-form-tabs-registry");
  const registerTab = getContext<((tab: { value: string; label: string; count?: number | null }) => void) | undefined>(
    "underlay-tabs-register"
  );
  const unregisterTab = getContext<((value: string) => void) | undefined>("underlay-tabs-unregister");

  let variant = $derived(getVariant?.() ?? "pills");
  let size = $derived(getSize?.() ?? "default");
  let sectionState = $derived(registry?.getSectionState(value) ?? "idle");

  let triggerRef = $state<HTMLElement | null>(null);

  onMount(() => {
    if (registerTab) {
      // Extract label: explicit prop > text content > value fallback
      const resolvedLabel =
        label ??
        triggerRef?.textContent?.trim() ??
        value;
      registerTab({ value, label: resolvedLabel, count });
    }
  });

  onDestroy(() => {
    unregisterTab?.(value);
  });
</script>

<BitsTabs.Trigger
  bind:ref={triggerRef}
  {value}
  {disabled}
  class={`underlay-tabs-trigger underlay-tabs-trigger--${variant} underlay-tabs-trigger--${size} ${className ?? ""}`}
  data-section-state={sectionState}
>
  {@render children?.()}
  {#if count != null}
    <span class="underlay-tabs-trigger__count">{count}</span>
  {/if}
  {#if sectionState === "invalid"}
    <span class="underlay-tabs-trigger__validation-dot underlay-tabs-trigger__validation-dot--error" aria-label="Has validation errors"></span>
  {:else if sectionState === "incomplete"}
    <span class="underlay-tabs-trigger__validation-dot underlay-tabs-trigger__validation-dot--warning" aria-label="Has required fields"></span>
  {/if}
</BitsTabs.Trigger>

<style>
  /* Base styles */
  :global(.underlay-tabs-trigger) {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.35rem;
    border: none;
    padding: 0.4rem 0.75rem;
    cursor: pointer;
    background: transparent;
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 0.9rem;
    line-height: 1;
  }

  :global(.underlay-tabs-trigger:hover) {
    color: var(--underlay-color-text, #e5e7eb);
  }

  :global(.underlay-tabs-trigger:focus-visible) {
    outline: 2px solid rgba(59, 130, 246, 0.9);
    outline-offset: 2px;
  }

  :global(.underlay-tabs-trigger[data-disabled]) {
    opacity: 0.55;
    cursor: default;
  }

  /* Pills variant */
  :global(.underlay-tabs-trigger--pills) {
    border-radius: 999px;
  }

  :global(.underlay-tabs-trigger--pills[data-state="active"]) {
    background: rgba(148, 163, 184, 0.18);
    color: var(--underlay-color-text, #e5e7eb);
  }

  /* Underline variant - minimal with bottom border */
  :global(.underlay-tabs-trigger--underline) {
    border-radius: 0;
    border-bottom: 2px solid transparent;
    padding: 0.4rem 0.5rem;
  }

  :global(.underlay-tabs-trigger--underline[data-state="active"]) {
    color: var(--underlay-color-text, #e5e7eb);
    border-bottom-color: var(--underlay-color-primary, #3b82f6);
  }

  /* Plain variant - like underline but no border */
  :global(.underlay-tabs-trigger--plain) {
    border-radius: 0;
    padding: 0.4rem 0.5rem;
  }

  :global(.underlay-tabs-trigger--plain[data-state="active"]) {
    color: var(--underlay-color-text, #e5e7eb);
  }

  /* Boxed variant - traditional tabs */
  :global(.underlay-tabs-trigger--boxed) {
    border-radius: 0.4rem 0.4rem 0 0;
    padding: 0.55rem 1rem;
    background: var(--underlay-color-button-neutral-bg, #1f2933);
    border: 1px solid transparent;
    border-bottom: none;
  }

  :global(.underlay-tabs-trigger--boxed:hover) {
    background: var(--underlay-color-button-neutral-hover, #374151);
  }

  :global(.underlay-tabs-trigger--boxed[data-state="active"]) {
    background: var(--underlay-color-button-neutral-hover, #374151);
    color: var(--underlay-color-text, #e5e7eb);
  }

  /* Small size variant */
  :global(.underlay-tabs-trigger--sm) {
    font-size: 0.8rem;
    padding: 0.3rem 0.6rem;
  }

  :global(.underlay-tabs-trigger--sm.underlay-tabs-trigger--boxed) {
    padding: 0.4rem 0.75rem;
  }

  /* Count badge */
  .underlay-tabs-trigger__count {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    margin-left: 0.4rem;
    padding: 0.1rem 0.4rem;
    font-size: 0.75rem;
    font-weight: 500;
    line-height: 1;
    border-radius: 999px;
    background: rgba(148, 163, 184, 0.2);
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  :global(.underlay-tabs-trigger[data-state="active"]) .underlay-tabs-trigger__count {
    background: rgba(148, 163, 184, 0.3);
    color: var(--underlay-color-text, #e5e7eb);
  }

  /* Small size count badge */
  :global(.underlay-tabs-trigger--sm) .underlay-tabs-trigger__count {
    font-size: 0.65rem;
    padding: 0.05rem 0.3rem;
    margin-left: 0.3rem;
  }

  /* Form variant - larger pills for form section navigation */
  :global(.underlay-tabs-trigger--form) {
    flex: 1;
    padding: 0.55rem 1.2rem;
    font-size: 0.95rem;
    font-weight: 500;
    border-radius: 999px;
    gap: 0.5rem;
    transition: background 0.15s ease, color 0.15s ease;
  }

  :global(.underlay-tabs-trigger--form[data-state="active"]) {
    background: rgba(148, 163, 184, 0.22);
    color: var(--underlay-color-text, #e5e7eb);
  }

  :global(.underlay-tabs-trigger--form:hover:not([data-state="active"])) {
    background: rgba(148, 163, 184, 0.1);
    color: var(--underlay-color-text, #e5e7eb);
  }

  /* Validation indicator dots */
  .underlay-tabs-trigger__validation-dot {
    display: inline-block;
    width: 0.5rem;
    height: 0.5rem;
    border-radius: 50%;
    flex-shrink: 0;
    margin-left: 0.25rem;
  }

  .underlay-tabs-trigger__validation-dot--error {
    background: var(--underlay-color-error, #ef4444);
  }

  .underlay-tabs-trigger__validation-dot--warning {
    background: var(--underlay-color-warning, #f59e0b);
  }
</style>
