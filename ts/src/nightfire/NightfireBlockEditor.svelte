<script lang="ts">
  import { getBlockEditor } from "./editor-registry";
  import "./render-registrations";
  import "./editor-registrations";
  import {
    normaliseNightfireBlock,
    type NightfireBlockDefinition,
    type NightfireTypeOption
  } from "./utils";
  import { untrack } from "svelte";

  /**
   * Single-block Nightfire editor.
   *
   * This component edits a single block for a given schema. It:
   * - Renders the appropriate block editor for the current type.
   * - Emits the updated block via `onChange` whenever it changes.
   *
   * It does not know about NightfireValue, single vs multi fields, or form
   * serialisation; those concerns live in the field-level Nightfire editor,
   * which is responsible for type selection and cardinality.
   */

  interface Props {
    schema: string;
    block: any;
    /**
     * Optional callback invoked whenever the block changes.
     */
    onChange?: (block: any) => void;
    /**
     * Pre-resolved schema definition and type options. NightfireBlockEditor
     * does not perform any registry lookups itself; these must be
     * provided by the caller (typically the field-level Nightfire editor).
     */
    definition: NightfireBlockDefinition;
    typeOptions: NightfireTypeOption[];
  }

  let {
    schema,
    block,
    onChange = () => {},
    definition,
    typeOptions
  }: Props = $props();

  // Initialize block state once on mount - child editors manage their own state
  // after that, so we only need to update when the block TYPE changes.
  const initialBlock = untrack(() => normaliseNightfireBlock(block, typeOptions, definition));

  let currentBlockType = $state(initialBlock.type);
  let BlockEditor: any = $state(getBlockEditor(schema, currentBlockType));
  let internalBlock = $state(initialBlock);

  // Update only when the block TYPE changes (e.g., user selects different block type)
  $effect(() => {
    const newBlock = normaliseNightfireBlock(block, typeOptions, definition);
    if (newBlock.type !== currentBlockType) {
      currentBlockType = newBlock.type;
      internalBlock = newBlock;
      BlockEditor = getBlockEditor(schema, currentBlockType);
    }
  });

  function handleBlockEditorChange(nextBlock: any) {
    const normalised = normaliseNightfireBlock(nextBlock, typeOptions, definition);
    onChange?.(normalised);
  }
</script>

<div class="nightfire-editor nightfire-editor--single">
  {#if BlockEditor}
    {#key currentBlockType}
      <svelte:component
        this={BlockEditor}
        block={internalBlock}
        onChange={handleBlockEditorChange}
      />
    {/key}
  {:else}
    <p>
      No editor found for type
      <code>{currentBlockType}</code>. You can still edit this value as JSON
      in Dairy.
    </p>
  {/if}
</div>

<style>
  .nightfire-editor {
    display: grid;
    gap: var(--underlay-density-gap);
  }
</style>
