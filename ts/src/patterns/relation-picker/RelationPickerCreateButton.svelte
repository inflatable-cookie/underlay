<script lang="ts">
  import type { Snippet } from "svelte";
  import Plus from "lucide-svelte/icons/plus";
  import type { PickableItem } from "../relation-picker-types.js";

  interface Props {
    allowCreate: boolean;
    createFormOpen: boolean;
    createForm?: Snippet<[onSuccess: (item: PickableItem) => void, onCancel: () => void]>;
    createLabel: string;
    onCreate?: () => void;
  }

  let {
    allowCreate,
    createFormOpen,
    createForm,
    createLabel,
    onCreate
  }: Props = $props();
</script>

{#if allowCreate && createForm && !createFormOpen}
  <div class="relation-picker-dialog__create">
    <button
      type="button"
      class="relation-picker-dialog__create-button"
      onclick={() => onCreate?.()}
    >
      <Plus size="1em" />
      <span>{createLabel}</span>
    </button>
  </div>
{/if}

<style>
  .relation-picker-dialog__create {
    border-top: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.3));
    margin-top: 0.5rem;
    padding-top: 0.75rem;
  }

  .relation-picker-dialog__create-button {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    width: 100%;
    padding: 0.6rem 0.7rem;
    border: none;
    border-radius: 0.35rem;
    background: transparent;
    color: var(--underlay-color-primary, #2563eb);
    font-size: 0.85rem;
    cursor: pointer;
    text-align: left;
  }

  .relation-picker-dialog__create-button:hover {
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.15));
  }

  .relation-picker-dialog__create-button:focus-visible {
    outline: var(--underlay-focus-ring-width, 2px) solid
      var(--underlay-color-primary, #2563eb);
    outline-offset: -1px;
  }
</style>
