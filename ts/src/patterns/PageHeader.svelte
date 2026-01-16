<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    title: string;
    subtitle?: string;
    backHref?: string | null;
    backLabel?: string;
    /** True when backHref/backLabel came from navigation context */
    backIsContextual?: boolean;

    /** Primary actions (e.g. "Add") shown next to back link */
    primaryActions?: Snippet;
    actions?: Snippet;
    children?: Snippet;
  }

  let {
    title,
    subtitle,
    backHref = null,
    backLabel = "Back",
    backIsContextual = false,
    primaryActions,
    actions,
    children
  }: Props = $props();

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

  const formattedBackLabel = $derived(formatBackLabel(backLabel));
</script>

<header class="underlay-page-header">
  <div class="underlay-page-header__row">
    <div class="underlay-page-header__top">
      <h1 class="underlay-page-header__title">{title}</h1>

      <div class="underlay-page-header__right">
        {#if backHref}
          <a
            class="underlay-page-header__back underlay-page-header__back--inline"
            href={backHref}
          >
            ← {formattedBackLabel}
            {#if backIsContextual}
              <span class="underlay-page-header__context-dot" aria-hidden="true"></span>
            {/if}
          </a>
        {/if}
        {#if primaryActions}
          <div class="underlay-page-header__primary-actions">{@render primaryActions?.()}</div>
        {/if}
        {#if actions}
          <div class="underlay-page-header__actions">{@render actions?.()}</div>
        {/if}
      </div>
    </div>

    {#if subtitle}
      <h2 class="underlay-page-header__subtitle">{subtitle}</h2>
    {/if}
  </div>

  {#if children}
    <div class="underlay-page-header__meta">{@render children?.()}</div>
  {/if}

  {#if backHref}
    <a class="underlay-page-header__back underlay-page-header__back--below" href={backHref}>
      ← {formattedBackLabel}
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
    margin-bottom: var(--underlay-space-5, 1.25rem);
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
  }

  .underlay-page-header__subtitle {
    margin: 0;
    font-size: 1.1em;
    font-weight: 500;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
  }

  .underlay-page-header__right {
    display: inline-flex;
    align-items: center;
    gap: var(--underlay-space-4, 1rem);
    flex-wrap: wrap;
  }

  .underlay-page-header__primary-actions {
    display: inline-flex;
    align-items: center;
    gap: var(--underlay-space-2, 0.5rem);
    flex-wrap: wrap;
  }

  .underlay-page-header__primary-actions :global(.underlay-button) {
    --underlay-button-font-size: calc(1em * 0.88);
    --underlay-button-padding-block: 0.48em;
    --underlay-button-padding-inline: 0.95em;
  }

  .underlay-page-header__actions {
    display: inline-flex;
    align-items: center;
    gap: var(--underlay-space-2, 0.5rem);
  }

  .underlay-page-header__meta {
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
    font-size: 0.85em;
  }

  .underlay-page-header__meta :global(p) {
    margin: 0.15rem 0;
  }

  .underlay-page-header__meta :global(code) {
    font-size: 0.9em;
  }

  @media (max-width: 500px) {
    .underlay-page-header__title {
      font-size: 1.5em;
    }

    .underlay-page-header__subtitle {
      font-size: 1em;
    }
  }
</style>
