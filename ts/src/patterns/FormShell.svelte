<script lang="ts">
  import type { Snippet } from "svelte";
  import { Card, Form } from "@decodelabs/underlay/components";
  import PageHeader from "./PageHeader.svelte";
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

  interface Props {
    title: string;
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
    autocomplete?: string;

    /** Content to render inside PageHeader (meta info like IDs) */
    headerMeta?: Snippet;
    children?: Snippet;
  }

  let {
    title,
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
</script>

{#if showTitle}
  <PageHeader {title} {subtitle} {backHref} {backLabel} {backIsContextual} {bannerMessage} {bannerVariant}>
    {#if headerMeta}
      {@render headerMeta()}
    {/if}
  </PageHeader>
{/if}

{#if success && successMessage}
  <p class="underlay-form-shell__success">{successMessage}</p>
{/if}

{#if error}
  <p class="underlay-form-shell__error">{error}</p>
{/if}

{#if fieldErrors && Object.keys(fieldErrors).length > 0}
  <div class="underlay-form-shell__error-summary" role="alert" aria-live="polite">
    <p>There are problems with some fields:</p>
    <ul>
      {#each Object.entries(fieldErrors) as [field, message]}
        <li><strong>{field}</strong>: {message}</li>
      {/each}
    </ul>
  </div>
{/if}

<Card as="section">
  <Form {method} class={formClass} {prepare} {enhance} {autocomplete}>
    {@render children?.()}
  </Form>
</Card>

<style>
  .underlay-form-shell__success {
    color: var(--underlay-color-success, #22c55e);
    margin-bottom: var(--underlay-space-4, 1rem);
  }

  .underlay-form-shell__error {
    color: var(--underlay-color-error, #ef4444);
    margin-bottom: var(--underlay-space-4, 1rem);
  }

  .underlay-form-shell__error-summary {
    margin-bottom: var(--underlay-space-4, 1rem);
    padding: var(--underlay-space-3, 0.75rem) var(--underlay-space-4, 1rem);
    border-radius: var(--underlay-radius-md, 0.5rem);
    background-color: rgba(239, 68, 68, 0.08);
    border: 1px solid rgba(239, 68, 68, 0.6);
    font-size: 0.9em;
  }

  .underlay-form-shell__error-summary ul {
    margin: var(--underlay-space-2, 0.5rem) 0 0;
    padding-left: 1.25rem;
  }
</style>
