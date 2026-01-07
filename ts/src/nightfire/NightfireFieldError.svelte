<script lang="ts">
  export let message: string | null = null;
</script>

{#if message}
  <p class="nightfire-field__error">
    {message}
  </p>
{/if}

<style>
  .nightfire-field__error {
    margin-top: calc(var(--underlay-space-2) * 0.8);
    font-size: var(--underlay-font-size-sm);
    color: var(--underlay-color-danger);
  }
</style>
