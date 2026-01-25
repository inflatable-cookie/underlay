<script lang="ts">
  import { Tabs as BitsTabs } from "bits-ui";
  import type { Snippet } from "svelte";
  import { getContext, onMount } from "svelte";
  import type { TabsVariant } from "./TabsRoot.svelte";
  import ChevronDown from "lucide-svelte/icons/chevron-down";

  interface TabItem {
    value: string;
    label: string;
    count?: number | null;
  }

  interface Props {
    children?: Snippet;
    class?: string;
    /**
     * When provided with collapsible=true, enables responsive collapse to dropdown.
     * Array of tab items with value, label, and optional count.
     */
    tabs?: TabItem[];
    /**
     * Enable responsive collapse to dropdown when tabs overflow.
     * Requires `tabs` prop to be set.
     */
    collapsible?: boolean;
  }

  let { children, class: className, tabs, collapsible = false }: Props = $props();

  const getVariant = getContext<() => TabsVariant>("underlay-tabs-variant");
  const getValue = getContext<() => string>("underlay-tabs-value");
  const setValue = getContext<((v: string) => void) | undefined>("underlay-tabs-set-value");
  const setParentCollapsed = getContext<((v: boolean) => void) | undefined>("underlay-tabs-set-collapsed");

  let variant = $derived(getVariant?.() ?? "pills");
  let currentValue = $derived(getValue?.() ?? "");

  let containerRef = $state<HTMLElement | null>(null);
  let tabsRef = $state<HTMLElement | null>(null);
  let isCollapsed = $state(false);
  let dropdownOpen = $state(false);
  let lastTabsWidth = $state(0);

  // Find the current tab for dropdown display
  const currentTab = $derived(tabs?.find((t) => t.value === currentValue));

  // Sync collapsed state to parent TabsRoot
  $effect(() => {
    setParentCollapsed?.(isCollapsed);
  });

  function handleDropdownSelect(value: string) {
    setValue?.(value);
    dropdownOpen = false;
  }

  onMount(() => {
    if (!collapsible || !tabs || !containerRef) return;

    const checkOverflow = () => {
      if (!containerRef) return;

      if (tabsRef) {
        // Tabs are visible - measure and remember their width
        lastTabsWidth = tabsRef.scrollWidth;
        isCollapsed = tabsRef.scrollWidth > containerRef.clientWidth;
      } else if (lastTabsWidth > 0) {
        // Tabs are hidden (collapsed mode) - use remembered width
        // Add some hysteresis: only expand if container is 20px wider than needed
        isCollapsed = lastTabsWidth > containerRef.clientWidth - 20;
      }
    };

    const observer = new ResizeObserver(checkOverflow);
    observer.observe(containerRef);

    // Initial check after a brief delay to let tabs render
    requestAnimationFrame(checkOverflow);

    // Close dropdown when clicking outside
    const handleClickOutside = (e: MouseEvent) => {
      if (dropdownOpen && containerRef && !containerRef.contains(e.target as Node)) {
        dropdownOpen = false;
      }
    };

    document.addEventListener("click", handleClickOutside);

    return () => {
      observer.disconnect();
      document.removeEventListener("click", handleClickOutside);
    };
  });
</script>

{#if collapsible && tabs}
  <div
    bind:this={containerRef}
    class="underlay-tabs-list-container underlay-tabs-list-container--{variant}"
  >
    <!-- Collapsed dropdown mode -->
    {#if isCollapsed}
      <div class="underlay-tabs-dropdown underlay-tabs-dropdown--{variant}">
        <button
          type="button"
          class="underlay-tabs-dropdown__trigger"
          onclick={() => (dropdownOpen = !dropdownOpen)}
          aria-expanded={dropdownOpen}
          aria-haspopup="listbox"
        >
          <span class="underlay-tabs-dropdown__label">
            {currentTab?.label ?? "Select tab"}
            {#if currentTab?.count != null}
              <span class="underlay-tabs-dropdown__count">{currentTab.count}</span>
            {/if}
          </span>
          <ChevronDown size={16} class="underlay-tabs-dropdown__chevron" />
        </button>

        {#if dropdownOpen}
          <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <ul
            class="underlay-tabs-dropdown__menu"
            role="listbox"
            onclick={(e) => e.stopPropagation()}
          >
            {#each tabs as tab}
              <li>
                <button
                  type="button"
                  class="underlay-tabs-dropdown__item"
                  class:underlay-tabs-dropdown__item--active={tab.value === currentValue}
                  role="option"
                  aria-selected={tab.value === currentValue}
                  onclick={() => handleDropdownSelect(tab.value)}
                >
                  {tab.label}
                  {#if tab.count != null}
                    <span class="underlay-tabs-dropdown__count">{tab.count}</span>
                  {/if}
                </button>
              </li>
            {/each}
          </ul>
        {/if}
      </div>
    {:else}
      <!-- Normal tabs mode -->
      <div bind:this={tabsRef} class="underlay-tabs-list__measure">
        <BitsTabs.List
          class={`underlay-tabs-list underlay-tabs-list--${variant} underlay-tabs-list__tabs ${className ?? ""}`}
        >
          {@render children?.()}
        </BitsTabs.List>
      </div>
    {/if}
  </div>
{:else}
  <!-- Non-collapsible mode - original behavior -->
  <BitsTabs.List
    class={`underlay-tabs-list underlay-tabs-list--${variant} ${className ?? ""}`}
  >
    {@render children?.()}
  </BitsTabs.List>
{/if}

<style>
  /* Base styles */
  :global(.underlay-tabs-list) {
    display: inline-flex;
    gap: 0.25rem;
  }

  /* Pills variant (default) */
  :global(.underlay-tabs-list--pills) {
    padding: 0.25rem;
    border-radius: 999px;
    border: 1px solid rgba(148, 163, 184, 0.35);
    background: rgba(255, 255, 255, 0.03);
  }

  /* Boxed variant - traditional tabs */
  :global(.underlay-tabs-list--boxed) {
    gap: 0.35rem;
    padding: 0.35rem 0.5rem 0;
    border: none;
    background: rgba(0, 0, 0, 0.25);
    border-radius: 0.5rem 0.5rem 0 0;
  }

  /* Container for collapsible tabs */
  .underlay-tabs-list-container {
    display: block;
    width: 100%;
    max-width: 100%;
  }

  .underlay-tabs-list-container--boxed {
    /* No special styling - let dropdown handle it */
  }

  /* Wrapper for measuring tabs width */
  .underlay-tabs-list__measure {
    display: inline-block;
    max-width: 100%;
  }

  /* Dropdown styles */
  .underlay-tabs-dropdown {
    position: relative;
    display: block;
    width: 100%;
  }

  .underlay-tabs-dropdown--boxed {
    background: transparent;
    border-radius: 0;
    padding: 0;
  }

  .underlay-tabs-dropdown--pills {
    padding: 0.25rem;
    border-radius: 999px;
    border: 1px solid rgba(148, 163, 184, 0.35);
    background: rgba(255, 255, 255, 0.03);
  }

  .underlay-tabs-dropdown__trigger {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: 100%;
    padding: 0.6rem 0.75rem;
    border: none;
    border-radius: 0.4rem;
    background: var(--underlay-color-button-neutral-bg, #1f2933);
    color: var(--underlay-color-text, #e5e7eb);
    font-size: 0.9rem;
    cursor: pointer;
    justify-content: space-between;
  }

  .underlay-tabs-dropdown__trigger:hover {
    background: var(--underlay-color-button-neutral-hover, #374151);
  }

  .underlay-tabs-dropdown__label {
    display: flex;
    align-items: center;
    gap: 0.4rem;
  }

  .underlay-tabs-dropdown__count {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0.1rem 0.4rem;
    font-size: 0.75rem;
    font-weight: 500;
    line-height: 1;
    border-radius: 999px;
    background: rgba(148, 163, 184, 0.3);
    color: var(--underlay-color-text, #e5e7eb);
  }

  :global(.underlay-tabs-dropdown__chevron) {
    transition: transform 0.15s ease;
  }

  .underlay-tabs-dropdown__trigger[aria-expanded="true"] :global(.underlay-tabs-dropdown__chevron) {
    transform: rotate(180deg);
  }

  .underlay-tabs-dropdown__menu {
    position: absolute;
    top: 100%;
    left: 0;
    z-index: 50;
    margin-top: 0.25rem;
    padding: 0.25rem;
    min-width: 100%;
    list-style: none;
    background: var(--underlay-color-surface, #1f2933);
    border: 1px solid rgba(148, 163, 184, 0.25);
    border-radius: 0.5rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  }

  .underlay-tabs-dropdown__item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 0.5rem 0.75rem;
    border: none;
    border-radius: 0.35rem;
    background: transparent;
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 0.9rem;
    cursor: pointer;
    text-align: left;
  }

  .underlay-tabs-dropdown__item:hover {
    background: rgba(148, 163, 184, 0.1);
    color: var(--underlay-color-text, #e5e7eb);
  }

  .underlay-tabs-dropdown__item--active {
    background: rgba(148, 163, 184, 0.18);
    color: var(--underlay-color-text, #e5e7eb);
  }
</style>
