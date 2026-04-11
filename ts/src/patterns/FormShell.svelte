<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLFormAttributes } from "svelte/elements";
  import { Card, Callout } from "@poodle/svelte";
  import { PageHeader as PoodlePageHeader } from "@poodle/svelte";
  import type { BannerVariant } from "./banner";

  type PrepareHook = ((formData: FormData) => void) | null;

  // Optional progressive-enhancement hook (e.g. SvelteKit's `$app/forms` enhance).
  // This component stays framework-agnostic by accepting this as a prop.
  type EnhanceHook =
    | ((
        node: HTMLFormElement,
        submit?: (options: { formData: FormData }) => void
      ) => { destroy?: () => void } | void)
    | null;

  // Use permissive type for snippets to handle linked dependency type mismatches
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  type SnippetLike = Snippet | ((...args: any[]) => any);

  interface Props {
    title?: string;
    section?: string;
    subtitle?: string;
    backHref?: string | null;
    backLabel?: string;
    backIsContextual?: boolean;
    method?: "post" | "get";

    showTitle?: boolean;

    /** Banner message to display below the header */
    bannerMessage?: string;
    /** Banner variant (warning, error, info) */
    bannerVariant?: BannerVariant;

    success?: boolean | null;
    successMessage?: string | null;
    error?: string | null;
    fieldErrors?: Record<string, string> | null;

    prepare?: PrepareHook;
    enhance?: EnhanceHook;

    formClass?: string;
    autocomplete?: HTMLFormAttributes["autocomplete"];

    /** Content to render inside PageHeader (meta info like IDs) */
    headerMeta?: SnippetLike;
    children?: SnippetLike;
  }

  let {
    title,
    section,
    subtitle,
    backHref = null,
    backLabel = "Back",
    backIsContextual = false,
    method = "post",
    showTitle = true,
    bannerMessage,
    bannerVariant = "warning",
    success = null,
    successMessage = null,
    error = null,
    fieldErrors = null,
    prepare = null,
    enhance = null,
    formClass = "underlay-form-grid",
    autocomplete = "off",
    headerMeta,
    children
  }: Props = $props();

  const hasFieldErrors = $derived(Boolean(fieldErrors && Object.keys(fieldErrors).length > 0));
  const visibleError = $derived(hasFieldErrors ? null : error);

  function handleFormData(event: FormDataEvent) {
    if (enhance) return;
    prepare?.(event.formData);
  }

  const useEnhanced = (
    node: HTMLFormElement,
    params: { enhance: EnhanceHook; prepare: PrepareHook }
  ) => {
    let teardown: (() => void) | null = null;

    function apply(next: { enhance: EnhanceHook; prepare: PrepareHook }) {
      teardown?.();
      teardown = null;

      if (!next.enhance) {
        return;
      }

      const enhanced = next.enhance(node, ({ formData }) => {
        next.prepare?.(formData);
      });

      if (enhanced && typeof (enhanced as { destroy?: () => void }).destroy === "function") {
        teardown = () => (enhanced as { destroy: () => void }).destroy();
      }
    }

    apply(params);

    return {
      update(next: { enhance: EnhanceHook; prepare: PrepareHook }) {
        apply(next);
      },
      destroy() {
        teardown?.();
        teardown = null;
      }
    };
  };
</script>

{#if showTitle}
  <div class="underlay-form-shell__header">
    <PoodlePageHeader
      {title}
      {section}
      {subtitle}
      {backHref}
      {backLabel}
      backIsContextual={backIsContextual}
      {bannerMessage}
      bannerTone={bannerVariant === "error" ? "danger" : bannerVariant}
    />

    {#if headerMeta}
      <div class="underlay-form-shell__meta">
        {@render headerMeta()}
      </div>
    {/if}
  </div>
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
    <ul class="underlay-form-shell__error-list">
      {#each Object.entries(fieldErrors ?? {}) as [field, message]}
        <li><strong>{field}</strong>: {message}</li>
      {/each}
    </ul>
  </Callout>
{/if}

<Card>
  <form
    {method}
    onformdata={handleFormData}
    use:useEnhanced={{ enhance, prepare }}
    class={formClass}
    {autocomplete}
  >
    {@render children?.()}
  </form>
</Card>

<style>
  .underlay-form-shell__header {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .underlay-form-shell__meta {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
  }

  .underlay-form-shell__error-list {
    margin: 0;
    padding-left: 1.25rem;
  }
</style>
