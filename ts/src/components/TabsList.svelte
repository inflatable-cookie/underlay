<script lang="ts">
  import { Tabs as BitsTabs } from "bits-ui";
  import type { Snippet } from "svelte";
  import { getContext, onMount } from "svelte";
  import type { TabsVariant, TabsSize, RegisteredTab } from "./TabsRoot.svelte";
  import type { FormTabsSectionRegistryContext, SectionValidationState } from "./form-tabs/types";
  import ChevronDown from "lucide-svelte/icons/chevron-down";

  interface Props {
    children?: Snippet;
    class?: string;
    /**
     * Enable responsive collapse to dropdown when tabs overflow.
     * Tabs are auto-registered from TabsTrigger children.
     */
    collapsible?: boolean;
  }

  let { children, class: className, collapsible = false }: Props = $props();

  const getVariant = getContext<() => TabsVariant>("underlay-tabs-variant");
  const getSize = getContext<() => TabsSize>("underlay-tabs-size");
  const getValue = getContext<() => string>("underlay-tabs-value");
  const setValue = getContext<((v: string) => void) | undefined>("underlay-tabs-set-value");
  const setParentCollapsed = getContext<((v: boolean) => void) | undefined>("underlay-tabs-set-collapsed");
  const registry = getContext<FormTabsSectionRegistryContext | undefined>("underlay-form-tabs-registry");
  const getRegisteredTabs = getContext<(() => RegisteredTab[]) | undefined>("underlay-tabs-get-registered");

  let variant = $derived(getVariant?.() ?? "pills");
  let size = $derived(getSize?.() ?? "default");
  let currentValue = $derived(getValue?.() ?? "");

  let containerRef = $state<HTMLElement | null>(null);
  let tabsRef = $state<HTMLElement | null>(null);
  let isCollapsed = $state(false);
  let dropdownOpen = $state(false);
  let lastTabsWidth = $state(0);

  // Use auto-registered tabs from TabsTrigger children
  let registeredTabs = $state<RegisteredTab[]>([]);
  
  // Poll for registered tabs (they're registered on mount)
  $effect(() => {
    if (!collapsible || !getRegisteredTabs) return;
    registeredTabs = getRegisteredTabs();
  });

  // Find the current tab for dropdown display
  const currentTab = $derived(registeredTabs.find((t) => t.value === currentValue));

  // Get section validation state for the current tab (form variant only)
  let currentTabState = $derived<SectionValidationState>(registry?.getSectionState(currentValue) ?? "idle");

  // Helper to get validation state for any tab
  function getTabState(value: string): SectionValidationState {
    return registry?.getSectionState(value) ?? "idle";
  }

  // Sync collapsed state to parent TabsRoot
  $effect(() => {
    setParentCollapsed?.(isCollapsed);
  });

  function handleDropdownSelect(value: string) {
    setValue?.(value);
    dropdownOpen = false;
  }

  onMount(() => {
    if (!collapsible || !containerRef) return;

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
    
    // Also check after registered tabs are populated
    const checkInterval = setInterval(() => {
      const tabs = getRegisteredTabs?.() ?? [];
      if (tabs.length > 0) {
        registeredTabs = tabs;
        requestAnimationFrame(checkOverflow);
        clearInterval(checkInterval);
      }
    }, 50);

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
      clearInterval(checkInterval);
    };
  });
</script>

{#if collapsible && registeredTabs.length > 0}
  <div
    bind:this={containerRef}
    class="underlay-tabs-list-container underlay-tabs-list-container--{variant} underlay-tabs-list-container--{size}"
  >
    <!-- Collapsed dropdown mode -->
    {#if isCollapsed}
      <div class="underlay-tabs-dropdown underlay-tabs-dropdown--{variant} underlay-tabs-dropdown--{size}">
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
            {#if currentTabState === "invalid"}
              <span class="underlay-tabs-dropdown__dot underlay-tabs-dropdown__dot--error" aria-label="Has validation errors"></span>
            {:else if currentTabState === "incomplete"}
              <span class="underlay-tabs-dropdown__dot underlay-tabs-dropdown__dot--warning" aria-label="Has required fields"></span>
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
            {#each registeredTabs as tab (tab.value)}
              <li>
                <button
                  type="button"
                  class="underlay-tabs-dropdown__item"
                  class:underlay-tabs-dropdown__item--active={tab.value === currentValue}
                  role="option"
                  aria-selected={tab.value === currentValue}
                  onclick={() => handleDropdownSelect(tab.value)}
                >
                  <span class="underlay-tabs-dropdown__item-label">
                    {tab.label}
                    {#if getTabState(tab.value) === "invalid"}
                      <span class="underlay-tabs-dropdown__dot underlay-tabs-dropdown__dot--error" aria-label="Has validation errors"></span>
                    {:else if getTabState(tab.value) === "incomplete"}
                      <span class="underlay-tabs-dropdown__dot underlay-tabs-dropdown__dot--warning" aria-label="Has required fields"></span>
                    {/if}
                  </span>
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
          class={`underlay-tabs-list underlay-tabs-list--${variant} underlay-tabs-list--${size} underlay-tabs-list__tabs ${className ?? ""}`}
        >
          {@render children?.()}
        </BitsTabs.List>
      </div>
    {/if}
  </div>
{:else}
  <!-- Non-collapsible mode - original behavior -->
  <BitsTabs.List
    class={`underlay-tabs-list underlay-tabs-list--${variant} underlay-tabs-list--${size} ${className ?? ""}`}
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

  /* Underline variant - minimal with bottom border */
  :global(.underlay-tabs-list--underline) {
    gap: 0;
    padding: 0;
    border: none;
    background: transparent;
    border-bottom: 1px solid rgba(148, 163, 184, 0.25);
  }

  /* Boxed variant - traditional tabs */
  :global(.underlay-tabs-list--boxed) {
    gap: 0.35rem;
    padding: 0.35rem 0.5rem 0;
    border: none;
    background: rgba(0, 0, 0, 0.25);
    border-radius: 0.5rem 0.5rem 0 0;
  }

  /* Form variant - larger pills for form section navigation */
  :global(.underlay-tabs-list--form) {
    display: flex;
    padding: 0.35rem;
    border-radius: 999px;
    border: 1px solid rgba(148, 163, 184, 0.35);
    background: rgba(255, 255, 255, 0.03);
    gap: 0.35rem;
  }

  /* Small size variant */
  :global(.underlay-tabs-list--sm.underlay-tabs-list--boxed) {
    gap: 0.25rem;
    padding: 0.25rem 0.4rem 0;
  }

  :global(.underlay-tabs-list--sm.underlay-tabs-list--pills) {
    padding: 0.15rem;
    gap: 0.15rem;
  }

  /* Container for collapsible tabs */
  .underlay-tabs-list-container {
    display: block;
    width: 100%;
    max-width: 100%;
  }

  /* .underlay-tabs-list-container--boxed: No special styling - let dropdown handle it */

  /* Wrapper for measuring tabs width */
  .underlay-tabs-list__measure {
    display: inline-block;
    max-width: 100%;
  }

  /* Form variant measure wrapper stretches full width */
  .underlay-tabs-list-container--form .underlay-tabs-list__measure {
    display: block;
    width: 100%;
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

  /* Small size dropdown */
  .underlay-tabs-dropdown--sm .underlay-tabs-dropdown__trigger {
    padding: 0.4rem 0.6rem;
    font-size: 0.8rem;
  }

  .underlay-tabs-dropdown--sm .underlay-tabs-dropdown__item {
    padding: 0.4rem 0.6rem;
    font-size: 0.8rem;
  }

  .underlay-tabs-dropdown--sm .underlay-tabs-dropdown__count {
    font-size: 0.65rem;
    padding: 0.05rem 0.3rem;
  }

  /* Form variant dropdown - larger pills style */
  .underlay-tabs-dropdown--form {
    padding: 0.35rem;
    border-radius: 999px;
    border: 1px solid rgba(148, 163, 184, 0.35);
    background: rgba(255, 255, 255, 0.03);
  }

  .underlay-tabs-dropdown--form .underlay-tabs-dropdown__trigger {
    padding: 0.55rem 1.2rem;
    font-size: 0.95rem;
    font-weight: 500;
    border-radius: 999px;
  }

  .underlay-tabs-dropdown--form .underlay-tabs-dropdown__item {
    font-size: 0.95rem;
    font-weight: 500;
    padding: 0.55rem 1rem;
  }

  /* Dropdown item label wrapper */
  .underlay-tabs-dropdown__item-label {
    display: flex;
    align-items: center;
    gap: 0.4rem;
  }

  /* Validation indicator dots in dropdown */
  .underlay-tabs-dropdown__dot {
    display: inline-block;
    width: 0.5rem;
    height: 0.5rem;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .underlay-tabs-dropdown__dot--error {
    background: var(--underlay-color-error, #ef4444);
  }

  .underlay-tabs-dropdown__dot--warning {
    background: var(--underlay-color-warning, #f59e0b);
  }
</style>
