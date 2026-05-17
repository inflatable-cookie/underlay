<script lang="ts">
  import { PasswordRequirements } from "@poodle/svelte";
  import type { PasswordRequirementsPolicy } from "@poodle/svelte";

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
    /** Optional class name */
    class?: string;
  }

  let {
    password,
    fetchRequirements,
    class: className = ""
  }: Props = $props();

  // Requirements state
  let requirements = $state<PasswordRequirementsPolicy | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);

  // Fetch requirements on mount
  $effect(() => {
    loadRequirements();
  });

  async function loadRequirements() {
    try {
      loading = true;
      error = null;
      requirements = await fetchRequirements();
    } catch (e) {
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
      loading = false;
    }
  }

</script>

{#if loading}
  <div class="underlay-password-requirements {className}">
    <p class="underlay-password-requirements__loading">Loading requirements...</p>
  </div>
{:else if requirements}
  <div class="underlay-password-requirements {className}">
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

</style>
