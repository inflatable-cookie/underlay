<script lang="ts">
  import { onMount, untrack } from "svelte";
  import type { NightfireValue } from "./index";
  import { normaliseNightfireValue } from "./utils";
  import NightfireBlockEditor from "./NightfireBlockEditor.svelte";
  import NightfireFieldError from "./NightfireFieldError.svelte";
  // Ensure registrations are loaded before we lookup schema definitions
  import "./editor-registrations";
  import {
    getBlockTypeOptionsForSchema
  } from "./editor-registry";
  import {
    isEmptyNightfire,
    writeNightfireToFormData
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
  import {
    addBlockToList,
    createDefaultBlock,
    moveBlockInList,
    removeBlockFromList
  } from "./editor/block-list";
  import { resolveSchemaDefinition } from "./editor/schema-resolution";
  import {
    SUMMARY_SCHEMA_ID,
    transformSummaryBlockOnLayoutChange
  } from "./editor/summary-transform";

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

  type NightfireFieldMode = "single" | "multi";

  export interface SchemaMismatchInfo {
    actualSchema: string | null;
    expectedSchema: string;
  }

  interface Props {
    name: string;
    schema: string;
    value: NightfireValue;
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
    onChange?: (value: NightfireValue) => void;
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
    onSchemaMismatch
  }: Props = $props();

  // Strategy loading
  const strategiesStore = useNightfireStrategies();
  let strategy = $state<NightfireStrategy | null>(null);
  let strategiesLoading = $state(false);
  let hasNormalised = $state(false);

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

  // Normalise value when strategy loads (only once)
  $effect(() => {
    if (hasNormalised || !strategy || strategiesLoading) return;

    // Normalise the value
    const normalised = normaliseNightfireValue(value, schema);

    // Check for schema mismatch
    const actualSchema = (() => {
      if (!value || typeof value !== "object") return null;
      const s = (value as Record<string, unknown>).schema;
      return typeof s === "string" ? s : null;
    })();

    if (actualSchema && actualSchema !== schema) {
      onSchemaMismatch?.({ actualSchema, expectedSchema: schema });
    }

    // Coerce single vs multi shape based on strategy cardinality
    const mode = strategy.cardinality.mode;
    let coerced: NightfireValue = { ...normalised, schema } as NightfireValue;
    const record = coerced as unknown as Record<string, unknown>;
    const single = record.block ?? null;
    const multi = Array.isArray(record.blocks) ? (record.blocks as unknown[]) : undefined;

    if (mode === "single") {
      if (!single && multi && multi.length > 0) {
        coerced = { ...coerced, block: multi[0], blocks: undefined } as NightfireValue;
      } else if (single && multi) {
        coerced = { ...coerced, blocks: undefined } as NightfireValue;
      }
    } else {
      if (!multi && single) {
        coerced = { ...coerced, block: undefined, blocks: [single] } as NightfireValue;
      } else if (multi && single) {
        coerced = { ...coerced, block: undefined } as NightfireValue;
      }
    }

    // Only update if something changed
    if (JSON.stringify(coerced) !== JSON.stringify(value)) {
      value = coerced;
    }

    hasNormalised = true;
  });

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

  // Single-block view - derives from value reactively
  // This ensures child editors always receive the latest data
  const singleBlock = $derived(isMulti ? null : ((value?.block as any) ?? null));
  const singleBlockType = $derived(singleBlock?.type ?? null);

  // Multi-block state view - use $derived.by to ensure stable reference
  const blocks = $derived.by(() => {
    if (isMulti && Array.isArray(value?.blocks)) {
      return value.blocks as any[];
    }
    return [];
  });

  function emit(nextValue: NightfireValue) {
    value = nextValue;
    onChange?.(nextValue);
  }

  function getLabelForType(type: string): string {
    const match = editorTypeOptions.find((opt) => opt.type === type);
    return match?.label ?? type;
  }

  function handleSingleBlockChange(nextBlock: any) {
    emit({
      schema,
      block: nextBlock,
      blocks: undefined
    });
  }

  function handleBlockChange(index: number, nextBlock: any) {
    const nextBlocks = blocks.slice();
    nextBlocks[index] = nextBlock;
    emit({
      schema,
      block: undefined,
      blocks: nextBlocks
    });
  }

  function handleSingleTypeChange(event: Event) {
    const select = event.currentTarget as HTMLSelectElement;
    const nextType = select.value;
    const current = singleBlock ?? {};
    let nextBlock: any = {
      ...current,
      type: nextType
    };

    if (schema === SUMMARY_SCHEMA_ID) {
      const transformed = transformSummaryBlockOnLayoutChange(
        current,
        nextType,
        getLabelForType
      );
      nextBlock = transformed.block;
      typeChangeWarning = transformed.warning;
    } else {
      typeChangeWarning = null;
    }

    handleSingleBlockChange(nextBlock);
  }

  function handleTypeChange(index: number, event: Event) {
    const select = event.currentTarget as HTMLSelectElement;
    const nextType = select.value;
    const current = blocks[index] ?? {};

    const nextBlock = {
      ...current,
      type: nextType
    };

    handleBlockChange(index, nextBlock);
  }

  function addBlock() {
    const defaultType =
      editorTypeOptions[0]?.type ?? effectiveDef.defaultType ?? "markdown";
    const nextBlocks = addBlockToList(blocks, defaultType);
    emit({
      schema,
      block: undefined,
      blocks: nextBlocks
    });
  }

  function removeBlock(index: number) {
    const nextBlocks = removeBlockFromList(blocks, index);
    emit({
      schema,
      block: undefined,
      blocks: nextBlocks
    });
  }

  function moveBlock(from: number, to: number) {
    const nextBlocks = moveBlockInList(blocks, from, to);
    if (!nextBlocks) {
      return;
    }
    emit({
      schema,
      block: undefined,
      blocks: nextBlocks
    });
  }

  // For required fields, ensure there is at least one block when the
  // editor is first initialised and the value is empty.
  $effect(() => {
    if (!hasInitialisedRequired && required && isEmptyNightfire(value)) {
      const defaultBlock = createDefaultBlock(effectiveDef.defaultType ?? "markdown");

      if (effectiveDef.mode === "multi") {
        value = {
          schema,
          blocks: [defaultBlock]
        } as any;
      } else {
        value = {
          schema,
          block: defaultBlock
        } as any;
      }

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
  prepare = (formData: FormData) => {
    writeNightfireToFormData(formData, nameRef.current, valueRef.current);
  };
</script>

<div class="nightfire-field">
  {#if !isMulti}
    <div class="nightfire-field-single">
      <div class="nightfire-field-single__toolbar">
        <select
          value={(singleBlock as any)?.type ??
            editorTypeOptions[0]?.type ??
            effectiveDef.defaultType}
          onchange={(event) => handleSingleTypeChange(event)}
          aria-label="Block type"
        >
          {#if groupedOptions}
            {#each groupedOptions as group}
              {#if group.category}
                <optgroup label={group.label}>
                  {#each group.options as opt}
                    <option value={opt.type}>
                      {opt.label}
                    </option>
                  {/each}
                </optgroup>
              {:else}
                {#each group.options as opt}
                  <option value={opt.type}>
                    {opt.label}
                  </option>
                {/each}
              {/if}
            {/each}
          {:else}
            {#each editorTypeOptions as opt}
              <option value={opt.type}>
                {opt.label}
              </option>
            {/each}
          {/if}
        </select>
      </div>
      <NightfireBlockEditor
        schema={editorSchema}
        block={singleBlock}
        definition={effectiveDef}
        typeOptions={editorTypeOptions}
        onChange={handleSingleBlockChange}
      />
    </div>
  {:else}
    <div class="nightfire-field-multi">
      {#each blocks as block, index (index)}
        <div class="nightfire-field-multi__item">
          <div class="nightfire-field-multi__toolbar">
            <select
              value={(block as any)?.type ??
                editorTypeOptions[0]?.type ??
                effectiveDef.defaultType}
              onchange={(event) => handleTypeChange(index, event)}
              aria-label="Block type"
            >
              {#if groupedOptions}
                {#each groupedOptions as group}
                  {#if group.category}
                    <optgroup label={group.label}>
                      {#each group.options as opt}
                        <option value={opt.type}>
                          {opt.label}
                        </option>
                      {/each}
                    </optgroup>
                  {:else}
                    {#each group.options as opt}
                      <option value={opt.type}>
                        {opt.label}
                      </option>
                    {/each}
                  {/if}
                {/each}
              {:else}
                {#each editorTypeOptions as opt}
                  <option value={opt.type}>
                    {opt.label}
                  </option>
                {/each}
              {/if}
            </select>
            <div class="nightfire-field-multi__controls">
              <button
                type="button"
                class="nightfire-field-multi__icon-btn"
                onclick={() => moveBlock(index, index - 1)}
                disabled={index === 0}
                aria-label="Move block up"
              >
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 15-6-6-6 6"/></svg>
              </button>
              <button
                type="button"
                class="nightfire-field-multi__icon-btn"
                onclick={() => moveBlock(index, index + 1)}
                disabled={index === blocks.length - 1}
                aria-label="Move block down"
              >
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>
              </button>
              <button
                type="button"
                class="nightfire-field-multi__icon-btn nightfire-field-multi__icon-btn--danger"
                onclick={() => removeBlock(index)}
                aria-label="Remove block"
              >
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/><line x1="10" x2="10" y1="11" y2="17"/><line x1="14" x2="14" y1="11" y2="17"/></svg>
              </button>
            </div>
          </div>
          <NightfireBlockEditor
            schema={editorSchema}
            {block}
            definition={effectiveDef}
            typeOptions={editorTypeOptions}
            onChange={(next) => handleBlockChange(index, next)}
          />
        </div>
      {/each}

      <button
        type="button"
        class="nightfire-field-multi__add"
        onclick={addBlock}
      >
        + Add block
      </button>
    </div>
  {/if}

  <NightfireFieldError message={showRequiredError ? "This field is required." : null} />
  {#if typeChangeWarning}
    <p class="nightfire-layout-warning">
      {typeChangeWarning}
    </p>
  {/if}
</div>

<style>
  .nightfire-field {
    border-radius: var(--underlay-radius-md);
    color: var(--underlay-color-text);
    padding: calc(var(--underlay-card-padding, 1.25rem) / 2);
    background-color: rgba(255, 255, 255, 0.03);
  }

  .nightfire-field-multi {
    display: grid;
    gap: var(--underlay-density-gap);
  }

  .nightfire-field-single {
    display: grid;
    gap: var(--underlay-space-2);
  }

  .nightfire-field-single__toolbar {
    display: flex;
    align-items: flex-start;
    gap: var(--underlay-space-2);
  }

  .nightfire-field-single__toolbar select {
    padding: var(--underlay-field-padding-block) var(--underlay-field-padding-inline);
    border-radius: var(--underlay-radius-sm);
    border: none;
    background: var(--underlay-color-field-bg);
    color: var(--underlay-color-text);
    font-size: calc(1em * var(--underlay-font-scale-xs));
  }

  .nightfire-field-single__toolbar select:focus,
  .nightfire-field-single__toolbar select:focus-visible {
    outline: var(--underlay-focus-ring-width) solid var(--underlay-color-primary);
    outline-offset: var(--underlay-focus-ring-offset);
  }

  .nightfire-field-multi__toolbar {
    display: flex;
    align-items: center;
    gap: var(--underlay-space-2);
  }

  .nightfire-field-multi__toolbar select {
    padding: var(--underlay-field-padding-block) var(--underlay-field-padding-inline);
    border-radius: var(--underlay-radius-sm);
    border: none;
    background: var(--underlay-color-field-bg);
    color: var(--underlay-color-text);
    font-size: calc(1em * var(--underlay-font-scale-xs));
  }

  .nightfire-field-multi__toolbar select:focus,
  .nightfire-field-multi__toolbar select:focus-visible {
    outline: var(--underlay-focus-ring-width) solid var(--underlay-color-primary);
    outline-offset: var(--underlay-focus-ring-offset);
  }

  .nightfire-field-multi__item {
    border-radius: var(--underlay-radius-control);
    display: grid;
    gap: var(--underlay-space-2);
  }

  .nightfire-field-multi__controls {
    display: flex;
    align-items: center;
    gap: var(--underlay-space-1);
  }

  .nightfire-field-multi__icon-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.75rem;
    height: 1.75rem;
    padding: 0;
    border-radius: var(--underlay-radius-sm);
    border: 1px solid var(--underlay-color-border-subtle);
    background: transparent;
    color: var(--underlay-color-text-muted);
    cursor: pointer;
  }

  .nightfire-field-multi__icon-btn:hover:not([disabled]) {
    background: var(--underlay-color-field-bg);
    color: var(--underlay-color-text);
  }

  .nightfire-field-multi__icon-btn[disabled] {
    opacity: 0.35;
    cursor: default;
  }

  .nightfire-field-multi__icon-btn--danger:hover:not([disabled]) {
    background: rgba(239, 68, 68, 0.15);
    color: #f87171;
    border-color: rgba(239, 68, 68, 0.4);
  }

  .nightfire-field-multi__icon-btn svg {
    display: block;
  }

  .nightfire-field-multi__add {
    font-size: calc(1em * var(--underlay-font-scale-xxs));
    padding: var(--underlay-button-chip-padding-block)
      var(--underlay-button-chip-padding-inline);
    border-radius: var(--underlay-radius-pill);
    border: 1px solid var(--underlay-color-border-strong);
    background: transparent;
    color: inherit;
    cursor: pointer;
  }

  .nightfire-layout-warning {
    margin-top: var(--underlay-space-2);
    font-size: calc(1em * var(--underlay-font-scale-xs));
    color: var(--underlay-color-danger);
  }
</style>
