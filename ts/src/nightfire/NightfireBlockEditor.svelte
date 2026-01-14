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

  // Initialize internalBlock synchronously so the correct editor is shown
  // on first render. Previously we used $state(null) + $effect which caused
  // the "No editor found" error on client-side navigation because the
  // $effect runs AFTER the initial render.
  const initialBlock = normaliseNightfireBlock(block, typeOptions, definition);
  let internalBlock: any = $state(initialBlock);

  // Track the last block reference to avoid redundant normalisation
  let lastBlockRef: any = block;

  // Update when props change (for subsequent updates after initial render)
  $effect(() => {
    // Re-normalise whenever block, typeOptions, or definition changes
    const currentBlock = block;

    // Only update if block reference actually changed
    if (currentBlock !== lastBlockRef) {
      lastBlockRef = currentBlock;
      internalBlock = normaliseNightfireBlock(currentBlock, typeOptions, definition);
    }
  });

  function emitChange(nextBlock: any) {
    const normalised = normaliseNightfireBlock(nextBlock, typeOptions, definition);
    // Update local state immediately so the editor reflects changes
    internalBlock = normalised;
    lastBlockRef = nextBlock;
    onChange?.(normalised);
  }

  function handleBlockEditorChange(nextBlock: any) {
    emitChange(nextBlock);
  }
</script>

<div class="nightfire-editor nightfire-editor--single">
  {#if internalBlock}
    {#if getBlockEditor(schema, internalBlock.type)}
      {@const BlockEditor = getBlockEditor(schema, internalBlock.type)}
      <BlockEditor
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
    gap: var(--underlay-density-gap);
  }
</style>
