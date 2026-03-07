<script lang="ts">
  import ReorderableList from "../../src/patterns/ReorderableList.svelte";

  interface ReorderItem {
    id: string;
    label: string;
  }

  interface Controller {
    pending: ReorderItem[];
    isDirty: boolean;
    isPending: boolean;
    move: (fromIndex: number, toIndex: number) => void;
    submit: () => Promise<void>;
    reset: () => void;
    updatePending: (items: ReorderItem[]) => void;
  }

  interface Props {
    controller: Controller;
    oncancel?: () => void;
    onsuccess?: () => void;
    onsubmiterror?: (error: unknown) => void | string | Promise<void | string>;
    disabled?: boolean;
    saveLabel?: string;
    cancelLabel?: string;
  }

  let {
    controller,
    oncancel = () => undefined,
    onsuccess,
    onsubmiterror,
    disabled = false,
    saveLabel = "Save Order",
    cancelLabel = "Cancel"
  }: Props = $props();
</script>

<ReorderableList
  {controller}
  {oncancel}
  {onsuccess}
  {onsubmiterror}
  {disabled}
  {saveLabel}
  {cancelLabel}
>
  {#snippet item(row)}
    <div data-testid={`row-${row.id}`}>{row.label}</div>
  {/snippet}
  {#snippet empty()}
    <div data-testid="empty-list">No rows</div>
  {/snippet}
</ReorderableList>
