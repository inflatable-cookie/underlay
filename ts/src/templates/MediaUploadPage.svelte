<script lang="ts">
  import type { Snippet } from "svelte";
  import { Callout, PageHeader, PageLoading } from "@poodle/svelte";
  import { getBackButtonInfo } from "../patterns/navigation";

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  type SnippetLike = Snippet | ((...args: any[]) => any);

  interface Props {
    title?: string;
    subtitle?: string;
    backHref?: string | null;
    backLabel?: string;
    backIsContextual?: boolean;
    resolveBackContext?: boolean;
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
    backIsContextual = false,
    resolveBackContext = true,
    bannerMessage,
    bannerTone = "warning",
    loading = false,
    loadingMessage = "Loading upload workflow...",
    error = null,
    errorTitle = "Could not upload media",
    intro,
    children
  }: Props = $props();

  let resolvedBackInfo = $state<{ href: string; label: string; contextual: boolean } | null>(null);

  $effect(() => {
    if (!backHref) {
      resolvedBackInfo = null;
      return;
    }

    if (!resolveBackContext) {
      resolvedBackInfo = {
        href: backHref,
        label: backLabel,
        contextual: backIsContextual
      };
      return;
    }

    const contextualBackInfo = getBackButtonInfo(backLabel, backHref);
    resolvedBackInfo = {
      href: contextualBackInfo.href,
      label: contextualBackInfo.label,
      contextual: Boolean(contextualBackInfo.isContextual || backIsContextual)
    };
  });
</script>

<div class="underlay-media-upload-page">
  <PageHeader
    {title}
    {subtitle}
    backHref={resolvedBackInfo?.href ?? backHref}
    backLabel={resolvedBackInfo?.label ?? backLabel}
    backIsContextual={resolvedBackInfo?.contextual ?? false}
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
