<script lang="ts">
  import { Code } from "@inflatable-cookie/poodle-svelte";
  import type { ControlDensity, ControlSize } from "@inflatable-cookie/poodle-svelte";
  import { default as AdminPill } from "./AdminPill.svelte";
  import {
    getMediaKindLabel,
    getMediaVisibilityLabel,
    MediaKind,
    MediaVisibility,
    type MediaKind as MediaKindType,
    type MediaVisibility as MediaVisibilityType
  } from "../runtime/media";
  import type { BreadcrumbItem } from "../patterns/types";
  import type { DetailMetaItemConfig, DetailTabConfig, FetchFn, TemplateSurface } from "./template.types";
  import { default as EntityDetailPage } from "./EntityDetailPage.svelte";

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
    tabsSize?: ControlSize | null;
    tabsDensity?: ControlDensity | null;
    tabsCollapseWhenOverflow?: boolean | null;
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
    tabsVariant = "underline",
    tabsSize = null,
    tabsDensity = null,
    tabsCollapseWhenOverflow = null,
    tabsHistoryKey,
    keepMountedTabs = false,
    headerActions
  }: Props = $props();

  const title = $derived(item?.title || item?.originalFilename || "Untitled");

  const headerMeta = $derived.by<DetailMetaItemConfig[]>(() => {
    const meta: DetailMetaItemConfig[] = [
      {
        label: "ID",
        value: idSnippet
      },
      {
        label: "Kind",
        value: kindSnippet,
        separator: false
      },
      {
        label: "Visibility",
        value: visibilitySnippet,
        separator: false
      }
    ];

    if (item?.deletedAt) {
      meta.push({
        label: "State",
        value: deletedSnippet,
        separator: false
      });
    }

    return [...meta, ...extraMeta];
  });

  function normalizeMediaKind(kind: string): MediaKindType {
    if (kind === MediaKind.Image) return MediaKind.Image;
    if (kind === MediaKind.Audio) return MediaKind.Audio;
    if (kind === MediaKind.Video) return MediaKind.Video;
    if (kind === MediaKind.Pdf) return MediaKind.Pdf;
    if (kind === MediaKind.Document) return MediaKind.Document;
    return MediaKind.Other;
  }

  function normalizeMediaVisibility(visibility: string): MediaVisibilityType {
    if (visibility === MediaVisibility.Public) return MediaVisibility.Public;
    return MediaVisibility.Restricted;
  }

  function mediaKindLabel(kind: string): string {
    return getMediaKindLabel(normalizeMediaKind(kind));
  }

  function mediaVisibilityLabel(visibility: string): string {
    return getMediaVisibilityLabel(normalizeMediaVisibility(visibility));
  }
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
  {tabsDensity}
  {tabsCollapseWhenOverflow}
  tabsHistoryKey={tabsHistoryKey}
  {keepMountedTabs}
  {headerActions}
/>

{#snippet idSnippet()}
  {#if item}
    <Code inline inlineVariant="plain" typography="inline" source={item.id} showCopyButton size="md" />
  {/if}
{/snippet}

{#snippet kindSnippet()}
  {#if item}
    <AdminPill label={mediaKindLabel(item.kind)} typography="inherit" />
  {/if}
{/snippet}

{#snippet visibilitySnippet()}
  {#if item}
    <AdminPill label={mediaVisibilityLabel(item.visibility)} typography="inherit" />
  {/if}
{/snippet}

{#snippet deletedSnippet()}
  <AdminPill kind="danger" label="deleted" typography="inherit" />
{/snippet}
