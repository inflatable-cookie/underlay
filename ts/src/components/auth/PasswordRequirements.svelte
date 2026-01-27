<script lang="ts">
  /**
   * Password requirements display with real-time validation feedback.
   *
   * Fetches requirements from the server and shows validation state
   * as the user types their password.
   */

  interface PasswordRequirementsData {
    minLength: number;
    requireMixedCase: boolean;
    requireDigit: boolean;
    requireSpecial: boolean;
    minStrengthScore: number;
    description: string;
  }

  interface Props {
    /** The password value to validate against */
    password: string;
    /** Function to fetch requirements from server */
    fetchRequirements: () => Promise<PasswordRequirementsData>;
    /** Optional class name */
    class?: string;
  }

  let {
    password,
    fetchRequirements,
    class: className = ""
  }: Props = $props();

  // Requirements state
  let requirements = $state<PasswordRequirementsData | null>(null);
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

  // Validation checks
  let lengthMet = $derived(requirements ? password.length >= requirements.minLength : false);
  let mixedCaseMet = $derived(
    !requirements?.requireMixedCase || (/[a-z]/.test(password) && /[A-Z]/.test(password))
  );
  let digitMet = $derived(
    !requirements?.requireDigit || /\d/.test(password)
  );
  let specialMet = $derived(
    !requirements?.requireSpecial || /[^a-zA-Z0-9]/.test(password)
  );
</script>

{#if loading}
  <div class="underlay-password-requirements {className}">
    <p class="underlay-password-requirements__loading">Loading requirements...</p>
  </div>
{:else if requirements}
  <div class="underlay-password-requirements {className}">
    <p class="underlay-password-requirements__title">Password requirements:</p>
    <ul class="underlay-password-requirements__list">
      <li class:met={lengthMet}>At least {requirements.minLength} characters</li>
      {#if requirements.requireMixedCase}
        <li class:met={mixedCaseMet}>Mix of uppercase and lowercase letters</li>
      {/if}
      {#if requirements.requireDigit}
        <li class:met={digitMet}>At least one number</li>
      {/if}
      {#if requirements.requireSpecial}
        <li class:met={specialMet}>At least one special character</li>
      {/if}
    </ul>
    <p class="underlay-password-requirements__hint">
      Avoid common words, patterns, and personal information.
    </p>
  </div>
{/if}

<style>
  .underlay-password-requirements {
    margin-bottom: 1rem;
    padding: 1rem;
    border-radius: 0.5rem;
    background: var(--underlay-color-surface-subtle, rgba(15, 23, 42, 0.4));
  }

  .underlay-password-requirements__title {
    margin: 0 0 0.5rem;
    font-weight: 500;
    color: var(--underlay-color-text, #e5e7eb);
  }

  .underlay-password-requirements__loading {
    margin: 0;
    color: var(--underlay-color-text-muted, #64748b);
    font-size: 0.9rem;
  }

  .underlay-password-requirements__list {
    margin: 0;
    padding: 0 0 0 1.25rem;
    list-style: disc;
  }

  .underlay-password-requirements__list li {
    color: var(--underlay-color-text-muted, #64748b);
    font-size: 0.9rem;
    line-height: 1.6;
    transition: color 0.15s;
  }

  .underlay-password-requirements__list li.met {
    color: var(--underlay-color-success, #22c55e);
  }

  .underlay-password-requirements__hint {
    margin: 0.75rem 0 0;
    font-size: 0.85rem;
    color: var(--underlay-color-text-muted, #64748b);
    font-style: italic;
  }
</style>
