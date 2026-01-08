<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    title: string;
    backHref?: string | null;
    backLabel?: string;

    actions?: Snippet;
    children?: Snippet;
  }

  let {
    title,
    backHref = null,
    backLabel = "Back",
    actions,
    children
  }: Props = $props();
</script>

<header class="underlay-page-header">
  {#if backHref}
    <a class="underlay-page-header__back" href={backHref}>
      ← {backLabel}
    </a>
  {/if}

  <div class="underlay-page-header__row">
    <h1 class="underlay-page-header__title">{title}</h1>

    {#if actions}
      <div class="underlay-page-header__actions">{@render actions?.()}</div>
    {/if}
  </div>

  {#if children}
    <div class="underlay-page-header__meta">{@render children?.()}</div>
  {/if}
</header>

<style>
  .underlay-page-header {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-2, 0.5rem);
    margin-bottom: var(--underlay-space-4, 1rem);
  }

  .underlay-page-header__back {
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
    text-decoration: none;
    width: fit-content;
    font-size: var(--underlay-font-size-sm, 0.85rem);
  }

  .underlay-page-header__back:hover {
    color: var(--underlay-color-text, #e5e7eb);
    text-decoration: underline;
    text-underline-offset: 0.12em;
  }

  .underlay-page-header__row {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--underlay-space-3, 0.75rem);
    flex-wrap: wrap;
  }

  .underlay-page-header__title {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 650;
    color: var(--underlay-color-text, #e5e7eb);
  }

  .underlay-page-header__actions {
    display: inline-flex;
    align-items: center;
    gap: var(--underlay-space-2, 0.5rem);
  }

  .underlay-page-header__meta {
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
    font-size: var(--underlay-font-size-sm, 0.85rem);
  }

  .underlay-page-header__meta :global(p) {
    margin: 0.15rem 0;
  }

  .underlay-page-header__meta :global(code) {
    font-size: 0.9em;
  }
</style>
