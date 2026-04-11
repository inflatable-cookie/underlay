<script lang="ts">
  /**
   * Generic success confirmation step with action button.
   *
   * Displays a success icon, title, message, and an action button
   * that can either navigate to a URL or trigger a callback.
   *
   * @example
   * ```svelte
   * <SuccessStep
   *   title="Password reset successfully"
   *   message="You can now log in with your new password."
   *   actionLabel="Go to login"
   *   actionHref="/login"
   * />
   * ```
   */

  import type { Snippet } from "svelte";
  import { Button, StatusIndicator } from "@poodle/svelte";

  interface Props {
    /** Title text */
    title: string;
    /** Message text */
    message?: string;
    /** Custom icon snippet (default: checkmark) */
    icon?: Snippet;
    /** Label for the action button */
    actionLabel?: string;
    /** URL to navigate to when action button is clicked */
    actionHref?: string;
    /** Callback when action button is clicked (alternative to actionHref) */
    onAction?: () => void;
    /** Additional CSS class */
    class?: string;
  }

  let {
    title,
    message,
    icon,
    actionLabel = "Continue",
    actionHref,
    onAction,
    class: className = "",
  }: Props = $props();

  function handleAction() {
    if (actionHref) {
      window.location.href = actionHref;
    } else {
      onAction?.();
    }
  }
</script>

<div class="underlay-success-step {className}">
  <div class="underlay-success-step__icon">
    {#if icon}
      {@render icon()}
    {:else}
      <StatusIndicator status="success" label="Success" ariaLabel="Success" />
    {/if}
  </div>

  <h2 class="underlay-success-step__title">{title}</h2>

  {#if message}
    <p class="underlay-success-step__message">{message}</p>
  {/if}

  {#if actionLabel && (actionHref || onAction)}
    <div class="underlay-success-step__action">
      <Button variant="primary" on:click={handleAction}>
        {actionLabel}
      </Button>
    </div>
  {/if}
</div>

<style>
  .underlay-success-step {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: var(--underlay-space-4, 1rem) 0;
    gap: var(--underlay-space-3, 0.75rem);
  }

  .underlay-success-step__icon {
    display: flex;
    justify-content: center;
  }

  .underlay-success-step__title {
    margin: 0;
    font-size: var(--underlay-font-size-lg, 1.1rem);
    font-weight: 600;
    color: var(--underlay-color-text, #e5e7eb);
  }

  .underlay-success-step__message {
    margin: 0;
    font-size: var(--underlay-font-size-sm, 0.875rem);
    line-height: 1.5;
    color: var(--underlay-color-text-muted, #64748b);
    max-width: 20rem;
  }

  .underlay-success-step__action {
    margin-top: var(--underlay-space-2, 0.5rem);
  }
</style>
