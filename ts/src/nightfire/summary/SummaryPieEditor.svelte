<script lang="ts">
  import Field from "../../components/Field.svelte";
  import MarkdownEditor from "../../components/MarkdownEditor.svelte";
  import TextInput from "../../components/TextInput.svelte";

  type Segment = {
    title?: string | null;
    body?: string | null;
  };

  type SummaryPieBlock = {
    type: string;
    version?: string;
    hash?: string;
    data?: {
      subTitle?: string | null;
      pages?: Segment[];
    };
  };

  const MAX_SEGMENTS = 5;

  export let block: SummaryPieBlock;
  export let onChange: (block: SummaryPieBlock) => void;

  function ensureBlock(b: SummaryPieBlock | undefined): SummaryPieBlock {
    const base: SummaryPieBlock =
      b ??
      ({
        type: "summary.pie",
        version: "initial",
        hash: "",
        data: { subTitle: null, pages: [] }
      } as SummaryPieBlock);

    if (!base.data) base.data = { subTitle: null, pages: [] };
    if (!Array.isArray(base.data.pages)) base.data.pages = [];

    return base;
  }

  $: block = ensureBlock(block);
  $: onChange?.(block);

  function setSubTitle(subTitle: string) {
    block = {
      ...block,
      data: {
        ...block.data,
        subTitle: subTitle || null
      }
    };
  }

  function addSegment() {
    const pages = block.data?.pages ?? [];
    if (pages.length >= MAX_SEGMENTS) return;

    block = {
      ...block,
      data: {
        ...block.data,
        pages: [
          ...(block.data?.pages ?? []),
          {
            title: "",
            body: ""
          }
        ]
      }
    };
  }

  function removeSegment(index: number) {
    const pages = [...(block.data?.pages ?? [])];
    pages.splice(index, 1);

    block = {
      ...block,
      data: {
        ...block.data,
        pages
      }
    };
  }

  function moveSegment(from: number, to: number) {
    const segments = [...(block.data?.pages ?? [])];
    if (
      from === to ||
      from < 0 ||
      to < 0 ||
      from >= segments.length ||
      to >= segments.length
    ) {
      return;
    }

    const [moved] = segments.splice(from, 1);
    segments.splice(to, 0, moved);

    block = {
      ...block,
      data: {
        ...block.data,
        pages: segments
      }
    };
  }

  function updateSegmentTitle(index: number, title: string) {
    const pages = [...(block.data?.pages ?? [])];
    const segment = pages[index] ?? {};
    pages[index] = { ...segment, title };

    block = {
      ...block,
      data: {
        ...block.data,
        pages
      }
    };
  }

  function updateSegmentBody(index: number, text: string) {
    const pages = [...(block.data?.pages ?? [])];
    const segment = pages[index] ?? {};
    pages[index] = { ...segment, body: text };

    block = {
      ...block,
      data: {
        ...block.data,
        pages
      }
    };
  }

  function getBodyText(segment: Segment | undefined): string {
    return typeof segment?.body === "string" ? segment.body : "";
  }
</script>

<div class="summary-pie-editor">
  <p class="hint">
    Configure pie segments for this summary. Each segment becomes a slice in
    the rendered chart.
  </p>

  <Field label="Subtitle">
    <TextInput
      type="text"
      placeholder="Optional subtitle for the pie chart"
      value={block.data?.subTitle ?? ""}
      on:input={(event: Event) =>
        setSubTitle((event.currentTarget as HTMLInputElement).value)}
    />
  </Field>

  <div class="segments">
    {#if !block.data?.pages || block.data.pages.length === 0}
      <p>No segments defined yet. At least one segment is recommended.</p>
    {/if}

    {#each block.data?.pages ?? [] as segment, index}
      <div class="segment-row">
        <div class="segment-meta">
          <span class="segment-index">Segment {index + 1}</span>
          <div class="segment-controls">
            <button
              type="button"
              class="move"
              on:click={() => moveSegment(index, index - 1)}
              disabled={index === 0}
            >
              ↑
            </button>
            <button
              type="button"
              class="move"
              on:click={() => moveSegment(index, index + 1)}
              disabled={index === (block.data?.pages?.length ?? 0) - 1}
            >
              ↓
            </button>
          </div>
          <button
            type="button"
            class="remove"
            on:click={() => removeSegment(index)}
          >
            Remove
          </button>
        </div>
        <div class="segment-fields">
          <TextInput
            type="text"
            placeholder="Segment title (optional)"
            value={segment.title ?? ""}
            on:input={(event: Event) =>
              updateSegmentTitle(
                index,
                (event.currentTarget as HTMLInputElement).value
              )}
          />
          <MarkdownEditor
            placeholder="Segment body (markdown)"
            value={getBodyText(segment)}
            onChange={(text: string) => updateSegmentBody(index, text)}
          />
        </div>
      </div>
    {/each}
  </div>

  <button type="button" class="add" on:click={addSegment}>
    + Add segment
  </button>
</div>

<style>
  .summary-pie-editor {
    display: grid;
    gap: var(--froyo-space-3);
  }

  .hint {
    font-size: var(--froyo-font-size-sm);
    opacity: var(--froyo-summary-placeholder-opacity);
  }

  .segments {
    display: grid;
    gap: var(--froyo-space-2);
  }

  .segment-row {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr);
    gap: var(--froyo-space-2);
  }

  .segment-meta {
    display: flex;
    flex-direction: column;
    gap: var(--froyo-space-1);
    align-items: flex-start;
  }

  .segment-index {
    font-size: var(--froyo-font-size-sm);
    font-weight: 600;
    opacity: var(--froyo-summary-placeholder-opacity);
  }

  .remove {
    font-size: var(--froyo-font-size-xs);
    border: none;
    background: transparent;
    color: var(--froyo-color-text-muted);
    cursor: pointer;
  }

  .segment-controls {
    display: flex;
    gap: var(--froyo-space-1);
  }

  .move {
    font-size: var(--froyo-font-size-xs);
    border: none;
    background: transparent;
    color: var(--froyo-color-text-muted);
    cursor: pointer;
  }

  .move[disabled] {
    opacity: 0.5;
    cursor: default;
  }

  .segment-fields {
    display: flex;
    flex-direction: column;
    gap: var(--froyo-space-1);
  }

  .add {
    font-size: var(--froyo-font-size-sm);
    border-radius: var(--froyo-radius-pill);
    border: 1px dashed var(--froyo-color-border-strong);
    padding: var(--froyo-space-1) var(--froyo-space-3);
    background: transparent;
    color: inherit;
    cursor: pointer;
    width: fit-content;
  }
</style>
