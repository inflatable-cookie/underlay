<script lang="ts">
  export let message: string | null | undefined = null;
</script>

{#if message}
  <p class="underlay-form-error" role="alert" aria-live="polite">
    {message}
  </p>
{/if}

<style>
  .underlay-form-error {
    color: var(--underlay-color-error, #ef4444);
    margin: 0 0 var(--underlay-space-4, 1rem);
    font-size: var(--underlay-font-size-sm, 0.9rem);
  }
</style>
