<script lang="ts">
  import { Code, Pill } from "@poodle/svelte";
  import { getMediaKindLabel, getMediaVisibilityLabel } from "../runtime/media";
  import type { BreadcrumbItem } from "../patterns/types";
  import type { DetailMetaItemConfig, DetailTabConfig, FetchFn, TemplateSurface } from "./template.types";
  import EntityDetailPage from "./EntityDetailPage.svelte";

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
    backIsContextual?: boolean;
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
    item = null,
    loading,
    error,
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
    backIsContextual = false,
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

  const title = $derived(item?.title || item?.originalFilename || "Untitled");

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
</script>

<EntityDetailPage
  {title}
  {section}
  {eyebrow}
  {subtitle}
  {showSubtitleWithBreadcrumbs}
  headerLevel={headerLevel}
  breadcrumbs={detailBreadcrumbs}
  {breadcrumbsMarkLastCurrent}
  {backHref}
  {backLabel}
  {backIsContextual}
  bannerMessage={item?.deletedAt ? deletedBannerMessage : undefined}
  bannerTone={deletedBannerTone}
  {item}
  {loading}
  {error}
  {onRetry}
  {loadingMessage}
  {errorTitle}
  {dataLoader}
  {reloadKey}
  meta={headerMeta}
  {tabs}
  {content}
  {onTabChange}
  {tabsVariant}
  {tabsSize}
  tabsHistoryKey={tabsHistoryKey}
  {keepMountedTabs}
  {headerActions}
/>

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
