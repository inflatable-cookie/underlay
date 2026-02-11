<script lang="ts">
  import X from "lucide-svelte/icons/x";
  import Check from "lucide-svelte/icons/check";
  import AlertCircle from "lucide-svelte/icons/alert-circle";
  import type { InputValidationStatus } from "./validation-state";

  interface Props {
    showClearButton?: boolean;
    showValidationIcon?: boolean;
    showValidationStatus?: boolean;
    showValidationMessage?: boolean;
    validationStatus?: InputValidationStatus;
    validationMessage?: string;
    onClear?: () => void;
  }

  let {
    showClearButton = false,
    showValidationIcon = false,
    showValidationStatus = false,
    showValidationMessage = true,
    validationStatus = "idle",
    validationMessage = "",
    onClear
  }: Props = $props();
</script>

{#if showClearButton}
  <button
    type="button"
    class="underlay-input-clear"
    aria-label="Clear"
    onclick={onClear}
  >
    <X size="1em" strokeWidth={2.5} />
  </button>
{:else if showValidationIcon}
  <div class="underlay-input-validation" aria-live="polite">
    {#if validationStatus === "validating"}
      <span class="underlay-input-validation__spinner" aria-label="Validating"></span>
    {:else if validationStatus === "valid"}
      <Check
        size="1em"
        class="underlay-input-validation__icon underlay-input-validation__icon--success"
        aria-label="Valid"
      />
    {:else if validationStatus === "invalid"}
      <AlertCircle
        size="1em"
        class="underlay-input-validation__icon underlay-input-validation__icon--error"
        aria-label="Invalid"
      />
    {/if}
  </div>
{/if}

{#if showValidationMessage && validationMessage && showValidationStatus}
  <p class="underlay-input-validation__message underlay-input-validation__message--{validationStatus}">
    {validationMessage}
  </p>
{/if}

<style>
  .underlay-input-clear {
    position: absolute;
    right: 0.5em;
    top: 50%;
    transform: translateY(-50%);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.15em;
    border: none;
    background: transparent;
    color: var(--underlay-color-text-muted, #9ca3af);
    cursor: pointer;
    border-radius: 0.2rem;
    opacity: 0.7;
    transition: opacity 0.15s ease, color 0.15s ease;
  }

  .underlay-input-clear:hover {
    opacity: 1;
    color: var(--underlay-color-danger, #ef4444);
  }

  .underlay-input-clear:focus-visible {
    outline: 2px solid var(--underlay-color-primary, #2563eb);
    outline-offset: 1px;
  }

  .underlay-input-validation {
    position: absolute;
    right: var(--underlay-input-suffix-width, var(--underlay-space-3, 0.75rem));
    top: 50%;
    transform: translateY(-50%);
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.25rem;
    height: 1.25rem;
    pointer-events: none;
  }

  :global(.underlay-input-wrapper--suffixed) .underlay-input-validation {
    right: calc(var(--underlay-input-suffix-width, 0px) + var(--underlay-space-3, 0.75rem));
  }

  .underlay-input-validation__spinner {
    width: 1rem;
    height: 1rem;
    border: 2px solid var(--underlay-color-text-muted, #9ca3af);
    border-top-color: transparent;
    border-radius: 50%;
    animation: underlay-input-spin 0.8s linear infinite;
  }

  @keyframes underlay-input-spin {
    to {
      transform: rotate(360deg);
    }
  }

  .underlay-input-validation__icon {
    font-size: 1rem;
  }

  :global(.underlay-input-validation__icon--success) {
    color: var(--underlay-color-success, #22c55e) !important;
  }

  :global(.underlay-input-validation__icon--error) {
    color: var(--underlay-color-danger, #ef4444) !important;
  }

  .underlay-input-validation__message {
    margin: var(--underlay-space-2, 0.5rem) 0 0;
    font-size: var(--underlay-font-size-sm, 0.8rem);
  }

  .underlay-input-validation__message--validating {
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .underlay-input-validation__message--valid {
    color: var(--underlay-color-success, #22c55e);
  }

  .underlay-input-validation__message--invalid {
    color: var(--underlay-color-error, #ef4444);
  }
</style>
