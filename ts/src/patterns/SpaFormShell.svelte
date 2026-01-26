<script lang="ts">
  import type { Snippet } from "svelte";
  import type { BannerVariant } from "./banner";
  import type { SpaFormResult, SpaSubmitHandler, SpaNavigateFn } from "./spa-form-types";
  import FormShell from "./FormShell.svelte";

  interface Props {
    title: string;
    subtitle?: string;
    backHref?: string | null;
    backLabel?: string;
    backIsContextual?: boolean;
    bannerMessage?: string;
    bannerVariant?: BannerVariant;
    /** Initial success state (from previous submission) */
    success?: boolean | null;
    successMessage?: string | null;
    /** Initial error state */
    error?: string | null;
    /** Initial field errors */
    fieldErrors?: Record<string, string> | null;
    /** Prepare hook called before submission to modify form data */
    prepare?: ((formData: FormData) => void) | null;
    formClass?: string;
    showTitle?: boolean;
    headerMeta?: Snippet;
    children?: Snippet;
    /** Handler for form submission - required for SPA mode */
    onSubmit: SpaSubmitHandler;
    /** Optional callback when submission completes */
    onResult?: (result: SpaFormResult) => void;
    /**
     * Navigation function for redirects after successful submission.
     * Pass SvelteKit's `goto` for SvelteKit apps, or leave undefined for
     * default browser navigation.
     */
    navigate?: SpaNavigateFn;
  }

  let {
    title,
    subtitle,
    backHref = null,
    backLabel = "Back",
    backIsContextual = false,
    bannerMessage,
    bannerVariant = "warning",
    success: initialSuccess = null,
    successMessage = null,
    error: initialError = null,
    fieldErrors: initialFieldErrors = null,
    prepare = null,
    formClass = "underlay-form-grid",
    showTitle = true,
    headerMeta,
    children,
    onSubmit,
    onResult,
    navigate
  }: Props = $props();

  // Internal state for form status
  let loading = $state(false);
  let success = $state(initialSuccess);
  let error = $state(initialError);
  let fieldErrors = $state(initialFieldErrors);

  // Reset state when initial props change
  $effect(() => {
    success = initialSuccess;
    error = initialError;
    fieldErrors = initialFieldErrors;
  });

  // Default navigation function uses window.location
  const defaultNavigate: SpaNavigateFn = (url: string) => {
    window.location.href = url;
  };

  /**
   * SPA-mode enhance function that intercepts form submission
   * and calls the onSubmit handler instead of posting to the server.
   */
  const spaEnhance = (node: HTMLFormElement) => {
    const handleSubmit = async (event: SubmitEvent) => {
      event.preventDefault();

      // Reset state
      loading = true;
      error = null;
      fieldErrors = null;
      success = null;

      try {
        // Create form data
        const formData = new FormData(node);

        // Call prepare hook if provided
        if (prepare) {
          prepare(formData);
        }

        // Call the submit handler
        const result = await onSubmit(formData);

        // Update state based on result
        success = result.success;
        error = result.error ?? null;
        fieldErrors = result.fieldErrors ?? null;

        // Notify callback if provided
        if (onResult) {
          onResult(result);
        }

        // Handle redirect if specified
        if (result.success && result.redirectTo) {
          const nav = navigate ?? defaultNavigate;
          await nav(result.redirectTo);
        }
      } catch (e) {
        // Handle unexpected errors
        error = e instanceof Error ? e.message : "An unexpected error occurred";
        success = false;
      } finally {
        loading = false;
      }
    };

    node.addEventListener("submit", handleSubmit);

    return {
      destroy() {
        node.removeEventListener("submit", handleSubmit);
      }
    };
  };
</script>

<FormShell
  {title}
  {subtitle}
  {backHref}
  {backLabel}
  {backIsContextual}
  {bannerMessage}
  {bannerVariant}
  method="post"
  {showTitle}
  {success}
  {successMessage}
  {error}
  {fieldErrors}
  {prepare}
  {formClass}
  {headerMeta}
  enhance={spaEnhance}
  autocomplete="off"
>
  {#if loading}
    <div class="underlay-spa-form-shell__loading">
      <span class="underlay-spa-form-shell__spinner"></span>
      Saving...
    </div>
  {/if}
  {@render children?.()}
</FormShell>

<style>
  .underlay-spa-form-shell__loading {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    margin-bottom: 1rem;
    border-radius: 0.5rem;
    background: color-mix(in srgb, var(--underlay-color-primary, #3b82f6) 15%, transparent);
    border: 1px solid color-mix(in srgb, var(--underlay-color-primary, #3b82f6) 30%, transparent);
    color: var(--underlay-color-text, inherit);
    font-size: 0.9rem;
  }

  .underlay-spa-form-shell__spinner {
    width: 1rem;
    height: 1rem;
    border: 2px solid currentColor;
    border-top-color: transparent;
    border-radius: 50%;
    animation: underlay-spa-form-spin 0.75s linear infinite;
  }

  @keyframes underlay-spa-form-spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
