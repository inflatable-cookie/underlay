<script lang="ts">
  import { BlockEditor } from "@poodle/svelte";
  import {
    buildNightfireBlockEditorBridge,
    toPoodleEditorBlocks,
  } from "@poodle/bridge-underlay/nightfire-block-editor";

  import type { MarkdownEditorContext } from "../markup/markdown-editor-context";
  import type { NightfireBlockDefinition, NightfireTypeOption } from "../utils";
  import type { GroupedOptions } from "./grouped-options";
  import type { NightfireSlashCommand } from "../slash-commands";
  import NightfireBlockEditor from "../NightfireBlockEditor.svelte";
  import NightfireTypeSelect from "./NightfireTypeSelect.svelte";
  import SlashCommandPalette from "../SlashCommandPalette.svelte";

  interface Props {
    schema: string;
    isMulti: boolean;
    definition: NightfireBlockDefinition;
    editorTypeOptions: NightfireTypeOption[];
    groupedOptions: GroupedOptions[] | null;
    singleBlock: any;
    blocks: any[];
    slashState: {
      blockIndex: number;
      start: number;
      end: number;
      query: string;
    } | null;
    filteredSlashCommands: NightfireSlashCommand[];
    onSingleBlockChange: (nextBlock: any) => void;
    onBlocksChange: (nextBlocks: any[]) => void;
    onSingleTypeChange: (nextType: string) => void;
    onTypeChange: (index: number, nextType: string) => void;
    onBlockContextChange?: (index: number, context: MarkdownEditorContext) => void;
    onSlashQueryChange: (query: string) => void;
    onSlashCommandSelect: (command: NightfireSlashCommand) => void;
    onCloseSlashPalette: () => void;
    onAddFirstBlock: () => void;
  }

  let {
    schema,
    isMulti,
    definition,
    editorTypeOptions,
    groupedOptions,
    singleBlock,
    blocks,
    slashState,
    filteredSlashCommands,
    onSingleBlockChange,
    onBlocksChange,
    onSingleTypeChange,
    onTypeChange,
    onBlockContextChange = () => {},
    onSlashQueryChange,
    onSlashCommandSelect,
    onCloseSlashPalette,
    onAddFirstBlock,
  }: Props = $props();

  const bridge = $derived(
    buildNightfireBlockEditorBridge({
      typeOptions: editorTypeOptions,
      groupedOptions,
    }),
  );

  const shellBlocks = $derived.by(() => {
    if (isMulti) {
      return toPoodleEditorBlocks(blocks as any[], {
        fallbackType: definition.defaultType,
      });
    }

    return toPoodleEditorBlocks(
      [
        singleBlock ?? {
          type: definition.defaultType,
          version: "initial",
          hash: "",
          data: {},
        },
      ],
      {
        fallbackType: definition.defaultType,
      },
    );
  });

  function handleShellChange(nextBlocks: any[]): void {
    const normalisedBlocks = nextBlocks.map((block) => {
      if (!block || typeof block !== "object") {
        return block;
      }

      // Poodle's BlockEditor uses `content` as its minimal editable payload.
      // Nightfire's markdown editor expects `data.text`. If we don't bridge this,
      // add/type/reorder operations coming from the shell can appear to "wipe"
      // the field because `content` is not part of the Nightfire envelope.
      const record = block as Record<string, unknown>;
      const type = typeof record.type === "string" ? record.type : null;
      const content = typeof record.content === "string" ? record.content : null;

      if (type !== "markdown" || content === null) {
        return block;
      }

      const data =
        record.data !== null && typeof record.data === "object" && !Array.isArray(record.data)
          ? (record.data as Record<string, unknown>)
          : {};

      if (typeof data.text !== "string") {
        return {
          ...record,
          data: {
            ...data,
            text: content,
          },
        };
      }

      return block;
    });

    if (isMulti) {
      onBlocksChange(normalisedBlocks);
      return;
    }

    onSingleBlockChange(normalisedBlocks[0] ?? null);
  }

  function handleTypePickerChange(index: number, nextType: string): void {
    if (isMulti) {
      onTypeChange(index, nextType);
      return;
    }

    onSingleTypeChange(nextType);
  }

  function handleBlockEditorChange(index: number, nextBlock: any): void {
    if (isMulti) {
      const nextBlocks = blocks.slice();
      nextBlocks[index] = nextBlock;
      onBlocksChange(nextBlocks);
      return;
    }

    onSingleBlockChange(nextBlock);
  }
</script>

{#if isMulti && shellBlocks.length === 0}
  <div class="underlay-nightfire-field__empty">
    <button
      type="button"
      class="underlay-nightfire-field__multi-add"
      onclick={onAddFirstBlock}
    >
      + Add block
    </button>
  </div>
{:else}
  <BlockEditor
    blocks={shellBlocks}
    blockTypes={bridge.blockTypes}
    blockTypeItems={bridge.blockTypeItems}
    mode={isMulti ? "multi" : "single"}
    onChange={handleShellChange}
  >
    {#snippet typePicker({ block, index })}
      <NightfireTypeSelect
        value={block?.type ?? editorTypeOptions[0]?.type ?? definition.defaultType}
        onChange={(value) => handleTypePickerChange(index, value)}
        {groupedOptions}
        typeOptions={editorTypeOptions}
        variant="ghost"
        menuMinWidth="10rem"
      />
    {/snippet}

    {#snippet block({ block, index })}
      <div
        class="underlay-nightfire-field__block-content"
        data-nightfire-block-card
        data-block-index={index}
      >
        <NightfireBlockEditor
          {schema}
          block={block}
          definition={definition}
          typeOptions={editorTypeOptions}
          onChange={(next) => handleBlockEditorChange(index, next)}
          onContextChange={(context) => onBlockContextChange(index, context)}
        />

        {#if isMulti && slashState?.blockIndex === index}
          <div class="underlay-nightfire-field__multi-slash-palette">
            <SlashCommandPalette
              commands={filteredSlashCommands}
              query={slashState?.query ?? ""}
              onQueryChange={onSlashQueryChange}
              onSelect={onSlashCommandSelect}
              onClose={onCloseSlashPalette}
            />
          </div>
        {/if}
      </div>
    {/snippet}
  </BlockEditor>
{/if}

<style>
  .underlay-nightfire-field__block-content {
    display: grid;
    gap: var(--underlay-space-2);
  }

  .underlay-nightfire-field__empty {
    display: flex;
    justify-content: flex-start;
  }

  .underlay-nightfire-field__multi-slash-palette {
    margin-top: var(--underlay-space-2);
  }

  .underlay-nightfire-field__multi-add {
    font-size: calc(1em * var(--underlay-font-scale-xxs));
    padding: var(--underlay-button-chip-padding-block)
      var(--underlay-button-chip-padding-inline);
    border-radius: var(--underlay-radius-pill);
    border: 1px solid var(--underlay-color-border-strong);
    background: transparent;
    color: inherit;
    cursor: pointer;
  }
</style>
