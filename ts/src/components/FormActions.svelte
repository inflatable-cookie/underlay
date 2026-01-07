<script lang="ts">
  export let align: "start" | "end" = "start";
</script>

<div class={`underlay-form-actions underlay-form-actions--${align}`}>
  <slot />
</div>

<style>
  .underlay-form-actions {
    margin-top: calc(
      var(--underlay-density-gap, var(--underlay-density-gap, 0.75rem)) * 1.5
    );
    display: flex;
    gap: var(--underlay-density-gap, var(--underlay-density-gap, 0.75rem));
    align-items: center;
  }

  .underlay-form-actions--start {
    justify-content: flex-start;
  }

  .underlay-form-actions--end {
    justify-content: flex-end;
  }
</style>
