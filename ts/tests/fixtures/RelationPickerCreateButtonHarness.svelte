<script lang="ts">
  import RelationPickerCreateButton from "../../src/patterns/relation-picker/RelationPickerCreateButton.svelte";

  interface Props {
    allowCreate: boolean;
    createFormOpen: boolean;
    createLabel: string;
    useCreateForm?: boolean;
    onCreate?: () => void;
  }

  let {
    allowCreate,
    createFormOpen,
    createLabel,
    useCreateForm = true,
    onCreate
  }: Props = $props();
</script>

{#snippet createFormSnippet(_onSuccess, _onCancel)}
  <div data-testid="create-form-snippet">Create Form</div>
{/snippet}

<RelationPickerCreateButton
  allowCreate={allowCreate}
  createFormOpen={createFormOpen}
  createForm={useCreateForm ? createFormSnippet : undefined}
  createLabel={createLabel}
  onCreate={onCreate}
/>
