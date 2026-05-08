<script lang="ts">
  import { onMount, tick, untrack } from "svelte";
  import type { MarkdownEditorContext } from "./markup/markdown-editor-context";
  import type { NightfireDraftValue } from "./types";
  // Ensure registrations are loaded before we lookup schema definitions
  import "./editor-registrations";
  import {
    getBlockTypeOptionsForSchema
  } from "./editor-registry";
  import {
    isEmptyNightfire
  } from "./utils";
  import {
    useNightfireStrategies,
    type NightfireStrategy,
    type NightfireBlockOption
  } from "./strategies";
  import {
    buildGroupedOptions,
    type NightfireBlockOptionInput
  } from "./editor/grouped-options";
  import { resolveSchemaDefinition } from "./editor/schema-resolution";
  import {
    createPrepareWriter,
    createRequiredInitialValue,
    type NightfireFieldMode
  } from "./editor/field-lifecycle";
  import NightfireFieldBlockShell from "./editor/NightfireFieldBlockShell.svelte";
  import { normaliseForStrategy } from "./editor/strategy-normalisation";
  import {
    addBlock as addEditorBlock,
    asMultiBlockValue,
    asSingleBlockValue,
    changeBlockType,
    changeSingleBlockType,
    insertBlockAfter,
    moveBlock as moveEditorBlock,
    removeBlock as removeEditorBlock,
    replaceBlockAtIndex
  } from "./editor/value-updates";
  import {
    buildNightfireSlashCommands,
    filterNightfireSlashCommands,
    findNightfireSlashMatch,
    removeNightfireSlashText,
    type NightfireSlashCommand,
    type NightfireSlashCommandsConfig
  } from "./slash-commands";

  /**
   * Field-level Nightfire editor.
   *
   * - Owns the NightfireValue (single or multi) for a given schema.
   * - Delegates single-block editing to NightfireBlockEditor.
   * - Provides an optional `prepare` hook so forms can serialise the
   *   current value into FormData just before submit.
   * - Automatically loads and applies Nightfire strategies when configured.
   * - Handles value normalisation internally.
   */

  export interface SchemaMismatchInfo {
    actualSchema: string | null;
    expectedSchema: string;
  }

  interface Props {
    name: string;
    schema: string;
    value: NightfireDraftValue;
    /**
     * Optional overrides derived from a Nightfire strategy.
     * @deprecated Use automatic strategy loading via configureNightfireStrategies() instead.
     *
     * - `modeOverride` lets the host control single vs multi editing
     *   based on strategy cardinality.
     * - `defaultTypeOverride` lets the host control which block type
     *   is used for new blocks.
     * - `blockOptions` provides a categorised list of available blocks,
     *   typically coming from Farmyard via Cattle Grid.
     *
     * When these are not provided, the editor falls back to the
     * local registry definitions so it remains usable in student views
     * or in tests without a strategy API.
     */
    modeOverride?: NightfireFieldMode | null;
    defaultTypeOverride?: string | null;
    blockOptions?: NightfireBlockOptionInput[] | null;
    /**
     * Whether this Nightfire field is required from the UI's
     * perspective. When true and the value is empty, a simple
     * client-side validation message is shown.
     */
    required?: boolean;
    /**
     * Optional callback invoked whenever the NightfireValue changes.
     */
    onChange?: (value: NightfireDraftValue) => void;
    /**
     * Hook used with Underlay Form's `prepare`. When bound from a parent
     * (via `bind:prepare`), the editor will populate it with an
     * implementation that writes the current Nightfire value into
     * `FormData[name]` just before submit.
     */
    prepare?: (formData: FormData) => void;
    /**
     * Callback invoked when a schema mismatch is detected during normalisation.
     * This allows the parent to display a warning to the user.
     */
    onSchemaMismatch?: (info: SchemaMismatchInfo) => void;
    slashCommands?: NightfireSlashCommandsConfig | null;
  }

  let {
    name,
    schema,
    value = $bindable(),
    modeOverride = null,
    defaultTypeOverride = null,
    blockOptions = null,
    required = false,
    onChange = () => {},
    prepare = $bindable(() => {}),
    onSchemaMismatch,
    slashCommands = null
  }: Props = $props();

  // Strategy loading
  const strategiesStore = useNightfireStrategies();
  let strategy = $state<NightfireStrategy | null>(null);
  let strategiesLoading = $state(false);
  let lastSchemaMismatchReported = $state<string | null>(null);

  // Load strategy on mount
  onMount(() => {
    if (strategiesStore) {
      strategiesLoading = true;
      strategiesStore.ensure().then(() => {
        strategy = strategiesStore.findById(schema);
        strategiesLoading = false;
      });
    }
  });

  // Derive effective overrides from strategy (if no manual overrides provided)
  const effectiveModeOverride = $derived(
    modeOverride ?? strategy?.cardinality.mode ?? null
  );
  const effectiveDefaultTypeOverride = $derived(
    defaultTypeOverride ?? strategy?.defaultType ?? null
  );
  const effectiveBlockOptions = $derived(
    blockOptions ?? (strategy?.blockOptions as NightfireBlockOptionInput[] | null) ?? null
  );

  /**
   * When switching block types for the summary schema, we apply
   * some simple layout-switching semantics so we:
   *
   * - Migrate shared fields (e.g. title/body pages, subtitles)
   *   where possible.
   * - Warn when a change may drop or reinterpret fields (e.g.
   *   image IDs, additional pages when moving to the image
   *   slider layout).
   */
  const FALLBACK_MARKUP_SCHEMA_ID = "acow:content/markup@1";

  let typeChangeWarning: string | null = $state(null);
  let hasInitialisedRequired = $state(false);

  /**
   * Editing schema vs storage schema
   *
   * - `schema` (prop) is the storage schema on the NightfireValue.
   * - `editorSchema` is the schema used to look up block editors and
   *   type options.
   *
   * When no explicit schema definition or block editors exist for the
   * requested schema, we fall back to the generic markup schema so that
   * fields can still be edited using basic text blocks.
   */

  // Initialize with correct values IMMEDIATELY (not deferred via $effect)
  // Use untrack to capture initial values without creating reactive dependencies
  const initialResolved = untrack(() =>
    resolveSchemaDefinition(schema, FALLBACK_MARKUP_SCHEMA_ID)
  );
  let editorSchema: string = $state(initialResolved.editorSchema);
  let registryDef: {
    schema: string;
    mode: NightfireFieldMode;
    defaultType: string;
  } = $state(initialResolved.registryDef);

  // Track the last schema prop to avoid redundant updates
  let lastSchemaProp = untrack(() => schema);

  // Update when schema prop changes (for subsequent navigations)
  $effect(() => {
    // Only update if schema prop actually changed
    if (schema === lastSchemaProp) {
      return;
    }
    lastSchemaProp = schema;

    const resolved = resolveSchemaDefinition(schema, FALLBACK_MARKUP_SCHEMA_ID);
    editorSchema = resolved.editorSchema;
    registryDef = resolved.registryDef;
  });

  const effectiveDef = $derived({
    schema: editorSchema,
    mode: (effectiveModeOverride ?? registryDef.mode) as NightfireFieldMode,
    defaultType: effectiveDefaultTypeOverride ?? registryDef.defaultType ?? "markdown"
  });

  // Keep stored value shape aligned with the effective editor mode.
  // This is important for schemas that don't have a local registry entry
  // (e.g. acow:content/qa@1), where we may fall back to markup editors but
  // still receive single-block payloads from the API.
  $effect(() => {
    const { coerced, schemaMismatch } = normaliseForStrategy(value, schema, effectiveDef.mode);

    if (schemaMismatch && schemaMismatch !== lastSchemaMismatchReported) {
      lastSchemaMismatchReported = schemaMismatch;
      onSchemaMismatch?.({ actualSchema: schemaMismatch, expectedSchema: schema });
    }

    const current = value as unknown as Record<string, unknown> | null | undefined;
    const next = coerced as unknown as Record<string, unknown>;
    const currentBlocks = Array.isArray(current?.blocks) ? (current?.blocks as unknown[]) : null;
    const nextBlocks = Array.isArray(next.blocks) ? (next.blocks as unknown[]) : null;

    const needsShapeUpdate =
      !current ||
      current.schema !== next.schema ||
      (!!current.block !== !!next.block) ||
      ((currentBlocks?.length ?? 0) !== (nextBlocks?.length ?? 0));

    if (needsShapeUpdate) {
      value = coerced;
    }
  });

  const baseTypeOptions = $derived(
    effectiveBlockOptions && effectiveBlockOptions.length > 0
      ? effectiveBlockOptions
      : getBlockTypeOptionsForSchema(editorSchema)
  );

  const editorTypeOptions = $derived(
    baseTypeOptions.map((opt) => ({
      type: opt.type,
      label: opt.label
    }))
  );

  const groupedOptions = $derived(
    effectiveBlockOptions && effectiveBlockOptions.some((o) => !!o.category)
      ? buildGroupedOptions(effectiveBlockOptions)
      : null
  );

  const isMulti = $derived(effectiveDef.mode === "multi" || Array.isArray(value?.blocks));
  const slashCommandsEnabled = $derived(Boolean(slashCommands?.enabled) && isMulti);
  const availableSlashCommands = $derived(
    slashCommandsEnabled
      ? buildNightfireSlashCommands(editorTypeOptions, slashCommands)
      : []
  );

  // Single-block view - derives from value reactively
  // This ensures child editors always receive the latest data
  const singleBlock = $derived(isMulti ? null : ((value?.block as any) ?? null));
  // Multi-block state view - use $derived.by to ensure stable reference
  const blocks = $derived.by(() => {
    if (isMulti && Array.isArray(value?.blocks)) {
      return value.blocks as any[];
    }
    return [];
  });
  let slashState = $state<{
    blockIndex: number;
    start: number;
    end: number;
    query: string;
  } | null>(null);
  const filteredSlashCommands = $derived(
    slashState
      ? filterNightfireSlashCommands(availableSlashCommands, slashState.query)
      : []
  );

  $effect(() => {
    if (!slashCommandsEnabled && slashState) {
      closeSlashPalette();
    }
  });

  function emit(nextValue: NightfireDraftValue) {
    value = nextValue;
    onChange?.(nextValue);
  }

  function getLabelForType(type: string): string {
    const match = editorTypeOptions.find((opt) => opt.type === type);
    return match?.label ?? type;
  }

  function handleSingleBlockChange(nextBlock: any) {
    emit(asSingleBlockValue(schema, nextBlock));
  }

  function handleBlockChange(index: number, nextBlock: any) {
    emit(asMultiBlockValue(schema, replaceBlockAtIndex(blocks, index, nextBlock)));
  }

  function handleSingleTypeChange(nextType: string) {
    const transformed = changeSingleBlockType(schema, singleBlock, nextType, getLabelForType);
    typeChangeWarning = transformed.warning;
    handleSingleBlockChange(transformed.block);
  }

  function handleTypeChange(index: number, nextType: string) {
    handleBlockChange(index, changeBlockType(blocks[index], nextType));
  }

  function addBlock() {
    const defaultType =
      editorTypeOptions[0]?.type ?? effectiveDef.defaultType ?? "markdown";
    emit(asMultiBlockValue(schema, addEditorBlock(blocks, defaultType)));
  }

  function removeBlock(index: number) {
    emit(asMultiBlockValue(schema, removeEditorBlock(blocks, index)));
  }

  function moveBlock(from: number, to: number) {
    const nextBlocks = moveEditorBlock(blocks, from, to);
    if (!nextBlocks) {
      return;
    }
    emit(asMultiBlockValue(schema, nextBlocks));
  }

  function closeSlashPalette() {
    slashState = null;
  }

  function focusBlockCard(index: number) {
    void tick().then(() => {
      const root = document.querySelector(
        `[data-nightfire-block-card][data-block-index="${index}"]`
      ) as HTMLElement | null;

      if (!root) {
        return;
      }

      const focusTarget = root.querySelector<HTMLElement>(
        [
          ".CodeMirror textarea",
          "textarea:not(.underlay-is-hidden)",
          "input:not([type='hidden'])",
          "select",
          "button"
        ].join(", ")
      );

      focusTarget?.focus();
    });
  }

  function handleSlashContextChange(index: number, context: MarkdownEditorContext) {
    if (!slashCommandsEnabled) {
      if (slashState?.blockIndex === index) {
        closeSlashPalette();
      }
      return;
    }

    const block = blocks[index] as { type?: string } | undefined;
    if (block?.type !== "markdown") {
      if (slashState?.blockIndex === index) {
        closeSlashPalette();
      }
      return;
    }

    const match = findNightfireSlashMatch(context);
    if (!match) {
      if (slashState?.blockIndex === index) {
        closeSlashPalette();
      }
      return;
    }

    slashState = {
      blockIndex: index,
      start: match.start,
      end: match.end,
      query: match.query
    };
  }

  function handleSlashQueryChange(query: string) {
    if (!slashState) {
      return;
    }

    slashState = {
      ...slashState,
      query
    };
  }

  function handleSlashCommandSelect(command: NightfireSlashCommand) {
    if (!slashState) {
      return;
    }

    const slashTarget = slashState;
    const currentBlock = blocks[slashState.blockIndex] as {
      type?: string;
      version?: string;
      hash?: string;
      data?: { text?: string };
    } | undefined;
    const currentText = currentBlock?.data?.text ?? "";
    const nextCurrentBlock = {
      type: currentBlock?.type ?? "markdown",
      version: currentBlock?.version ?? "initial",
      hash: currentBlock?.hash ?? "",
      data: {
        ...(currentBlock?.data ?? {}),
        text: removeNightfireSlashText(currentText, slashState)
      }
    };
    const updatedBlocks = replaceBlockAtIndex(blocks, slashTarget.blockIndex, nextCurrentBlock);
    const nextBlocks = insertBlockAfter(updatedBlocks, slashTarget.blockIndex, command.type);

    closeSlashPalette();
    emit(asMultiBlockValue(schema, nextBlocks));
    focusBlockCard(slashTarget.blockIndex + 1);
  }

  // For required fields, ensure there is at least one block when the
  // editor is first initialised and the value is empty.
  $effect(() => {
    if (!hasInitialisedRequired && required && isEmptyNightfire(value)) {
      value = createRequiredInitialValue(
        schema,
        effectiveDef.mode,
        effectiveDef.defaultType ?? "markdown"
      );

      hasInitialisedRequired = true;
    }
  });

  const isEmpty = $derived(isEmptyNightfire(value));
  const showRequiredError = $derived(required && isEmpty);

  // Form serialisation hook: always serialise the current NightfireValue
  // (single or multi) into FormData[name].
  // Use a ref pattern to avoid creating a new function on every value change,
  // which would propagate through bindings and potentially cause re-renders.
  // Use untrack to capture initial values - the $effect below keeps them updated.
  let valueRef = { current: untrack(() => value) };
  let nameRef = { current: untrack(() => name) };

  $effect(() => {
    valueRef.current = value;
    nameRef.current = name;
  });

  // Set prepare only once - it reads current values from refs at call time
  prepare = createPrepareWriter(
    () => valueRef.current,
    () => nameRef.current
  );
</script>

<div class="underlay-nightfire-field">
  <NightfireFieldBlockShell
    schema={editorSchema}
    {isMulti}
    definition={effectiveDef}
    {editorTypeOptions}
    {groupedOptions}
    {singleBlock}
    {blocks}
    {slashState}
    {filteredSlashCommands}
    onSingleBlockChange={handleSingleBlockChange}
    onBlocksChange={(nextBlocks) => emit(asMultiBlockValue(schema, nextBlocks))}
    onSingleTypeChange={handleSingleTypeChange}
    onTypeChange={handleTypeChange}
    onBlockContextChange={handleSlashContextChange}
    onSlashQueryChange={handleSlashQueryChange}
    onSlashCommandSelect={handleSlashCommandSelect}
    onCloseSlashPalette={() => {
      if (slashState) {
        focusBlockCard(slashState.blockIndex);
      }
      closeSlashPalette();
    }}
    onAddFirstBlock={addBlock}
  />

  {#if showRequiredError}
    <p class="underlay-nightfire-field__error">This field is required.</p>
  {/if}
  {#if typeChangeWarning}
    <p class="underlay-nightfire-field__layout-warning">
      {typeChangeWarning}
    </p>
  {/if}
</div>

<style>
  .underlay-nightfire-field {
    display: grid;
    gap: var(--underlay-space-2);
    color: var(--underlay-color-text);
  }

  .underlay-nightfire-field__error {
    margin-top: calc(var(--underlay-space-2) * 0.8);
    font-size: var(--underlay-font-size-sm);
    color: var(--underlay-color-danger);
  }

  .underlay-nightfire-field__layout-warning {
    margin-top: var(--underlay-space-2);
    font-size: calc(1em * var(--underlay-font-scale-xs));
    color: var(--underlay-color-danger);
  }
</style>
