<script lang="ts">
  import type { Snippet } from "svelte";
  import PageHeader from "../PageHeader.svelte";
  import TabsRoot from "../../components/TabsRoot.svelte";
  import TabsList from "../../components/TabsList.svelte";
  import TabsTrigger from "../../components/TabsTrigger.svelte";
  import TabsSeparator from "../../components/TabsSeparator.svelte";
  import TabsContent from "../../components/TabsContent.svelte";
  import type { BreadcrumbItem, PageHeaderLevel } from "../types";
  import type { BannerVariant } from "../banner";

  interface TabConfig {
    value: string;
    label: string;
    count?: number;
    /** Render a vertical separator before this tab */
    separator?: boolean;
  }

  interface Props {
    /** Primary heading text */
    title?: string;
    /** Section name — renders as the prominent h1 */
    section?: string;
    /** Subtitle text (e.g., slug) */
    subtitle?: string;
    /** Breadcrumb trail */
    breadcrumbs?: BreadcrumbItem[];
    /** Heading level */
    level?: PageHeaderLevel;
    /** Back button href */
    backHref?: string | null;
    /** Back button label */
    backLabel?: string;
    /** True when back navigation came from context */
    backIsContextual?: boolean;
    /** Banner message below header */
    bannerMessage?: string;
    /** Banner variant */
    bannerVariant?: BannerVariant;
    /** Title suffix (e.g., Pill, Badge) */
    titleSuffix?: Snippet;

    /** Metadata row (use DetailMeta sub-components) */
    meta?: Snippet;
    /** Actions menu snippet */
    actions?: Snippet;

    /** Tab definitions for tabbed layout */
    tabs?: TabConfig[];
    /** Current active tab (bindable) */
    activeTab?: string;
    /** History key for URL tab sync (default: "tab") */
    tabsHistoryKey?: string;

    /** Content for each tab — receives the tab value */
    tabContent?: Snippet<[string]>;
    /** Content when no tabs are defined */
    children?: Snippet;

    /** Additional CSS class */
    class?: string;
  }

  let {
    title = undefined,
    section = undefined,
    subtitle = undefined,
    breadcrumbs = undefined,
    level = undefined,
    backHref = undefined,
    backLabel = undefined,
    backIsContextual = false,
    bannerMessage = undefined,
    bannerVariant = undefined,
    titleSuffix = undefined,
    meta,
    actions,
    tabs = undefined,
    activeTab = $bindable(""),
    tabsHistoryKey = "tab",
    tabContent,
    children,
    class: className = ""
  }: Props = $props();

  // Default active tab to first tab
  $effect(() => {
    if (tabs && tabs.length > 0 && !activeTab) {
      activeTab = tabs[0].value;
    }
  });

  /**
   * Track which tabs have been visited so we can lazy-mount their content.
   * Once a tab is activated it stays mounted to preserve component state.
   *
   * We use a plain (non-reactive) Set and a reactive counter to avoid an
   * effect that reads and writes the same state (which triggers Svelte's
   * effect_update_depth_exceeded error).
   */
  const mountedTabsSet = new Set<string>();
  let mountedTabsVersion = $state(0);

  $effect(() => {
    if (activeTab && !mountedTabsSet.has(activeTab)) {
      mountedTabsSet.add(activeTab);
      mountedTabsVersion++;
    }
  });

  function isTabMounted(value: string): boolean {
    // Read the reactive counter to subscribe to changes
    void mountedTabsVersion;
    return mountedTabsSet.has(value);
  }
</script>

<div class="underlay-detail-page {className}">
  <PageHeader
    {title}
    {section}
    {subtitle}
    {breadcrumbs}
    {level}
    {backHref}
    {backLabel}
    {backIsContextual}
    {bannerMessage}
    {bannerVariant}
    {titleSuffix}
    {actions}
  >
    {#if meta}
      {@render meta()}
    {/if}
  </PageHeader>

  {#if tabs && tabs.length > 0 && tabContent}
    <TabsRoot bind:value={activeTab} variant="boxed" size="sm" historyKey={tabsHistoryKey}>
      <TabsList>
        {#each tabs as tab (tab.value)}
          {#if tab.separator}
            <TabsSeparator />
          {/if}
          <TabsTrigger value={tab.value} count={tab.count}>
            {tab.label}
          </TabsTrigger>
        {/each}
      </TabsList>

      {#each tabs as tab (tab.value)}
        <TabsContent value={tab.value}>
          {#if isTabMounted(tab.value)}
            {@render tabContent(tab.value)}
          {/if}
        </TabsContent>
      {/each}
    </TabsRoot>
  {:else if children}
    {@render children()}
  {/if}
</div>

<style>
  :global(.underlay-detail-page) {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>
