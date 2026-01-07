<script lang="ts">
  import { getBlockEditor } from "./editor-registry";
  import "./render-registrations";
  import "./editor-registrations";
  import {
    normaliseNightfireBlock,
    type NightfireBlockDefinition,
    type NightfireTypeOption
  } from "./utils";

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

  export let schema: string;
  export let block: any;

  /**
   * Optional callback invoked whenever the block changes.
   */
  export let onChange: (block: any) => void = () => {};

  /**
   * Pre-resolved schema definition and type options. NightfireBlockEditor
   * does not perform any registry lookups itself; these must be
   * provided by the caller (typically the field-level Nightfire editor).
   */
  export let definition: NightfireBlockDefinition;

  export let typeOptions: NightfireTypeOption[];

  let internalBlock: any = normaliseNightfireBlock(
    block,
    typeOptions,
    definition
  );

  $: if (block !== internalBlock) {
    internalBlock = normaliseNightfireBlock(block, typeOptions, definition);
  }

  function emitChange(nextBlock: any) {
    internalBlock = normaliseNightfireBlock(nextBlock, typeOptions, definition);
    onChange?.(internalBlock);
  }

  function handleBlockEditorChange(nextBlock: any) {
    emitChange(nextBlock);
  }
</script>

<div class="nightfire-editor nightfire-editor--single">
  {#if internalBlock}
    {#if getBlockEditor(schema, internalBlock.type)}
      <svelte:component
        this={getBlockEditor(schema, internalBlock.type)}
        block={internalBlock}
        onChange={handleBlockEditorChange}
      />
    {:else}
      <p>
        No editor found for type
        <code>{internalBlock.type}</code>. You can still edit this value as JSON
        in Dairy.
      </p>
    {/if}
  {/if}
</div>

<style>
  .nightfire-editor {
    display: grid;
    gap: var(--froyo-density-gap);
  }
</style>
