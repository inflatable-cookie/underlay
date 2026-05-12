<script lang="ts">
  import { Code, Pill, PageHeader, Breadcrumbs, Tabs, PageLoading, Callout, Button, MetaBar, MetaItem } from "@poodle/svelte";
  import { useAuthenticatedData } from "../runtime/auth";
  import { getMediaKindLabel, getMediaVisibilityLabel } from "../runtime/media";
  import type { BreadcrumbItem } from "../patterns/types";
  import type { DetailMetaItemConfig, DetailTabConfig, FetchFn, TemplateSurface } from "./template.types";

  interface MediaDetailLike {
    id: string;
    title?: string | null;
    originalFilename?: string | null;
    kind: string;
    visibility: string;
    deletedAt?: string | null;
  }

  interface Props {
    item?: MediaDetailLike | null;
    loading?: boolean;
    error?: string | null;
    onRetry?: () => void;
    dataLoader?: (fetch: FetchFn, token: string | null) => Promise<MediaDetailLike | null>;
    reloadKey?: string | number | null;
    section?: string;
    eyebrow?: string;
    subtitle?: string;
    showSubtitleWithBreadcrumbs?: boolean;
    headerLevel?: 1 | 2 | 3 | 4 | 5 | 6;
    breadcrumbs?: BreadcrumbItem[];
    breadcrumbsMarkLastCurrent?: boolean;
    backHref?: string;
    backLabel?: string;
    deletedBannerMessage?: string;
    deletedBannerTone?: "warning" | "info" | "danger" | "success";
    loadingMessage?: string;
    errorTitle?: string;
    meta?: DetailMetaItemConfig[];
    tabs?: DetailTabConfig<MediaDetailLike>[];
    content?: TemplateSurface;
    onTabChange?: (tabId: string) => void;
    tabsVariant?: "underline" | "card";
    tabsSize?: "sm" | "md" | "lg";
    tabsHistoryKey?: string;
    keepMountedTabs?: boolean;
    headerActions?: TemplateSurface;
  }

  let {
    item: providedItem = null,
    loading: providedLoading,
    error: providedError,
    onRetry,
    dataLoader,
    reloadKey = null,
    section = "Media",
    eyebrow,
    subtitle,
    showSubtitleWithBreadcrumbs = false,
    headerLevel = 2,
    breadcrumbs: detailBreadcrumbs = [],
    breadcrumbsMarkLastCurrent = true,
    backHref,
    backLabel,
    deletedBannerMessage = "This media has been soft-deleted.",
    deletedBannerTone = "warning",
    loadingMessage = "Loading media...",
    errorTitle = "Unable to load media",
    meta: extraMeta = [],
    tabs = [],
    content,
    onTabChange,
    tabsVariant = "card",
    tabsSize = "sm",
    tabsHistoryKey,
    keepMountedTabs = false,
    headerActions
  }: Props = $props();

  const pageData = useAuthenticatedData<MediaDetailLike | null>(
    async (fetch, token) => {
      if (!dataLoader) return providedItem;
      return await dataLoader(fetch, token);
    },
    { defaultValue: null }
  );

  const item = $derived(providedItem ?? pageData.data ?? null);
  const loading = $derived(
    typeof providedLoading === "boolean"
      ? providedLoading
      : dataLoader
        ? (pageData.loading ?? false)
        : false
  );
  const error = $derived(
    providedError ?? (dataLoader ? (pageData.error ?? null) : null)
  );
  const title = $derived(item?.title || item?.originalFilename || "Untitled");
  const allTabs = $derived<DetailTabConfig<MediaDetailLike>[]>(tabs);
  let activeTab = $state("");
  const mountedTabsSet = new Set<string>();
  let mountedTabsVersion = $state(0);
  let previousReloadKey = $state<string | number | null>(null);

  const headerMeta = $derived<DetailMetaItemConfig[]>([
    {
      label: "ID",
      value: idSnippet as never
    },
    {
      label: "",
      value: kindSnippet as never,
      separator: false
    },
    {
      label: "",
      value: visibilitySnippet as never,
      separator: false
    },
    ...(item?.deletedAt
      ? [{
          label: "",
          value: deletedSnippet as never,
          separator: false
        }]
      : []),
    ...extraMeta
  ]);

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

    if (reloadKey !== previousReloadKey && dataLoader) {
      previousReloadKey = reloadKey;
      void pageData.refetch();
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
      void pageData.refetch();
    }
  }
</script>

{#if loading}
  <PageLoading presentation="inline" message={loadingMessage} />
{:else if error}
  <Callout tone="danger" title={errorTitle} message={error}>
    {#if onRetry || dataLoader}
      <Button slot="actions" type="button" variant="ghost" size="sm" onclick={handleRetry}>
        Retry
      </Button>
    {/if}
  </Callout>
{:else if item}
  <div class="underlay-media-detail-page">
    <PageHeader
      title={title}
      {section}
      {eyebrow}
      {subtitle}
      {showSubtitleWithBreadcrumbs}
      backHref={backHref ?? null}
      backLabel={backLabel}
      bannerMessage={item.deletedAt ? deletedBannerMessage : undefined}
      bannerTone={deletedBannerTone}
      level={headerLevel}
      posture="entity-detail"
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
        <MetaBar ariaLabel="Media metadata">
          {#each headerMeta as metaItem}
            <MetaItem label={metaItem.label} separator={metaItem.separator ?? true}>
              {#if typeof metaItem.value === "string"}
                {metaItem.value}
              {:else}
                {@render metaItem.value()}
              {/if}
            </MetaItem>
          {/each}
        </MetaBar>
      {/snippet}

      {#snippet actions()}
        {#if headerActions}
          {@render headerActions(item)}
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
        historyKey={tabsHistoryKey}
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
    {:else if content}
      {@render content(item)}
    {/if}
  </div>
{/if}

{#snippet idSnippet()}
  {#if item}
    <Code inline inlineVariant="plain" typography="inline" source={item.id} showCopyButton />
  {/if}
{/snippet}

{#snippet kindSnippet()}
  {#if item}
    <Pill tone="neutral" appearance="badge" size="sm" typography="inherit">
      {getMediaKindLabel(item.kind as never)}
    </Pill>
  {/if}
{/snippet}

{#snippet visibilitySnippet()}
  {#if item}
    <Pill tone="neutral" appearance="badge" size="sm" typography="inherit">
      {getMediaVisibilityLabel(item.visibility as never)}
    </Pill>
  {/if}
{/snippet}

{#snippet deletedSnippet()}
  <Pill tone="danger" appearance="badge" size="sm" typography="inherit">
    Deleted
  </Pill>
{/snippet}

<style>
  .underlay-media-detail-page {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-4, 1rem);
  }
</style>
