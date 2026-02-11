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

  // Normalize the block and derive values reactively
  // This ensures child editors always receive the latest data
  const normalisedBlock = $derived(normaliseNightfireBlock(block, typeOptions, definition));
  const currentBlockType = $derived(normalisedBlock.type);
  const BlockEditor = $derived(getBlockEditor(schema, currentBlockType));

  function handleBlockEditorChange(next: any) {
    const candidate = next && typeof next === "object" && "block" in next
      ? (next as { block?: any }).block
      : next;
    const normalised = normaliseNightfireBlock(candidate, typeOptions, definition);
    onChange?.(normalised);
  }
</script>

<div class="nightfire-editor nightfire-editor--single">
  {#if BlockEditor}
    {#key currentBlockType}
      {@const EditorComponent = BlockEditor}
      <EditorComponent
        block={normalisedBlock}
        value={{ schema, block: normalisedBlock }}
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
