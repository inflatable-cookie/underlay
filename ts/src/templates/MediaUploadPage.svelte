<script lang="ts">
  import type { Snippet } from "svelte";
  import { Callout, PageHeader, PageLoading } from "@poodle/svelte";

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  type SnippetLike = Snippet | ((...args: any[]) => any);

  interface Props {
    title?: string;
    subtitle?: string;
    backHref?: string | null;
    backLabel?: string;
    bannerMessage?: string;
    bannerTone?: "warning" | "info" | "danger";
    loading?: boolean;
    loadingMessage?: string;
    error?: string | null;
    errorTitle?: string;
    intro?: SnippetLike;
    children: SnippetLike;
  }

  let {
    title = "Upload Media",
    subtitle,
    backHref = "/media",
    backLabel = "Back to library",
    bannerMessage,
    bannerTone = "warning",
    loading = false,
    loadingMessage = "Loading upload workflow...",
    error = null,
    errorTitle = "Could not upload media",
    intro,
    children
  }: Props = $props();
</script>

<div class="underlay-media-upload-page">
  <PageHeader
    {title}
    {subtitle}
    {backHref}
    {backLabel}
    {bannerMessage}
    {bannerTone}
  />

  {#if intro}
    <div class="underlay-media-upload-page__intro">
      {@render intro()}
    </div>
  {/if}

  {#if loading}
    <PageLoading presentation="inline" message={loadingMessage} />
  {:else}
    {#if error}
      <Callout tone="danger" title={errorTitle} message={error} announceMode="polite" />
    {/if}

    {@render children()}
  {/if}
</div>

<style>
  .underlay-media-upload-page {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-4, 1rem);
  }

  .underlay-media-upload-page__intro {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
</style>
