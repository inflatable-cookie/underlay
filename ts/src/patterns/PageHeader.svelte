<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    title: string;
    subtitle?: string;
    backHref?: string | null;
    backLabel?: string;

    actions?: Snippet;
    children?: Snippet;
  }

  let {
    title,
    subtitle,
    backHref = null,
    backLabel = "Back",
    actions,
    children
  }: Props = $props();
</script>

<header class="underlay-page-header">
  <div class="underlay-page-header__row">
    <div class="underlay-page-header__titles">
      <h1 class="underlay-page-header__title">{title}</h1>
      {#if subtitle}
        <h2 class="underlay-page-header__subtitle">{subtitle}</h2>
      {/if}
    </div>

    <div class="underlay-page-header__right">
      {#if backHref}
        <a class="underlay-page-header__back underlay-page-header__back--inline" href={backHref}>
          ← {backLabel}
        </a>
      {/if}
      {#if actions}
        <div class="underlay-page-header__actions">{@render actions?.()}</div>
      {/if}
    </div>
  </div>

  {#if children}
    <div class="underlay-page-header__meta">{@render children?.()}</div>
  {/if}

  {#if backHref}
    <a class="underlay-page-header__back underlay-page-header__back--below" href={backHref}>
      ← {backLabel}
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
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--underlay-space-4, 1rem);
    flex-wrap: wrap;
  }

  .underlay-page-header__titles {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-1, 0.25rem);
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
