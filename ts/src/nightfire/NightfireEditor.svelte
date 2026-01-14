<script lang="ts">
  import type { NightfireValue } from "./index";
  import NightfireBlockEditor from "./NightfireBlockEditor.svelte";
  import NightfireFieldError from "./NightfireFieldError.svelte";
  import {
    getSchemaDefinition,
    getBlockTypeOptionsForSchema
  } from "./editor-registry";
  import {
    isEmptyNightfire,
    writeNightfireToFormData
  } from "./utils";

  /**
   * Field-level Nightfire editor.
   *
   * - Owns the NightfireValue (single or multi) for a given schema.
   * - Delegates single-block editing to NightfireBlockEditor.
   * - Provides an optional `prepare` hook so forms can serialise the
   *   current value into FormData just before submit.
   */

  type NightfireFieldMode = "single" | "multi";

  interface NightfireBlockOptionInput {
    type: string;
    label: string;
    category?: string;
  }

  interface Props {
    name: string;
    schema: string;
    value: NightfireValue;
    /**
     * Optional overrides derived from a Nightfire strategy.
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
    prepare = $bindable(() => {})
  }: Props = $props();

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
  const SUMMARY_SCHEMA_ID = "acow:content/summary@1";
  const FALLBACK_MARKUP_SCHEMA_ID = "acow:content/markup@1";
  const SUMMARY_TEXT_PAGE_TYPES = new Set([
    "summary.book",
    "summary.circles",
    "summary.pie",
    "summary.steps"
  ]);
  const SUMMARY_IMAGE_PAGE_TYPES = new Set([
    "summary.diagram",
    "summary.slideshow"
  ]);
  const SUMMARY_IMAGE_SLIDER_TYPE = "summary.imageSlider";

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

  // Helper function to resolve schema synchronously - called both at init
  // and when schema prop changes. This ensures we have correct values
  // IMMEDIATELY on first render, not after an $effect runs.
  function resolveSchema(schemaId: string): {
    editorSchema: string;
    registryDef: { schema: string; mode: NightfireFieldMode; defaultType: string };
  } {
    const def = getSchemaDefinition(schemaId);
    if (def) {
      return {
        editorSchema: schemaId,
        registryDef: {
          schema: def.schema,
          mode: def.mode,
          defaultType: def.defaultType
        }
      };
    }

    const fallbackDef = getSchemaDefinition(FALLBACK_MARKUP_SCHEMA_ID);
    if (fallbackDef) {
      return {
        editorSchema: FALLBACK_MARKUP_SCHEMA_ID,
        registryDef: {
          schema: fallbackDef.schema,
          mode: fallbackDef.mode,
          defaultType: fallbackDef.defaultType
        }
      };
    }

    return {
      editorSchema: schemaId,
      registryDef: {
        schema: schemaId,
        mode: "single",
        defaultType: "markdown"
      }
    };
  }

  // Initialize with correct values IMMEDIATELY (not deferred via $effect)
  const initialResolved = resolveSchema(schema);
  let editorSchema: string = $state(initialResolved.editorSchema);
  let registryDef: {
    schema: string;
    mode: NightfireFieldMode;
    defaultType: string;
  } = $state(initialResolved.registryDef);

  // Track the last schema prop to avoid redundant updates
  let lastSchemaProp = schema;

  // Update when schema prop changes (for subsequent navigations)
  $effect(() => {
    // Only update if schema prop actually changed
    if (schema === lastSchemaProp) {
      return;
    }
    lastSchemaProp = schema;

    const resolved = resolveSchema(schema);
    editorSchema = resolved.editorSchema;
    registryDef = resolved.registryDef;
  });

  const effectiveDef = $derived({
    schema: editorSchema,
    mode: (modeOverride ?? registryDef.mode) as NightfireFieldMode,
    defaultType: defaultTypeOverride ?? registryDef.defaultType ?? "markdown"
  });

  const baseTypeOptions = $derived(
    blockOptions && blockOptions.length > 0
      ? blockOptions
      : getBlockTypeOptionsForSchema(editorSchema)
  );

  const editorTypeOptions = $derived(
    baseTypeOptions.map((opt) => ({
      type: opt.type,
      label: opt.label
    }))
  );

  interface GroupedOptions {
    category: string | null;
    label: string;
    options: NightfireBlockOptionInput[];
  }

  function buildGroupedOptions(
    options: NightfireBlockOptionInput[]
  ): GroupedOptions[] {
    const groups = new Map<string | null, NightfireBlockOptionInput[]>();

    for (const opt of options) {
      const key = opt.category ?? null;
      const existing = groups.get(key) ?? [];
      existing.push(opt);
      groups.set(key, existing);
    }

    const sortedKeys = Array.from(groups.keys()).sort((a, b) => {
      const labelA = (a ?? "").toLowerCase();
      const labelB = (b ?? "").toLowerCase();
      if (labelA < labelB) return -1;
      if (labelA > labelB) return 1;
      return 0;
    });

    return sortedKeys.map((key) => {
      const opts = groups.get(key) ?? [];
      opts.sort((a, b) => a.label.localeCompare(b.label));
      return {
        category: key,
        label: key ?? "Other",
        options: opts
      };
    });
  }

  const groupedOptions = $derived(
    blockOptions && blockOptions.some((o) => !!o.category)
      ? buildGroupedOptions(blockOptions)
      : null
  );

  const isMulti = $derived(effectiveDef.mode === "multi" || Array.isArray(value?.blocks));

  // Single-block state view
  const singleBlock = $derived(!isMulti ? ((value?.block as any) ?? null) : null);

  // Multi-block state view
  const blocks = $derived(
    isMulti && Array.isArray(value?.blocks)
      ? (value.blocks as any[]).slice()
      : []
  );

  function emit(nextValue: NightfireValue) {
    value = nextValue;
    onChange?.(nextValue);
  }

  function getLabelForType(type: string): string {
    const match = editorTypeOptions.find((opt) => opt.type === type);
    return match?.label ?? type;
  }

  function clonePagesWithTitleBody(source: any): any[] {
    if (!source || !Array.isArray((source as any).pages)) return [];
    return (source as any).pages.map((page: any) => ({
      title:
        typeof page?.title === "string" && page.title.length > 0
          ? page.title
          : null,
      body:
        typeof page?.body === "string" && page.body.length > 0
          ? page.body
          : null
    }));
  }

  function hasAnyImageIds(source: any): boolean {
    if (!source || !Array.isArray((source as any).pages)) return false;
    return (source as any).pages.some(
      (page: any) =>
        typeof page?.image_id === "string" && page.image_id.trim().length > 0
    );
  }

  function transformSummaryBlockOnLayoutChange(
    currentBlock: any,
    nextType: string
  ): { block: any; warning: string | null } {
    const fromType: string | null =
      currentBlock && typeof currentBlock.type === "string"
        ? (currentBlock.type as string)
        : null;

    if (!fromType || fromType === nextType) {
      return {
        block: {
          ...(currentBlock ?? {}),
          type: nextType
        },
        warning: null
      };
    }

    const fromIsTextPages = SUMMARY_TEXT_PAGE_TYPES.has(fromType);
    const toIsTextPages = SUMMARY_TEXT_PAGE_TYPES.has(nextType);
    const fromIsImagePages = SUMMARY_IMAGE_PAGE_TYPES.has(fromType);
    const toIsImagePages = SUMMARY_IMAGE_PAGE_TYPES.has(nextType);
    const fromIsSlider = fromType === SUMMARY_IMAGE_SLIDER_TYPE;
    const toIsSlider = nextType === SUMMARY_IMAGE_SLIDER_TYPE;

    const fromLabel = getLabelForType(fromType);
    const toLabel = getLabelForType(nextType);

    // Default: keep data as-is; Nightfire block editors will
    // normalise fields.
    let warning: string | null = null;
    let data: any =
      currentBlock && typeof currentBlock.data === "object"
        ? (currentBlock.data as any)
        : {};

    // Text-only page layouts ↔ text-only page layouts.
    if (fromIsTextPages && toIsTextPages) {
      const pages = clonePagesWithTitleBody({ pages: data.pages });
      const subTitle =
        typeof data.subTitle === "string" && data.subTitle.length > 0
          ? data.subTitle
          : null;

      data = {
        ...data,
        pages
      };

      if (
        nextType === "summary.circles" ||
        nextType === "summary.pie" ||
        nextType === "summary.steps"
      ) {
        data.subTitle = subTitle;
      } else {
        delete data.subTitle;
      }

      return {
        block: {
          ...currentBlock,
          type: nextType,
          data
        },
        warning: null
      };
    }

    // Image page layouts ↔ image page layouts: keep pages as-is.
    if (fromIsImagePages && toIsImagePages) {
      return {
        block: {
          ...currentBlock,
          type: nextType
        },
        warning: null
      };
    }

    // Image page → text-only page: keep titles/bodies, drop images.
    if (fromIsImagePages && toIsTextPages) {
      const pages = clonePagesWithTitleBody({ pages: data.pages });
      const hadImages = hasAnyImageIds(data);

      data = {
        ...data,
        pages
      };

      if (
        nextType === "summary.circles" ||
        nextType === "summary.pie" ||
        nextType === "summary.steps"
      ) {
        data.subTitle =
          typeof data.subTitle === "string" && data.subTitle.length > 0
            ? data.subTitle
            : null;
      } else {
        delete data.subTitle;
      }

      if (hadImages) {
        warning = `Changing layout from ${fromLabel} to ${toLabel} keeps titles and bodies but drops image selections.`;
      }

      return {
        block: {
          ...currentBlock,
          type: nextType,
          data
        },
        warning
      };
    }

    // Text-only page → image page: keep titles/bodies; images start empty.
    if (fromIsTextPages && toIsImagePages) {
      const pages = clonePagesWithTitleBody({ pages: data.pages }).map(
        (page) => ({
          ...page,
          image_id: null
        })
      );

      data = {
        ...data,
        pages
      };

      return {
        block: {
          ...currentBlock,
          type: nextType,
          data
        },
        warning: null
      };
    }

    // Any page-based layout → image slider: keep first body as description.
    if ((fromIsTextPages || fromIsImagePages) && toIsSlider) {
      const pagesArray: any[] = Array.isArray(data.pages)
        ? (data.pages as any[])
        : [];
      const first = pagesArray[0] ?? {};
      const description =
        typeof first.body === "string" && first.body.length > 0
          ? first.body
          : "";

      data = {
        subTitle:
          typeof data.subTitle === "string" && data.subTitle.length > 0
            ? data.subTitle
            : null,
        description: description || null,
        image1Id: null,
        image1Alt: null,
        image2Id: null,
        image2Alt: null,
        startPosition: "left"
      };

      if (pagesArray.length > 1 || hasAnyImageIds(currentBlock.data)) {
        warning = `Changing layout from ${fromLabel} to ${toLabel} keeps the first page's text as the slider description but discards other pages and any image selections.`;
      } else {
        warning = `Changing layout from ${fromLabel} to ${toLabel} keeps the first page's text as the slider description.`;
      }

      return {
        block: {
          ...currentBlock,
          type: nextType,
          data
        },
        warning
      };
    }

    // Image slider → page-based layouts: turn description into a single page.
    if (fromIsSlider && (toIsTextPages || toIsImagePages)) {
      const description =
        typeof data.description === "string" && data.description.length > 0
          ? data.description
          : "";

      const page = {
        title: null,
        body: description || null
      };

      data = {
        pages: [page]
      };

      if (toIsImagePages) {
        data.pages = [
          {
            ...page,
            image_id: null
          }
        ];
      }

      const hadImages =
        (typeof data.image1Id === "string" && data.image1Id.length > 0) ||
        (typeof data.image2Id === "string" && data.image2Id.length > 0);

      if (hadImages) {
        warning = `Changing layout from ${fromLabel} to ${toLabel} keeps the description as the first page's body but drops image selections.`;
      } else {
        warning = `Changing layout from ${fromLabel} to ${toLabel} keeps the description as the first page's body.`;
      }

      return {
        block: {
          ...currentBlock,
          type: nextType,
          data
        },
        warning
      };
    }

    // Fallback: keep data and update type; rely on block editors to
    // normalise. No explicit warning.
    return {
      block: {
        ...(currentBlock ?? {}),
        type: nextType
      },
      warning: null
    };
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
      const transformed = transformSummaryBlockOnLayoutChange(current, nextType);
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
    const nextBlocks = blocks.concat({
      type: defaultType,
      version: "initial",
      hash: "",
      data: {}
    });
    emit({
      schema,
      block: undefined,
      blocks: nextBlocks
    });
  }

  function removeBlock(index: number) {
    const nextBlocks = blocks.slice();
    nextBlocks.splice(index, 1);
    emit({
      schema,
      block: undefined,
      blocks: nextBlocks
    });
  }

  function moveBlock(from: number, to: number) {
    if (
      from === to ||
      from < 0 ||
      to < 0 ||
      from >= blocks.length ||
      to >= blocks.length
    ) {
      return;
    }
    const nextBlocks = blocks.slice();
    const [moved] = nextBlocks.splice(from, 1);
    nextBlocks.splice(to, 0, moved);
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
      const defaultBlock = {
        type: effectiveDef.defaultType ?? "markdown",
        version: "initial",
        hash: "",
        data: {}
      };

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
  $effect(() => {
    prepare = (formData: FormData) => {
      writeNightfireToFormData(formData, name, value);
    };
  });
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
      {#if blocks.length === 0}
        <p>No blocks defined yet.</p>
      {/if}

      {#each blocks as block, index}
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
                onclick={() => moveBlock(index, index - 1)}
                disabled={index === 0}
                aria-label="Move block up"
              >
                ↑
              </button>
              <button
                type="button"
                onclick={() => moveBlock(index, index + 1)}
                disabled={index === blocks.length - 1}
                aria-label="Move block down"
              >
                ↓
              </button>
              <button
                type="button"
                onclick={() => removeBlock(index)}
                aria-label="Remove block"
              >
                ✕
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
    align-items: flex-start;
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
    gap: var(--underlay-space-1);
  }

  .nightfire-field-multi__controls button,
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

  .nightfire-field-multi__controls button[disabled] {
    opacity: 0.5;
    cursor: default;
  }

  .nightfire-layout-warning {
    margin-top: var(--underlay-space-2);
    font-size: calc(1em * var(--underlay-font-scale-xs));
    color: var(--underlay-color-danger);
  }
</style>
