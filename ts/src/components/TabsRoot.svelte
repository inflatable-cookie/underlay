<script lang="ts">
  import { Tabs as BitsTabs } from "bits-ui";
  import type { Snippet } from "svelte";
  import { setContext, onMount } from "svelte";
  import { browser } from "$app/environment";
  import { page } from "$app/stores";

  export type TabsVariant = "pills" | "boxed" | "underline" | "plain" | "form";
  export type TabsSize = "default" | "sm";

  export interface RegisteredTab {
    value: string;
    label: string;
    count?: number | null;
  }

  interface Props {
    /** Current active tab value (bindable) */
    value: string;
    /** Visual style variant */
    variant?: TabsVariant;
    /** Size variant - 'sm' gives smaller text and padding */
    size?: TabsSize;
    /**
     * When provided, syncs the active tab with URL query param.
     * The tab state will be preserved in browser history, so hitting
     * back after navigating away will restore the previous tab.
     * Example: historyKey="tab" stores as ?tab=details
     */
    historyKey?: string;
    /** Additional CSS classes */
    class?: string;
    children?: Snippet;
  }

  let { value = $bindable(), variant = "pills", size = "default", historyKey, class: className, children }: Props =
    $props();

  // Collapsed state (set by TabsList when in collapsible mode)
  let isCollapsed = $state(false);

  // Auto-registered tabs (populated by TabsTrigger components)
  let registeredTabs = $state<Map<string, RegisteredTab>>(new Map());

  function registerTab(tab: RegisteredTab) {
    registeredTabs = new Map(registeredTabs).set(tab.value, tab);
  }

  function unregisterTab(value: string) {
    const newMap = new Map(registeredTabs);
    newMap.delete(value);
    registeredTabs = newMap;
  }

  function getRegisteredTabs(): RegisteredTab[] {
    return Array.from(registeredTabs.values());
  }

  setContext("underlay-tabs-variant", () => variant);
  setContext("underlay-tabs-size", () => size);
  setContext("underlay-tabs-value", () => value);
  setContext("underlay-tabs-set-value", (v: string) => {
    value = v;
  });
  setContext("underlay-tabs-collapsed", () => isCollapsed);
  setContext("underlay-tabs-set-collapsed", (v: boolean) => {
    isCollapsed = v;
  });
  setContext("underlay-tabs-register", registerTab);
  setContext("underlay-tabs-unregister", unregisterTab);
  setContext("underlay-tabs-get-registered", getRegisteredTabs);

  // Track the last value we synced to URL to prevent loops
  let lastSyncedValue = $state<string | null>(null);

  function replaceUrlTabParam(nextValue: string) {
    if (!browser || !historyKey) return;
    const url = new URL(window.location.href);
    url.searchParams.set(historyKey, nextValue);
    window.history.replaceState(window.history.state, "", url);
  }

  // Read initial tab from URL on mount
  onMount(() => {
    if (!browser || !historyKey) return;

    const urlValue = $page.url.searchParams.get(historyKey);
    if (urlValue) {
      value = urlValue;
      lastSyncedValue = urlValue;
    } else {
      // Set initial value in URL without navigation or history entry churn
      replaceUrlTabParam(value);
      lastSyncedValue = value;
    }

    // Listen for back/forward navigation
    const handlePopState = () => {
      const newUrl = new URL(window.location.href);
      const newValue = newUrl.searchParams.get(historyKey);
      if (newValue && newValue !== value) {
        value = newValue;
        lastSyncedValue = newValue;
      }
    };

    window.addEventListener("popstate", handlePopState);
    return () => window.removeEventListener("popstate", handlePopState);
  });

  // Update URL when tab changes (without creating history entry)
  $effect(() => {
    if (!browser || !historyKey) return;
    // Skip if this is the same value we just synced (prevents loops)
    if (value === lastSyncedValue) return;

    replaceUrlTabParam(value);
    lastSyncedValue = value;
  });
</script>

<BitsTabs.Root bind:value class={`underlay-tabs underlay-tabs--${variant} underlay-tabs--${size} ${isCollapsed ? "underlay-tabs--collapsed" : ""} ${className ?? ""}`}>
  {@render children?.()}
</BitsTabs.Root>
