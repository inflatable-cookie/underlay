<script lang="ts">
  import type { Snippet } from "svelte";
  import { Button } from "@decodelabs/underlay/components";

  interface Props {
    /** Whether the button is disabled (typically bound to form.isSubmitting) */
    disabled?: boolean;
    /** Whether the form is submitting (shows loading state) */
    submitting?: boolean;
    /** Text to show while submitting */
    submittingText?: string;
    /** Button variant */
    variant?: "primary" | "secondary" | "subtle";
    /** Additional CSS class */
    class?: string;
    /** Default slot content */
    children?: Snippet;
  }

  let {
    disabled = false,
    submitting = false,
    submittingText = "Saving...",
    variant = "primary",
    class: className = "",
    children
  }: Props = $props();

  const isDisabled = $derived(disabled || submitting);
</script>

<Button
  type="submit"
  {variant}
  disabled={isDisabled}
  aria-busy={submitting}
  class={className}
>
  {#if submitting}
    <span class="underlay-submit-button__spinner" aria-hidden="true"></span>
    {submittingText}
  {:else if children}
    {@render children()}
  {:else}
    Submit
  {/if}
</Button>

<style>
  .underlay-submit-button__spinner {
    display: inline-block;
    width: 1em;
    height: 1em;
    border: 2px solid currentColor;
    border-right-color: transparent;
    border-radius: 50%;
    animation: underlay-spin 0.6s linear infinite;
  }

  @keyframes underlay-spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
