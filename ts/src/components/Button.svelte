<script lang="ts">
  export let type: "button" | "submit" | "reset" = "button";
  export let disabled = false;
</script>

<button class="underlay-button" {type} {disabled}>
  <slot />
</button>

<style>
  .underlay-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    border-radius: 0.5rem;
    padding: 0.5rem 0.875rem;
    border: 1px solid rgba(148, 163, 184, 0.35);
    background: rgba(255, 255, 255, 0.03);
    color: inherit;
    cursor: pointer;
  }

  .underlay-button:disabled {
    opacity: 0.55;
    cursor: not-allowed;
  }
</style>
