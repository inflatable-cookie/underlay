<script lang="ts">
  import { Tabs as BitsTabs } from "bits-ui";
  import type { Snippet } from "svelte";
  import { setContext, onMount } from "svelte";
  import { browser } from "$app/environment";
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";

  export type TabsVariant = "pills" | "boxed";
  export type TabsSize = "default" | "sm";

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
    children?: Snippet;
  }

  let { value = $bindable(), variant = "pills", size = "default", historyKey, children }: Props =
    $props();

  // Collapsed state (set by TabsList when in collapsible mode)
  let isCollapsed = $state(false);

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

  // Track the last value we synced to URL to prevent loops
  let lastSyncedValue = $state<string | null>(null);

  // Read initial tab from URL on mount
  onMount(() => {
    if (!browser || !historyKey) return;

    const urlValue = $page.url.searchParams.get(historyKey);
    if (urlValue) {
      value = urlValue;
      lastSyncedValue = urlValue;
    } else {
      // Set initial value in URL without creating history entry
      const url = new URL($page.url);
      url.searchParams.set(historyKey, value);
      lastSyncedValue = value;
      goto(url.toString(), { replaceState: true, keepFocus: true, noScroll: true });
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

    const url = new URL($page.url);
    url.searchParams.set(historyKey, value);
    lastSyncedValue = value;
    goto(url.toString(), { replaceState: true, keepFocus: true, noScroll: true });
  });
</script>

<BitsTabs.Root bind:value class={`underlay-tabs underlay-tabs--${variant} underlay-tabs--${size} ${isCollapsed ? "underlay-tabs--collapsed" : ""}`}>
  {@render children?.()}
</BitsTabs.Root>
