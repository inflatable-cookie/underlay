<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Button from "../Button.svelte";

  type NavigateEvent = { url: string };
  type ErrorEvent = { message: string };

  const dispatch = createEventDispatcher<{ navigate: NavigateEvent; error: ErrorEvent }>();

  export let authorizationUrl: string | null = null;
  export let getAuthorizationUrl:
    | (() => Promise<string> | string)
    | null = null;

  export let label: string = "Continue with Google";
  export let variant: "primary" | "secondary" | "subtle" = "subtle";
  export let disabled: boolean = false;
  export let className: string = "";

  let loading = false;

  async function handleClick(e: CustomEvent<MouseEvent>) {
    if (disabled || loading) {
      return;
    }

    const event = e.detail;
    event.preventDefault();

    try {
      loading = true;

      const url =
        (getAuthorizationUrl ? await getAuthorizationUrl() : authorizationUrl) ??
        null;

      if (!url) {
        dispatch("error", { message: "missing authorization url" });
        return;
      }

      dispatch("navigate", { url });

      // Avoid direct browser globals for SSR/guardrails.
      if (globalThis?.location?.assign) {
        globalThis.location.assign(url);
      } else {
        dispatch("error", { message: "navigation not available" });
      }
    } finally {
      loading = false;
    }
  }
</script>

<Button
  type="button"
  {variant}
  className={`underlay-google-signin ${className}`}
  disabled={disabled || loading}
  aria-busy={loading}
  on:click={handleClick}
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
