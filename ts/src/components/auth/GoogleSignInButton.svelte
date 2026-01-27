<script lang="ts">
  import type { Snippet } from "svelte";

  import Button from "../Button.svelte";

  type NavigateEvent = { url: string };
  type ErrorEvent = { message: string };

  interface Props {
    authorizationUrl?: string | null;
    getAuthorizationUrl?: (() => Promise<string> | string) | null;
    /** Custom click handler (alternative to URL-based navigation) */
    onclick?: () => void | Promise<void>;
    label?: string;
    variant?: "primary" | "secondary" | "subtle";
    disabled?: boolean;
    class?: string;
    onNavigate?: (event: NavigateEvent) => void;
    onError?: (event: ErrorEvent) => void;
  }

  let {
    authorizationUrl = null,
    getAuthorizationUrl = null,
    onclick,
    label = "Continue with Google",
    variant = "subtle",
    disabled = false,
    class: className = "",
    onNavigate,
    onError,
  }: Props = $props();

  let loading = $state(false);

  async function handleClick(e: MouseEvent) {
    if (disabled || loading) {
      return;
    }

    e.preventDefault();

    try {
      loading = true;

      // If custom onclick handler provided, use it instead of URL navigation
      if (onclick) {
        await onclick();
        return;
      }

      const url =
        (getAuthorizationUrl ? await getAuthorizationUrl() : authorizationUrl) ??
        null;

      if (!url) {
        onError?.({ message: "missing authorization url" });
        return;
      }

      onNavigate?.({ url });

      // Avoid direct browser globals for SSR/guardrails.
      if (globalThis?.location?.assign) {
        globalThis.location.assign(url);
      } else {
        onError?.({ message: "navigation not available" });
      }
    } finally {
      loading = false;
    }
  }
</script>

<Button
  type="button"
  {variant}
  class={`underlay-google-signin ${className}`}
  disabled={disabled || loading}
  aria-busy={loading}
  onclick={handleClick}
>
  <span class="underlay-google-signin__logo" aria-hidden="true">G</span>
  <span class="underlay-google-signin__label">{label}</span>
</Button>

<style>
  :global(.underlay-google-signin) {
    gap: 0.65em;
  }

  :global(.underlay-google-signin__logo) {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.6em;
    height: 1.6em;
    border-radius: 999px;
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
    color: var(--underlay-color-text, inherit);
    font-weight: 700;
    font-size: 0.95em;
    line-height: 1;
  }

  :global(.underlay-google-signin__label) {
    white-space: nowrap;
  }
</style>
