<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLFormAttributes } from "svelte/elements";
  import { untrack } from "svelte";
  import { getBackButtonInfo } from "../patterns/navigation";
  import { resolveRedirectTo } from "../client/route-protection";
  import {
    Card,
    Callout,
    PageHeader,
    PageLoading
  } from "@poodle/svelte";
  import type {
    SpaFormResult,
    SpaSubmitHandler,
    SpaNavigateFn
  } from "../patterns/spa-form-types";

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  type SnippetLike = Snippet | ((...args: any[]) => any);

  interface Props {
    title?: string;
    section?: string;
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
    fieldErrors?: Record<string, string> | null;
    success?: boolean | null;
    successMessage?: string;
    prepare?: ((formData: FormData) => void) | null;
    method?: "post" | "get";
    formClass?: string;
    autocomplete?: HTMLFormAttributes["autocomplete"];
    headerMeta?: SnippetLike;
    headerActions?: SnippetLike;
    secondaryContent?: SnippetLike;
    contentLayout?: "single" | "split";
    onSubmit?: SpaSubmitHandler;
    onResult?: (result: SpaFormResult) => void;
    navigate?: SpaNavigateFn;
    children: SnippetLike;
  }

  let {
    title,
    section,
    subtitle,
    backHref = null,
    backLabel = "Back",
    backIsContextual = false,
    resolveBackContext = true,
    bannerMessage,
    bannerTone = "warning",
    loading = false,
    loadingMessage = "Loading...",
    error: initialError = null,
    fieldErrors: initialFieldErrors = null,
    success: initialSuccess = null,
    successMessage = "Saved successfully.",
    prepare = null,
    method = "post",
    formClass = "underlay-form-grid",
    autocomplete = "off",
    headerMeta,
    headerActions,
    secondaryContent,
    contentLayout = "single",
    onSubmit,
    onResult,
    navigate,
    children
  }: Props = $props();

  let submitting = $state(false);
  let success = $state<boolean | null>(untrack(() => initialSuccess));
  let error = $state<string | null>(untrack(() => initialError));
  let fieldErrors = $state<Record<string, string> | null>(untrack(() => initialFieldErrors));

  // Track only the props and assign unconditionally (mirrors SpaFormShell).
  // Reading the local state in a condition would make this effect re-run on
  // submit-handler writes and clobber the feedback back to the initial props.
  $effect(() => {
    success = initialSuccess;
    error = initialError;
    fieldErrors = initialFieldErrors;
  });

  const hasFieldErrors = $derived(Boolean(fieldErrors && Object.keys(fieldErrors).length > 0));
  const visibleError = $derived(hasFieldErrors ? null : error);
  let resolvedBackInfo = $state<{ href: string; label: string; contextual: boolean } | null>(null);

  $effect(() => {
    let nextResolvedBackInfo: { href: string; label: string; contextual: boolean } | null;

    if (!backHref) {
      nextResolvedBackInfo = null;
    } else if (!resolveBackContext) {
      nextResolvedBackInfo = {
        href: backHref,
        label: backLabel,
        contextual: backIsContextual
      };
    } else {
      const contextualBackInfo = getBackButtonInfo(backLabel, backHref);
      nextResolvedBackInfo = {
        href: contextualBackInfo.href,
        label: contextualBackInfo.label,
        contextual: Boolean(contextualBackInfo.isContextual || backIsContextual)
      };
    }

    const current = resolvedBackInfo;
    if (
      current?.href !== nextResolvedBackInfo?.href ||
      current?.label !== nextResolvedBackInfo?.label ||
      current?.contextual !== nextResolvedBackInfo?.contextual
    ) {
      resolvedBackInfo = nextResolvedBackInfo;
    }
  });

  const defaultNavigate: SpaNavigateFn = (url: string) => {
    window.location.href = url;
  };

  function handleFormData(event: FormDataEvent) {
    if (onSubmit) return;
    prepare?.(event.formData);
  }

  const spaEnhance = (node: HTMLFormElement) => {
    if (!onSubmit) {
      return;
    }
    const submit = onSubmit;

    async function handleSubmit(event: SubmitEvent) {
      event.preventDefault();

      if (submitting) {
        return;
      }

      submitting = true;
      error = null;
      fieldErrors = null;
      success = null;

      try {
        const formData = new FormData(node);
        prepare?.(formData);

        const result = await submit(formData);

        success = result.success;
        error = result.error ?? null;
        fieldErrors = result.fieldErrors ?? null;
        onResult?.(result);

        if (result.success && result.redirectTo) {
          await (navigate ?? defaultNavigate)(resolveRedirectTo(result.redirectTo));
        }
      } catch (e) {
        error = e instanceof Error ? e.message : "An unexpected error occurred";
        success = false;
      } finally {
        submitting = false;
      }
    }

    node.addEventListener("submit", handleSubmit);

    return {
      destroy() {
        node.removeEventListener("submit", handleSubmit);
      }
    };
  };
</script>

<div class="underlay-entity-form-page">
  <div class="underlay-entity-form-page__header">
    <PageHeader
      {title}
      {section}
      {subtitle}
      backHref={resolvedBackInfo?.href ?? backHref}
      backLabel={resolvedBackInfo?.label ?? backLabel}
      backIsContextual={resolvedBackInfo?.contextual ?? backIsContextual}
      {bannerMessage}
      {bannerTone}
    >
      {#snippet actions()}
        {#if headerActions}
          {@render headerActions()}
        {/if}
      {/snippet}
    </PageHeader>

    {#if headerMeta}
      <div class="underlay-entity-form-page__meta">
        {@render headerMeta()}
      </div>
    {/if}
  </div>

  {#if loading}
    <PageLoading presentation="inline" message={loadingMessage} />
  {:else}
    {#if submitting}
      <Callout
        tone="pending"
        title="Saving"
        message="Your changes are being submitted."
        announceMode="polite"
      />
    {/if}

    {#if success && successMessage}
      <Callout
        tone="success"
        title="Saved"
        message={successMessage}
        announceMode="polite"
      />
    {/if}

    {#if visibleError}
      <Callout
        tone="danger"
        title="Could not save"
        message={visibleError}
        announceMode="assertive"
      />
    {/if}

    {#if hasFieldErrors}
      <Callout
        tone="danger"
        title="There are problems with some fields"
        announceMode="polite"
      >
        <ul class="underlay-entity-form-page__error-list">
          {#each Object.entries(fieldErrors ?? {}) as [field, message]}
            <li><strong>{field}</strong>: {message}</li>
          {/each}
        </ul>
      </Callout>
    {/if}

    <div class={`underlay-entity-form-page__content underlay-entity-form-page__content--${contentLayout}`}>
      <Card>
        <form
          {method}
          onformdata={handleFormData}
          use:spaEnhance
          class={formClass}
          {autocomplete}
        >
          {@render children()}
        </form>
      </Card>

      {#if secondaryContent}
        <div class="underlay-entity-form-page__secondary">
          {@render secondaryContent()}
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .underlay-entity-form-page {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-4, 1rem);
  }

  .underlay-entity-form-page__header {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .underlay-entity-form-page__meta {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
  }

  .underlay-entity-form-page__content {
    display: grid;
    gap: var(--underlay-space-4, 1rem);
  }

  .underlay-entity-form-page__content--split {
    grid-template-columns: repeat(auto-fit, minmax(22rem, 1fr));
    align-items: start;
  }

  .underlay-entity-form-page__secondary {
    display: contents;
  }

  .underlay-entity-form-page__error-list {
    margin: 0;
    padding-left: 1.25rem;
  }
</style>
