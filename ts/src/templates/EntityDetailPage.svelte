<script lang="ts">
  import { onMount } from "svelte";
  import type { Snippet } from "svelte";
  import type { BreadcrumbItem } from "../patterns/types";
  import { useAuthenticatedData } from "../runtime/auth";
  import { getBackButtonInfo } from "../patterns/navigation";
  import {
    PageHeader,
    Breadcrumbs,
    Tabs,
    AlertDialog,
    IconButton,
    Menu,
    MetaBar,
    PageLoading,
    Callout,
    Button
  } from "@inflatable-cookie/poodle-svelte";
  import type { ControlDensity, ControlSize } from "@inflatable-cookie/poodle-svelte";
  import { default as EntityMetaItem } from "./EntityMetaItem.svelte";
  import DetailDataTab from "./DetailDataTab.svelte";
  import type {
    DetailActionConfig,
    FetchFn,
    DetailMetaItemConfig,
    DetailTabConfig,
    TemplateSurface
  } from "./template.types";

  interface Props {
    /** Entity name or item title */
    title: string;
    
    /** Entity type label (e.g., "Project", "User") */
    section?: string;

    /** Optional eyebrow above the title */
    eyebrow?: string;

    /** Keep subtitle visible even when breadcrumbs are present */
    showSubtitleWithBreadcrumbs?: boolean;

    /** Heading level for nested composition */
    headerLevel?: 1 | 2 | 3 | 4 | 5 | 6;

    /** Optional breadcrumb trail above the header title */
    breadcrumbs?: BreadcrumbItem[];

    /** Optional subtitle below the title */
    subtitle?: string;

    /** Mark the last breadcrumb as the current page */
    breadcrumbsMarkLastCurrent?: boolean;
    
    /** Back link URL */
    backHref?: string;
    
    /** Back link label */
    backLabel?: string;

    /** Explicit contextual-back state override when the caller resolved back info already. */
    backIsContextual?: boolean;

    /** Resolve contextual back navigation from shared Underlay navigation state. */
    resolveBackContext?: boolean;
    
    /** Banner message (e.g., for warnings) */
    bannerMessage?: string;
    
    /** Banner tone */
    bannerTone?: "warning" | "info" | "danger" | "success";
    
    /** Optional preloaded item. When present, the template uses it directly. */
    item?: T | null;

    /** Optional external loading state override. */
    loading?: boolean;

    /** Optional external error state override. */
    error?: string | null;

    /** Optional retry handler for the error state. */
    onRetry?: () => void;

    /** Loading message override. */
    loadingMessage?: string;

    /** Error title override. */
    errorTitle?: string;

    /** Data loading function */
    dataLoader?: (fetch: FetchFn, token: string | null) => Promise<T | null>;

    /** Change this to force a refetch of the primary entity */
    reloadKey?: string | number | null;
    
    /** Metadata items */
    meta?: DetailMetaItemConfig[];
    
    /** Tabs configuration */
    tabs?: DetailTabConfig<T>[];

    /** Non-tab detail body for single-surface pages */
    content?: TemplateSurface;

    /** Notified when the active tab changes */
    onTabChange?: (tabId: string) => void;

    /** Tabs visual variant */
    tabsVariant?: "underline" | "card";

    /** Tabs size */
    tabsSize?: ControlSize | null;

    /** Tabs density */
    tabsDensity?: ControlDensity | null;

    /** Collapse tabs into a menu when the strip no longer fits */
    tabsCollapseWhenOverflow?: boolean | null;

    /** Optional tab history key passthrough for browser history persistence */
    tabsHistoryKey?: string;

    /** Keep visited tabs mounted after first activation */
    keepMountedTabs?: boolean;

    /** Page actions */
    actions?: DetailActionConfig[];

    /** Fully custom header actions surface */
    headerActions?: TemplateSurface;
  }

  type T = $$Generic;

  // --- Props ---

  let {
    title,
    section,
    eyebrow,
    showSubtitleWithBreadcrumbs = false,
    headerLevel = 2,
    breadcrumbs: detailBreadcrumbs = [],
    subtitle,
    breadcrumbsMarkLastCurrent = true,
    backHref,
    backLabel,
    backIsContextual = false,
    resolveBackContext = true,
    bannerMessage,
    bannerTone = "warning",
    item: providedItem = null,
    loading: providedLoading,
    error: providedError,
    onRetry,
    loadingMessage = "Loading...",
    errorTitle = "Unable to load details",
    dataLoader,
    reloadKey = null,
    meta: detailMeta = [],
    tabs = [],
    content,
    onTabChange,
    tabsVariant = "underline",
    tabsSize = null,
    tabsDensity = null,
    tabsCollapseWhenOverflow = null,
    tabsHistoryKey,
    keepMountedTabs = false,
    actions: pageActions = [],
    headerActions
  }: Props = $props();

  // --- State ---

  let activeTab = $state("");
  const mountedTabsSet = new Set<string>();
  let mountedTabsVersion = $state(0);
  let showConfirmDialog = $state(false);
  let pendingAction: DetailActionConfig | null = $state(null);
  let isNarrowViewport = $state(false);
  const pageActionItems = $derived(
    pageActions.map((action, index) => ({
      value: `action-${index}`,
      label: action.label,
      tone: (action.tone === "danger" ? "danger" : "default") as "default" | "danger"
    }))
  );
  const headerActionSize = $derived<ControlSize>(isNarrowViewport ? "sm" : "md");

  // --- Actions ---

  function handleAction(action: DetailActionConfig) {
    if (action.confirm) {
      pendingAction = action;
      showConfirmDialog = true;
    } else {
      action.handler();
    }
  }

  function handlePageActionSelect(actionValue: string) {
    const actionIndex = Number(actionValue.replace("action-", ""));
    const action = pageActions[actionIndex];
    if (action) {
      handleAction(action);
    }
  }

  function handleConfirm() {
    if (pendingAction) {
      pendingAction.handler();
      pendingAction = null;
    }
    showConfirmDialog = false;
  }

  function handleCancel() {
    pendingAction = null;
    showConfirmDialog = false;
  }

  async function loadPageData(fetch: FetchFn, token: string | null) {
    if (!dataLoader) return providedItem;
    return await dataLoader(fetch, token);
  }

  const pageData = useAuthenticatedData<T | null>(
    loadPageData,
    { defaultValue: null }
  );

  const item = $derived(providedItem ?? pageData?.data ?? null);
  const loading = $derived(
    typeof providedLoading === "boolean"
      ? providedLoading
      : dataLoader
        ? (pageData?.loading ?? false)
        : false
  );
  const error = $derived(providedError ?? (dataLoader ? (pageData?.error ?? null) : null));
  const allTabs = $derived<DetailTabConfig<T>[]>(tabs);
  const resolvedTabsSize = $derived<ControlSize>(
    tabsSize ?? (tabsVariant === "underline" ? "xs" : "sm")
  );
  const resolvedTabsDensity = $derived<ControlDensity | null>(
    tabsDensity ?? (tabsVariant === "underline" ? "compact" : null)
  );
  const resolvedTabsCollapseWhenOverflow = $derived(
    tabsCollapseWhenOverflow ?? (tabsVariant === "underline")
  );
  const resolvedTabsHistoryKey = $derived<string | null>(
    tabsHistoryKey === undefined ? "tab" : tabsHistoryKey
  );
  const shouldShowSubtitleWithBreadcrumbs = $derived(showSubtitleWithBreadcrumbs || detailBreadcrumbs.length === 0);
  let resolvedBackInfo = $state<{ href: string; label: string; contextual: boolean } | null>(null);
  let previousReloadKey = $state<string | number | null>(null);

  $effect(() => {
    if (!backHref) {
      resolvedBackInfo = null;
      return;
    }

    const fallbackLabel = backLabel ?? "Back";
    if (!resolveBackContext) {
      resolvedBackInfo = {
        href: backHref,
        label: fallbackLabel,
        contextual: backIsContextual
      };
      return;
    }

    const contextualBackInfo = getBackButtonInfo(fallbackLabel, backHref);
    resolvedBackInfo = {
      href: contextualBackInfo.href,
      label: contextualBackInfo.label,
      contextual: Boolean(contextualBackInfo.isContextual || backIsContextual)
    };
  });

  $effect(() => {
    const firstTabId = allTabs[0]?.id ?? "";
    const hasActiveTab = allTabs.some((tab) => tab.id === activeTab);
    if (!hasActiveTab) {
      activeTab = getUrlTabValue() ?? firstTabId;
    }
  });

  $effect(() => {
    if (previousReloadKey === null) {
      previousReloadKey = reloadKey;
      return;
    }

    if (reloadKey !== previousReloadKey) {
      previousReloadKey = reloadKey;
      void pageData?.refetch();
    }
  });

  $effect(() => {
    if (activeTab) {
      onTabChange?.(activeTab);
    }
  });

  $effect(() => {
    if (keepMountedTabs && activeTab && !mountedTabsSet.has(activeTab)) {
      mountedTabsSet.add(activeTab);
      mountedTabsVersion++;
    }
  });

  function shouldRenderTab(tabId: string): boolean {
    if (tabId === activeTab) return true;
    if (!keepMountedTabs) return false;
    void mountedTabsVersion;
    return mountedTabsSet.has(tabId);
  }

  function handleRetry() {
    if (onRetry) {
      onRetry();
      return;
    }

    if (dataLoader) {
      void pageData?.refetch();
    }
  }

  function getDefaultTabId(): string | null {
    return allTabs[0]?.id ?? null;
  }

  function getUrlTabValue(): string | null {
    if (typeof window === "undefined" || !resolvedTabsHistoryKey) return null;

    const nextValue = new URL(window.location.href).searchParams.get(resolvedTabsHistoryKey);
    if (!nextValue) return null;
    return allTabs.some((tab) => tab.id === nextValue) ? nextValue : null;
  }

  function replaceUrlTabValue(tabId: string): void {
    if (typeof window === "undefined" || !resolvedTabsHistoryKey) return;

    const defaultTabId = getDefaultTabId();
    const url = new URL(window.location.href);
    const currentValue = url.searchParams.get(resolvedTabsHistoryKey);
    const nextValue = tabId === defaultTabId ? null : tabId;

    if (currentValue === nextValue) return;

    if (nextValue) {
      url.searchParams.set(resolvedTabsHistoryKey, nextValue);
    } else {
      url.searchParams.delete(resolvedTabsHistoryKey);
    }

    window.history.replaceState(window.history.state, "", url);
  }

  function syncTabFromUrl(): void {
    const nextTab = getUrlTabValue() ?? getDefaultTabId();
    if (!nextTab) return;
    activeTab = nextTab;
  }

  function handleTabValueChange(value: string): void {
    activeTab = value;
    replaceUrlTabValue(value);
  }

  onMount(() => {
    const mediaQuery = window.matchMedia("(max-width: 45rem)");
    const sync = () => {
      isNarrowViewport = mediaQuery.matches;
    };

    sync();
    mediaQuery.addEventListener("change", sync);

    return () => {
      mediaQuery.removeEventListener("change", sync);
    };
  });

  onMount(() => {
    if (!resolvedTabsHistoryKey) return;

    syncTabFromUrl();

    const handlePopState = () => {
      syncTabFromUrl();
    };

    window.addEventListener("popstate", handlePopState);

    return () => {
      window.removeEventListener("popstate", handlePopState);
    };
  });
</script>

{#if loading}
  <PageLoading presentation="inline" message={loadingMessage} />
{:else if error}
  <Callout tone="danger" title={errorTitle} message={error} announceMode="polite">
    {#if onRetry || dataLoader}
      {#snippet actions()}
        <Button type="button" variant="ghost" size="sm" onClick={handleRetry}>
          Retry
        </Button>
      {/snippet}
    {/if}
  </Callout>
{:else if item}
  <div class="underlay-entity-detail-page">
    <PageHeader
      {title}
      {section}
      {eyebrow}
      {subtitle}
      showSubtitleWithBreadcrumbs={shouldShowSubtitleWithBreadcrumbs}
      backHref={resolvedBackInfo?.href ?? null}
      backLabel={resolvedBackInfo?.label ?? backLabel}
      backIsContextual={resolvedBackInfo?.contextual ?? false}
      bannerMessage={bannerMessage}
      bannerTone={bannerTone}
      level={headerLevel}
    >
      {#snippet breadcrumbs()}
        {#if detailBreadcrumbs.length > 0}
          <Breadcrumbs
            items={detailBreadcrumbs.map((crumb, index) => ({
              value: crumb.href ?? crumb.label,
              label: crumb.label,
              href: crumb.href,
              current: breadcrumbsMarkLastCurrent && index === detailBreadcrumbs.length - 1
            }))}
            forceLastItemCurrent={breadcrumbsMarkLastCurrent}
            sizeRole="chrome"
          />
        {/if}
      {/snippet}

      {#snippet meta()}
        {#if detailMeta.length > 0}
          <MetaBar ariaLabel="Detail metadata">
            {#each detailMeta as metaItem}
              <EntityMetaItem item={metaItem} />
            {/each}
          </MetaBar>
        {/if}
      {/snippet}

      {#snippet actions()}
        {#if headerActions}
          {@render headerActions(item)}
        {:else if pageActionItems.length > 0}
          <Menu
            items={pageActionItems}
            ariaLabel={`${section ?? title} actions`}
            triggerAriaLabel={`${section ?? title} actions`}
            onAction={handlePageActionSelect}
          >
            {#snippet trigger()}
              <IconButton
                type="button"
                icon="ellipsis"
                variant="secondary"
                size={headerActionSize}
                ariaLabel={`${section ?? title} actions`}
                tooltip="Actions"
              />
            {/snippet}
          </Menu>
        {/if}
      {/snippet}
    </PageHeader>

    {#if allTabs.length > 0}
      <!-- `tabsVariant` stays underlay's own vocabulary: it also drives size,
           density, collapse and the panel class below, and callers pass it.
           Poodle consolidated its union to card|pill|block, where "card" is the
           former "text"/"underline" look and the former "card" is reproduced by
           pill + activeEdge="outline". Mapped here so the two vocabularies stay
           decoupled and both looks survive. -->
      <Tabs
        value={activeTab}
        items={allTabs.map((tab) => ({
          value: tab.id,
          label: tab.label,
          count: tab.count,
          separator: tab.id === allTabs[allTabs.length - 1]?.id ? false : tab.separator
        }))}
        variant={tabsVariant === "underline" ? "card" : "pill"}
        bordered={tabsVariant === "underline"}
        activeEdge={tabsVariant === "card" ? "outline" : "none"}
        size={resolvedTabsSize}
        density={resolvedTabsDensity}
        collapseWhenOverflow={resolvedTabsCollapseWhenOverflow}
        ariaLabel={`${title} sections`}
        onValueChange={handleTabValueChange}
      >
        {#snippet children(activeValue)}
        {#each allTabs as tab}
          {#if tab.content && shouldRenderTab(tab.id)}
            <div
              class:underlay-entity-detail-page__tab-panel={tabsVariant === "underline"}
              hidden={tab.id !== activeValue}
            >
              {@render tab.content(item)}
            </div>
          {:else if tab.dataLoader && tab.render && shouldRenderTab(tab.id)}
            <div
              class:underlay-entity-detail-page__tab-panel={tabsVariant === "underline"}
              hidden={tab.id !== activeValue}
            >
              <DetailDataTab {item} loader={tab.dataLoader} render={tab.render} />
            </div>
          {/if}
        {/each}
        {/snippet}
      </Tabs>
    {:else if content}
      {@render content(item)}
    {/if}
  </div>
{/if}

<!-- Action confirmation dialog -->
{#if showConfirmDialog && pendingAction}
  <AlertDialog
    open={true}
    title={typeof pendingAction.confirm === "object" 
      ? pendingAction.confirm.title 
      : `${pendingAction.label}?`}
    description={typeof pendingAction.confirm === "object" 
      ? pendingAction.confirm.description 
      : `Are you sure you want to ${pendingAction.label.toLowerCase()}?`}
    confirmLabel={typeof pendingAction.confirm === "object"
      ? pendingAction.confirm.confirmLabel ?? pendingAction.label
      : pendingAction.label}
    cancelLabel={typeof pendingAction.confirm === "object"
      ? pendingAction.confirm.cancelLabel ?? "Cancel"
      : "Cancel"}
    tone={pendingAction.tone === "danger" ? "danger" : "warning"}
    onConfirm={handleConfirm}
    onCancel={handleCancel}
  />
{/if}

<style>
  .underlay-entity-detail-page {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-4, 1rem);
  }

  .underlay-entity-detail-page__tab-panel {
    padding-top: var(--underlay-space-3, 0.75rem);
  }
</style>
