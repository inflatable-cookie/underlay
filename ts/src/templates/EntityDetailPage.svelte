<script lang="ts">
  import type { Snippet } from "svelte";
  import type { BreadcrumbItem } from "../patterns/types";
  import { useAuthenticatedData } from "../runtime/auth";
  import {
    PageHeader,
    Breadcrumbs,
    Tabs,
    AlertDialog,
    IconButton,
    Menu,
    MetaBar,
    MetaItem,
    PageLoading,
    Callout
  } from "@poodle/svelte";
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
    
    /** Banner message (e.g., for warnings) */
    bannerMessage?: string;
    
    /** Banner tone */
    bannerTone?: "warning" | "info" | "danger" | "success";
    
    /** Optional preloaded item. When present, the template uses it directly. */
    item?: T | null;

    /** Data loading function */
    dataLoader?: (fetch: FetchFn, token: string | null) => Promise<T | null>;

    /** Change this to force a refetch of the primary entity */
    reloadKey?: string | number | null;
    
    /** Metadata items */
    meta?: DetailMetaItemConfig[];
    
    /** Tabs configuration */
    tabs?: DetailTabConfig<T>[];

    /** Notified when the active tab changes */
    onTabChange?: (tabId: string) => void;

    /** Tabs visual variant */
    tabsVariant?: "underline" | "card";

    /** Tabs size */
    tabsSize?: "sm" | "md" | "lg";

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
    bannerMessage,
    bannerTone = "warning",
    item: providedItem = null,
    dataLoader,
    reloadKey = null,
    meta: detailMeta = [],
    tabs = [],
    onTabChange,
    tabsVariant = "underline",
    tabsSize = "sm",
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
  const pageActionItems = $derived(
    pageActions.map((action, index) => ({
      value: `action-${index}`,
      label: action.label,
      tone: (action.tone === "danger" ? "danger" : "default") as "default" | "danger"
    }))
  );

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

  const pageData = useAuthenticatedData<T | null>(
    async (fetch, token) => {
      if (!dataLoader) return providedItem;
      return await dataLoader(fetch, token);
    },
    { defaultValue: null }
  );

  const item = $derived(providedItem ?? pageData?.data ?? null);
  const loading = $derived(dataLoader ? (pageData?.loading ?? false) : false);
  const error = $derived(dataLoader ? (pageData?.error ?? null) : null);
  const allTabs = $derived<DetailTabConfig<T>[]>(tabs);
  let previousReloadKey = $state<string | number | null>(null);

  $effect(() => {
    const firstTabId = allTabs[0]?.id ?? "";
    const hasActiveTab = allTabs.some((tab) => tab.id === activeTab);
    if (!hasActiveTab) {
      activeTab = firstTabId;
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
</script>

{#if loading}
  <PageLoading presentation="inline" message="Loading..." />
{:else if error}
  <Callout tone="danger" message={error} announceMode="polite" />
{:else if item}
  <div class="underlay-entity-detail-page">
    <PageHeader
      {title}
      {section}
      {eyebrow}
      {subtitle}
      {showSubtitleWithBreadcrumbs}
      backHref={backHref ?? null}
      backLabel={backLabel}
      bannerMessage={bannerMessage}
      bannerTone={bannerTone}
      level={headerLevel}
      posture={section ? "entity-detail" : "default"}
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
              <MetaItem label={metaItem.label} separator={metaItem.separator ?? true}>
                {#if typeof metaItem.value === "string"}
                  {metaItem.value}
                {:else}
                  {@render metaItem.value()}
                {/if}
              </MetaItem>
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
            on:action={(event) => handlePageActionSelect(event.detail.value)}
          >
            <svelte:fragment slot="trigger">
              <IconButton
                type="button"
                icon="ellipsis"
                variant="secondary"
                ariaLabel={`${section ?? title} actions`}
                tooltip="Actions"
              />
            </svelte:fragment>
          </Menu>
        {/if}
      {/snippet}
    </PageHeader>

    {#if allTabs.length > 0}
      <Tabs
        value={activeTab}
        items={allTabs.map((tab) => ({
          value: tab.id,
          label: tab.label,
          count: tab.count,
          separator: tab.separator
        }))}
        variant={tabsVariant}
        size={tabsSize}
        ariaLabel={`${title} sections`}
        on:valueChange={(event) => {
          activeTab = event.detail.value;
        }}
      >
        {#each allTabs as tab}
          {#if tab.content && shouldRenderTab(tab.id)}
            <div hidden={tab.id !== activeTab}>
              {@render tab.content(item)}
            </div>
          {/if}
        {/each}
      </Tabs>
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
</style>
