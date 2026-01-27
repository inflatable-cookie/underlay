<!--
  Full-page loading indicator for pages fetching auth-protected data.
  Use with useAuthenticatedData pattern.

  Usage:
    if pageData.loading -> <PageLoading message="Loading settings..." />
    else -> your content
-->
<script lang="ts">
  interface Props {
    /** Loading message to display */
    message?: string;
    /** Size of the spinner: "sm" | "md" | "lg" */
    size?: "sm" | "md" | "lg";
  }

  let { message = "Loading...", size = "md" }: Props = $props();

  const sizeMap = {
    sm: "1.5rem",
    md: "2rem",
    lg: "3rem"
  };

  const spinnerSize = $derived(sizeMap[size]);
</script>

<div class="underlay-page-loading" role="status" aria-live="polite">
  <div
    class="underlay-page-loading__spinner"
    style:width={spinnerSize}
    style:height={spinnerSize}
  ></div>
  {#if message}
    <p class="underlay-page-loading__text">{message}</p>
  {/if}
</div>

<style>
  .underlay-page-loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--underlay-space-12, 3rem) var(--underlay-space-4, 1rem);
    gap: var(--underlay-space-4, 1rem);
  }

  .underlay-page-loading__spinner {
    border: 2px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.2));
    border-top-color: var(--underlay-color-primary, #14b8a6);
    border-radius: 50%;
    animation: underlay-page-loading-spin 0.8s linear infinite;
  }

  @keyframes underlay-page-loading-spin {
    to {
      transform: rotate(360deg);
    }
  }

  .underlay-page-loading__text {
    color: var(--underlay-color-text-muted, #64748b);
    font-size: var(--underlay-text-sm, 0.875rem);
    margin: 0;
  }

  /* Reduce motion for accessibility */
  @media (prefers-reduced-motion: reduce) {
    .underlay-page-loading__spinner {
      animation-duration: 1.5s;
    }
  }
</style>
