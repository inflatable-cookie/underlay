<script lang="ts">
  import Plus from "lucide-svelte/icons/plus";

  interface Props {
    allowCreate?: boolean;
    hasCreateForm: boolean;
    createLabel: string;
    onCreate: () => void;
  }

  let {
    allowCreate = false,
    hasCreateForm,
    createLabel,
    onCreate
  }: Props = $props();
</script>

{#if allowCreate && hasCreateForm}
  <div class="relation-selector-popover__create">
    <button
      type="button"
      class="relation-selector-popover__create-button"
      onclick={onCreate}
    >
      <Plus size="0.9em" />
      <span>{createLabel}</span>
    </button>
  </div>
{/if}

<style>
  .relation-selector-popover__create {
    border-top: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.3));
    margin-top: 0.35rem;
    padding-top: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .relation-selector-popover__create-button {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    width: 100%;
    padding: 0.45rem 0.5rem;
    border: none;
    border-radius: 0.3rem;
    background: transparent;
    color: var(--underlay-color-primary, #2563eb);
    font-size: 0.8rem;
    cursor: pointer;
    text-align: left;
  }

  .relation-selector-popover__create-button:hover {
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.15));
  }

  .relation-selector-popover__create-button:focus-visible {
    outline: var(--underlay-focus-ring-width, 2px) solid
      var(--underlay-color-primary, #2563eb);
    outline-offset: -1px;
  }
</style>
