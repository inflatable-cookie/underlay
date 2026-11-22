<script lang="ts">
  import { untrack } from "svelte";
  import { PasswordRequirements } from "@inflatable-cookie/poodle-svelte";
  import type { PasswordRequirementsPolicy } from "@inflatable-cookie/poodle-svelte";

  /**
   * Password requirements display with real-time validation feedback.
   *
   * Fetches requirements from the server and shows validation state
   * as the user types their password.
   */

  interface Props {
    /** The password value to validate against */
    password: string;
    /** Function to fetch requirements from server */
    fetchRequirements: () => Promise<PasswordRequirementsPolicy>;
    /** Optional explicit reload key; changing it refetches requirements */
    reloadKey?: string | number | null;
    /** Optional class name */
    class?: string;
  }

  let {
    password,
    fetchRequirements,
    reloadKey = null,
    class: className = ""
  }: Props = $props();

  // Requirements state
  let requirements = $state<PasswordRequirementsPolicy | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);

  // Fetch requirements once on mount (or when reloadKey explicitly changes).
  // Untrack fetchRequirements so inline arrow props recreated on parent keystrokes
  // do not trigger refetch storms.
  $effect(() => {
    void reloadKey;
    let cancelled = false;
    const loader = untrack(() => fetchRequirements);

    void (async () => {
      try {
        loading = true;
        error = null;
        const policy = await loader();
        if (cancelled) return;
        requirements = policy;
      } catch (e) {
        if (cancelled) return;
        error = e instanceof Error ? e.message : "Failed to load requirements";
        // Fallback to sensible defaults
        requirements = {
          minLength: 12,
          requireMixedCase: true,
          requireDigit: true,
          requireSpecial: true,
          minStrengthScore: 3,
          description: "Password must be at least 12 characters with a mix of letters, numbers, and symbols."
        };
      } finally {
        if (!cancelled) {
          loading = false;
        }
      }
    })();

    return () => {
      cancelled = true;
    };
  });

</script>

{#if loading}
  <div class="underlay-password-requirements {className}">
    <p class="underlay-password-requirements__loading">Loading requirements...</p>
  </div>
{:else if requirements}
  <div class="underlay-password-requirements {className}">
    {#if error}
      <p class="underlay-password-requirements__error" role="alert">{error}</p>
    {/if}
    <PasswordRequirements
      {password}
      {requirements}
      hint="Avoid common words, patterns, and personal information."
    />
  </div>
{/if}

<style>
  .underlay-password-requirements {
    margin-bottom: 1rem;
  }

  .underlay-password-requirements__loading {
    margin: 0;
    color: var(--underlay-color-text-muted, #64748b);
    font-size: 0.9rem;
  }

  .underlay-password-requirements__error {
    margin: 0 0 var(--underlay-space-2, 0.5rem);
    color: var(--underlay-color-error, #dc2626);
    font-size: var(--underlay-font-size-sm, 0.85rem);
  }

</style>
