<script lang="ts">
  import type { Snippet } from "svelte";
  import {
    PageHeader,
    Callout,
    PageLoading
  } from "@poodle/svelte";

  // --- Types ---

  interface Props {
    /** Page title */
    title: string;
    /** Section label (e.g., "Edit Project") */
    section?: string;
    /** Back link URL */
    backHref?: string;
    /** Back link label */
    backLabel?: string;
    /** Banner message (e.g., for warnings) */
    bannerMessage?: string;
    /** Banner tone */
    bannerTone?: "warning" | "info" | "danger";
    /** Whether data is loading */
    loading?: boolean;
    /** Loading message */
    loadingMessage?: string;
    /** Form-level error message */
    error?: string | null;
    /** Success message */
    success?: boolean;
    /** Success message text */
    successMessage?: string;
    /** Additional actions in the header */
    headerActions?: Snippet;
    /** The form content */
    children: Snippet;
  }

  // --- Props ---

  let {
    title,
    section,
    backHref,
    backLabel,
    bannerMessage,
    bannerTone = "warning",
    loading = false,
    loadingMessage = "Loading...",
    error,
    success = false,
    successMessage = "Saved successfully.",
    headerActions,
    children
  }: Props = $props();
</script>

<div class="entity-form-page">
  <PageHeader
    {title}
    {section}
    backHref={backHref ?? null}
    backLabel={backLabel}
    bannerMessage={bannerMessage}
    bannerTone={bannerTone}
  >
    {#snippet actions()}
      {#if headerActions}
        {@render headerActions()}
      {/if}
    {/snippet}
  </PageHeader>

  {#if loading}
    <PageLoading presentation="inline" message={loadingMessage} />
  {:else}
    {#if error}
      <Callout tone="danger" message={error} announceMode="polite" />
    {:else if success}
      <Callout tone="success" message={successMessage} announceMode="polite" />
    {/if}

    {@render children()}
  {/if}
</div>

<style>
  .entity-form-page {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-4, 1rem);
  }
</style>
