<script lang="ts">
  import RelationPickerFooter from "../../src/patterns/relation-picker/RelationPickerFooter.svelte";

  interface Props {
    multiSelect: boolean;
    createFormOpen: boolean;
    selectedCount: number;
    useFooterSnippet?: boolean;
    onCancel?: () => void;
    onConfirm?: () => void;
  }

  let {
    multiSelect,
    createFormOpen,
    selectedCount,
    useFooterSnippet = false,
    onCancel,
    onConfirm
  }: Props = $props();
</script>

{#snippet footerSnippet()}
  <div data-testid="custom-footer">Custom Footer</div>
{/snippet}

<RelationPickerFooter
  multiSelect={multiSelect}
  createFormOpen={createFormOpen}
  selectedCount={selectedCount}
  onCancel={onCancel}
  onConfirm={onConfirm}
  footer={useFooterSnippet ? footerSnippet : undefined}
/>
