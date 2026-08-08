<script lang="ts">
  import type { Snippet } from "svelte";
  import { untrack } from "svelte";
  import { Callout } from "@inflatable-cookie/poodle-svelte";
  import type { BannerVariant } from "./banner";
  import type { SpaFormResult, SpaSubmitHandler, SpaNavigateFn } from "./spa-form-types";
  import { resolveRedirectTo } from "../client/route-protection";
  import { default as FormShell } from "./FormShell.svelte";

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
    resolveBackContext?: boolean;
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
    headerMeta?: SnippetLike;
    children?: SnippetLike;
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
    section,
    subtitle,
    backHref = null,
    backLabel = "Back",
    backIsContextual = false,
    resolveBackContext = true,
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

  // Internal state for form status - initialized from props, then managed locally
  // Use untrack to capture initial values without creating reactive dependencies
  let loading = $state(false);
  let success = $state(untrack(() => initialSuccess));
  let error = $state(untrack(() => initialError));
  let fieldErrors = $state(untrack(() => initialFieldErrors));

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
    async function handleSubmit(event: SubmitEvent) {
      event.preventDefault();

      if (loading) {
        return;
      }

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

        // Handle redirect if specified. Redirect targets can originate from
        // an untrusted redirectTo query param, so resolve to a safe
        // same-origin path before navigating.
        if (result.success && result.redirectTo) {
          const nav = navigate ?? defaultNavigate;
          await nav(resolveRedirectTo(result.redirectTo));
        }
      } catch (e) {
        // Handle unexpected errors
        error = e instanceof Error ? e.message : "An unexpected error occurred";
        success = false;
      } finally {
        loading = false;
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

<FormShell
  {title}
  {section}
  {subtitle}
  {backHref}
  {backLabel}
  {backIsContextual}
  {resolveBackContext}
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
    <Callout
      tone="pending"
      title="Saving"
      message="Your changes are being submitted."
      announceMode="polite"
    />
  {/if}
  {@render children?.()}
</FormShell>
