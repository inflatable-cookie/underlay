<script lang="ts">
  import type { Snippet } from "svelte";
  import type { BannerVariant } from "./banner";
  import type { PageHeaderLevel, BreadcrumbItem } from "./types";
  import Banner from "./Banner.svelte";
  import ChevronRight from "lucide-svelte/icons/chevron-right";

  interface Props {
    /** Primary heading text. Required when `section` is not set. */
    title?: string;
    /** Section name — renders as the prominent h1. When set, `title` renders as a smaller h2 below. */
    section?: string;
    subtitle?: string;
    /** Breadcrumb trail rendered in the subtitle area */
    breadcrumbs?: BreadcrumbItem[];
    /** Heading level: 1 (page), 2, 3 (section), 4 (subsection) */
    level?: PageHeaderLevel;
    /** Optional count to display in parentheses after the title */
    count?: number;
    backHref?: string | null;
    backLabel?: string;
    /** True when backHref/backLabel came from navigation context */
    backIsContextual?: boolean;

    /** Banner message to display below the header */
    bannerMessage?: string;
    /** Banner variant (warning, error, info) */
    bannerVariant?: BannerVariant;

    actions?: Snippet;
    /** Content to render inline after the title (e.g., a Pill or Badge) */
    titleSuffix?: Snippet;
    /** Content to render inline after the subtitle (e.g., a Pill or Badge) */
    subtitleSuffix?: Snippet;
    children?: Snippet;
  }

  let {
    title,
    section,
    subtitle,
    breadcrumbs,
    level = 1,
    count,
    backHref = null,
    backLabel = "Back",
    backIsContextual = false,
    bannerMessage,
    bannerVariant = "warning",
    actions,
    titleSuffix,
    subtitleSuffix,
    children
  }: Props = $props();

  /** The primary heading text — `section` takes priority, falls back to `title` */
  const primaryHeading = $derived(section ?? title ?? "");
  /** The secondary heading — only shown when both `section` and `title` are set */
  const secondaryHeading = $derived(section && title ? title : undefined);
  const sectionTitleTag = $derived(`h${Math.min(level + 1, 6)}` as "h2" | "h3" | "h4" | "h5");

  const headingTag = $derived(`h${level}` as "h1" | "h2" | "h3" | "h4");
  const subtitleTag = $derived(`h${Math.min(level + 1, 6)}` as "h2" | "h3" | "h4" | "h5");

  function titleCaseWords(input: string): string {
    return input
      .split(" ")
      .map((word) => {
        // Preserve already-cased words (acronyms, IDs, etc.)
        if (/[A-Z]/.test(word)) return word;

        const firstChar = word.at(0);
        if (!firstChar) return word;

        return firstChar.toUpperCase() + word.slice(1);
      })
      .join(" ");
  }

  function formatBackLabel(input: string): string {
    const prefix = "Back to ";
    if (!input.startsWith(prefix)) return input;

    return prefix + titleCaseWords(input.slice(prefix.length));
  }

  function truncateLabel(input: string, maxLength: number): string {
    if (input.length <= maxLength) return input;
    return input.slice(0, maxLength - 1).trimEnd() + "…";
  }

  const formattedBackLabel = $derived(formatBackLabel(backLabel));
  const displayBackLabel = $derived(
    backIsContextual ? truncateLabel(formattedBackLabel, 35) : formattedBackLabel
  );
</script>

<header class="underlay-page-header underlay-page-header--level-{level}">
  <div class="underlay-page-header__row">
    <div class="underlay-page-header__top">
      <svelte:element this={headingTag} class="underlay-page-header__title">
        {primaryHeading}{#if count !== undefined}<span class="underlay-page-header__count-badge">{count}</span>{/if}{#if !secondaryHeading && titleSuffix}<span class="underlay-page-header__title-suffix">{@render titleSuffix()}</span>{/if}
      </svelte:element>

      <div class="underlay-page-header__right">
        {#if backHref}
          <a
            class="underlay-page-header__back underlay-page-header__back--inline"
            href={backHref}
            title={formattedBackLabel}
          >
            ← {displayBackLabel}
            {#if backIsContextual}
              <span class="underlay-page-header__context-dot" aria-hidden="true"></span>
            {/if}
          </a>
        {/if}
        {#if actions}
          <div class="underlay-page-header__actions">{@render actions?.()}</div>
        {/if}
      </div>
    </div>

    {#if secondaryHeading}
      <svelte:element this={sectionTitleTag} class="underlay-page-header__section-title">
        {secondaryHeading}{#if titleSuffix}<span class="underlay-page-header__title-suffix">{@render titleSuffix()}</span>{/if}
      </svelte:element>
    {/if}

    {#if breadcrumbs && breadcrumbs.length > 0}
      <nav class="underlay-page-header__breadcrumbs" aria-label="Breadcrumb">
        {#each breadcrumbs as crumb, i}
          {#if i > 0}<span class="underlay-page-header__breadcrumb-sep" aria-hidden="true"><ChevronRight size={14} /></span>{/if}
          {#if crumb.href}<a href={crumb.href} class="underlay-page-header__breadcrumb-link">{crumb.label}</a>{:else}<span class="underlay-page-header__breadcrumb-current" aria-current="page">{crumb.label}</span>{/if}
        {/each}
      </nav>
    {:else if subtitle || subtitleSuffix}
      <svelte:element this={subtitleTag} class="underlay-page-header__subtitle">{subtitle}{#if subtitleSuffix}<span class="underlay-page-header__subtitle-suffix">{@render subtitleSuffix()}</span>{/if}</svelte:element>
    {/if}
  </div>

  {#if children}
    <div class="underlay-page-header__meta">{@render children?.()}</div>
  {/if}

  {#if bannerMessage}
    <Banner variant={bannerVariant} message={bannerMessage} />
  {/if}

  {#if backHref}
    <a class="underlay-page-header__back underlay-page-header__back--below" href={backHref} title={formattedBackLabel}>
      ← {displayBackLabel}
      {#if backIsContextual}
        <span class="underlay-page-header__context-dot" aria-hidden="true"></span>
      {/if}
    </a>
  {/if}
</header>

<style>
  .underlay-page-header {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-2, 0.5rem);
    margin-bottom: var(--underlay-space-6, 1.5rem);
  }

  .underlay-page-header__back {
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
    text-decoration: none;
    width: fit-content;
    font-size: 0.85em;
    display: inline-flex;
    align-items: center;
  }

  .underlay-page-header__context-dot {
    display: inline-block;
    flex: none;
    width: 0.38rem;
    height: 0.38rem;
    border-radius: 999px;
    background: var(--underlay-color-success, #22c55e);
    margin-left: 0.35rem;
    opacity: 0.9;
  }

  .underlay-page-header__back:hover {
    color: var(--underlay-color-text, #e5e7eb);
    text-decoration: underline;
    text-underline-offset: 0.12em;
  }

  /* Inline back link - visible on wide screens (> 1200px) */
  .underlay-page-header__back--inline {
    display: none;
  }

  /* Below back link - visible on narrower screens */
  .underlay-page-header__back--below {
    display: block;
    margin-top: var(--underlay-space-2, 0.5rem);
  }

  @media (min-width: 1201px) {
    .underlay-page-header__back--inline {
      display: block;
    }

    .underlay-page-header__back--below {
      display: none;
    }
  }

  .underlay-page-header__row {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-1, 0.25rem);
  }

  /* Top row holds h1 + back/actions (aligned to h1) */
  .underlay-page-header__top {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--underlay-space-4, 1rem);
    flex-wrap: wrap;
  }

  .underlay-page-header__title {
    margin: 0;
    font-size: 2em;
    font-weight: 650;
    color: var(--underlay-color-text, #e5e7eb);
    display: inline-flex;
    align-items: center;
  }

  .underlay-page-header__count-badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 1.5em;
    height: 1.5em;
    padding: 0 0.4em;
    margin-left: 0.4em;
    font-size: 0.4em;
    font-weight: 600;
    font-style: normal;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
    background: var(--underlay-color-surface-subtle, rgba(148, 163, 184, 0.1));
    border-radius: 9999px;
    vertical-align: super;
    transform: translateY(-0.3em);
  }

  .underlay-page-header__title-suffix {
    display: inline-flex;
    align-items: center;
    margin-left: 0.4em;
  }

  .underlay-page-header__title-suffix :global(.underlay-pill) {
    font-size: 0.45em;
  }

  .underlay-page-header__section-title {
    margin: 0;
    font-size: 1.35em;
    font-weight: 500;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
    display: inline-flex;
    align-items: center;
  }

  .underlay-page-header__subtitle {
    margin: 0;
    font-size: 1.1em;
    font-weight: 500;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
    display: inline-flex;
    align-items: center;
  }

  .underlay-page-header__subtitle-suffix {
    display: inline-flex;
    align-items: center;
    margin-left: 0.5em;
  }

  .underlay-page-header__subtitle-suffix :global(.underlay-pill) {
    font-size: 0.7em;
  }

  .underlay-page-header__breadcrumbs {
    display: flex;
    align-items: center;
    gap: 0.4em;
    font-size: 0.95em;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
  }

  .underlay-page-header__breadcrumb-link {
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
    text-decoration: none;
  }

  .underlay-page-header__breadcrumb-link:hover {
    color: var(--underlay-color-text, #e5e7eb);
    text-decoration: underline;
    text-underline-offset: 0.12em;
  }

  .underlay-page-header__breadcrumb-current {
    color: var(--underlay-color-text, #e5e7eb);
  }

  .underlay-page-header__breadcrumb-sep {
    display: inline-flex;
    align-items: center;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.5));
  }

  .underlay-page-header__right {
    display: inline-flex;
    align-items: center;
    gap: var(--underlay-space-4, 1rem);
    flex-wrap: wrap;
  }

  .underlay-page-header__actions {
    display: inline-flex;
    align-items: center;
    gap: var(--underlay-space-2, 0.5rem);
    flex-wrap: wrap;
  }

  /* Smaller buttons in header actions (doesn't affect TextButton) */
  .underlay-page-header__actions :global(.underlay-button) {
    --underlay-button-font-size: calc(1em * 0.8);
    --underlay-button-padding-block: 0.4em;
    --underlay-button-padding-inline: 0.85em;
  }

  .underlay-page-header__meta {
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
    font-size: 0.85em;
  }

  @media (max-width: 500px) {
    .underlay-page-header__title {
      font-size: 1.5em;
    }

    .underlay-page-header__subtitle {
      font-size: 1em;
    }
  }

  /* Level variants */
  @media (min-width: 768px) {
    .underlay-page-header--level-1 .underlay-page-header__title {
      font-size: 2em;
    }
  }

  /* Smaller level titles get lighter weight and italic */
  .underlay-page-header--level-2 .underlay-page-header__title,
  .underlay-page-header--level-3 .underlay-page-header__title,
  .underlay-page-header--level-4 .underlay-page-header__title {
    font-weight: 500;
    font-style: italic;
  }

  .underlay-page-header--level-2 .underlay-page-header__title {
    font-size: 1.5em;
  }

  .underlay-page-header--level-2 .underlay-page-header__subtitle {
    font-size: 1em;
  }

  .underlay-page-header--level-3 .underlay-page-header__title {
    font-size: 1.25em;
  }

  .underlay-page-header--level-3 .underlay-page-header__subtitle {
    font-size: 0.95em;
  }

  .underlay-page-header--level-4 .underlay-page-header__title {
    font-size: 1.1em;
  }

  .underlay-page-header--level-4 .underlay-page-header__subtitle {
    font-size: 0.9em;
  }

  /* Reduce margins for nested headers */
  .underlay-page-header--level-2,
  .underlay-page-header--level-3,
  .underlay-page-header--level-4 {
    margin-bottom: var(--underlay-space-4, 1rem);
  }

  /* Tighter title/subtitle gap for smaller levels */
  .underlay-page-header--level-3 .underlay-page-header__row,
  .underlay-page-header--level-4 .underlay-page-header__row {
    gap: 0.1rem;
  }
</style>
