<script lang="ts">
  import PageHeader from "../../src/patterns/PageHeader.svelte";

  interface Crumb {
    label: string;
    href?: string;
  }

  interface Props {
    title?: string;
    section?: string;
    subtitle?: string;
    level?: 1 | 2 | 3 | 4;
    count?: number;
    backHref?: string | null;
    backLabel?: string;
    backIsContextual?: boolean;
    bannerMessage?: string;
    bannerVariant?: "warning" | "error" | "info" | "success";
    breadcrumbs?: Crumb[];
    withActions?: boolean;
    withTitleSuffix?: boolean;
    withSubtitleSuffix?: boolean;
    withMeta?: boolean;
  }

  let {
    title = "Overview",
    section,
    subtitle = "Subheading",
    level = 1,
    count,
    backHref = null,
    backLabel = "Back",
    backIsContextual = false,
    bannerMessage,
    bannerVariant = "warning",
    breadcrumbs,
    withActions = false,
    withTitleSuffix = false,
    withSubtitleSuffix = false,
    withMeta = false
  }: Props = $props();
</script>

{#snippet actionSnippet()}
  <button data-testid="ph-action">Header Action</button>
{/snippet}

{#snippet titleSuffixSnippet()}
  <span data-testid="ph-title-suffix">TS</span>
{/snippet}

{#snippet subtitleSuffixSnippet()}
  <span data-testid="ph-subtitle-suffix">SS</span>
{/snippet}

{#snippet metaSnippet()}
  <div data-testid="ph-meta">Meta content</div>
{/snippet}

<PageHeader
  {title}
  {section}
  {subtitle}
  {level}
  {count}
  {backHref}
  {backLabel}
  {backIsContextual}
  {bannerMessage}
  {bannerVariant}
  {breadcrumbs}
  actions={withActions ? actionSnippet : undefined}
  titleSuffix={withTitleSuffix ? titleSuffixSnippet : undefined}
  subtitleSuffix={withSubtitleSuffix ? subtitleSuffixSnippet : undefined}
  children={withMeta ? metaSnippet : undefined}
>
</PageHeader>
