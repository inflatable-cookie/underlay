<script lang="ts">
  import Field from "../../components/Field.svelte";
  import MarkdownEditor from "../../components/MarkdownEditor.svelte";
  import TextInput from "../../components/TextInput.svelte";

  type SummaryImageSliderBlock = {
    type: string;
    version?: string;
    hash?: string;
    data?: {
      subTitle?: string | null;
      description?: string | null;
      image1Id?: string | null;
      image1Alt?: string | null;
      image2Id?: string | null;
      image2Alt?: string | null;
      startPosition?: string;
    };
  };

  export let block: SummaryImageSliderBlock;
  export let onChange: (block: SummaryImageSliderBlock) => void;

  function ensureBlock(
    b: SummaryImageSliderBlock | undefined
  ): SummaryImageSliderBlock {
    const base: SummaryImageSliderBlock =
      b ??
      ({
        type: "summary.imageSlider",
        version: "initial",
        hash: "",
        data: {
          subTitle: null,
          description: null,
          image1Id: null,
          image1Alt: null,
          image2Id: null,
          image2Alt: null,
          startPosition: "left"
        }
      } as SummaryImageSliderBlock);

    if (!base.data) {
      base.data = {
        subTitle: null,
        description: null,
        image1Id: null,
        image1Alt: null,
        image2Id: null,
        image2Alt: null,
        startPosition: "left"
      };
    }

    return base;
  }

  $: block = ensureBlock(block);
  $: onChange?.(block);

  $: hasAtLeastOneImage =
    !!(block.data?.image1Id && block.data.image1Id.trim()) ||
    !!(block.data?.image2Id && block.data.image2Id.trim());

  function setField<K extends keyof NonNullable<SummaryImageSliderBlock["data"]>>(
    key: K,
    value: NonNullable<SummaryImageSliderBlock["data"]>[K]
  ) {
    block = {
      ...block,
      data: {
        ...(block.data ?? {}),
        [key]: value
      }
    };
  }
</script>

<div class="summary-slider-editor">
  <p class="hint">
    Configure a two-image slider. Provide image IDs from the Content Library,
    optional alt text, and a short markdown description.
  </p>

  <div class="field-group">
    <Field label="Subtitle">
      <TextInput
        type="text"
        placeholder="Optional subtitle shown above the slider"
        value={block.data?.subTitle ?? ""}
        on:input={(event: Event) =>
          setField(
            "subTitle",
            (event.currentTarget as HTMLInputElement).value || null
          )}
      />
    </Field>
  </div>

  <div class="field-group field-group--images">
    <div class="image-column">
      <h3>Primary image</h3>
      <Field label="Image ID">
        <TextInput
          type="text"
          placeholder="Primary image ID"
          value={block.data?.image1Id ?? ""}
          on:input={(event: Event) =>
            setField(
              "image1Id",
              (event.currentTarget as HTMLInputElement).value || null
            )}
        />
      </Field>
      <Field label="Alt text">
        <TextInput
          type="text"
          placeholder="Optional alt text for the primary image"
          value={block.data?.image1Alt ?? ""}
          on:input={(event: Event) =>
            setField(
              "image1Alt",
              (event.currentTarget as HTMLInputElement).value || null
            )}
        />
      </Field>
    </div>

    <div class="image-column">
      <h3>Secondary image</h3>
      <Field label="Image ID">
        <TextInput
          type="text"
          placeholder="Secondary image ID"
          value={block.data?.image2Id ?? ""}
          on:input={(event: Event) =>
            setField(
              "image2Id",
              (event.currentTarget as HTMLInputElement).value || null
            )}
        />
      </Field>
      <Field label="Alt text">
        <TextInput
          type="text"
          placeholder="Optional alt text for the secondary image"
          value={block.data?.image2Alt ?? ""}
          on:input={(event: Event) =>
            setField(
              "image2Alt",
              (event.currentTarget as HTMLInputElement).value || null
            )}
        />
      </Field>
    </div>
  </div>

  <div class="field-group">
    <Field label="Start position" hint="e.g. left or right">
      <TextInput
        type="text"
        placeholder="e.g. left or right"
        value={block.data?.startPosition ?? "left"}
        on:input={(event: Event) =>
          setField(
            "startPosition",
            (event.currentTarget as HTMLInputElement).value || "left"
          )}
      />
    </Field>
  </div>

  <div class="field-group">
    <Field label="Description" hint="Markdown description shown beneath the slider">
      <MarkdownEditor
        placeholder="Markdown description shown beneath the slider"
        value={block.data?.description ?? ""}
        onChange={(text: string) =>
          setField(
            "description",
            (text ?? "").length > 0 ? text : null
          )}
      />
    </Field>
  </div>

  {#if !hasAtLeastOneImage}
    <p class="hint hint--warning">
      At least one image ID should be provided for this slider.
    </p>
  {/if}
</div>

<style>
  .summary-slider-editor {
    display: grid;
    gap: var(--froyo-space-3);
  }

  .hint {
    font-size: var(--froyo-font-size-sm);
    opacity: var(--froyo-summary-placeholder-opacity);
  }

  .hint--warning {
    color: var(--froyo-color-danger);
  }

  .field-group {
    display: grid;
    gap: var(--froyo-space-5);
  }

  .field-group--images {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .image-column {
    display: flex;
    flex-direction: column;
    gap: var(--froyo-space-5);
  }

  .image-column h3 {
    font-size: var(--froyo-font-size-md);
    font-weight: 600;
    margin: 0;
  }

  @media (max-width: 768px) {
    .field-group--images {
      grid-template-columns: minmax(0, 1fr);
    }
  }
</style>
