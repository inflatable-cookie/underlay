<script lang="ts">
  import MarkdownEditor from "../../components/MarkdownEditor.svelte";
  import TextInput from "../../components/TextInput.svelte";

  type Page = {
    title?: string | null;
    image_id?: string | null;
    body?: string | null;
  };

  type SummaryDiagramBlock = {
    type: string;
    version?: string;
    hash?: string;
    data?: {
      pages?: Page[];
    };
  };

  export let block: SummaryDiagramBlock;
  export let onChange: (block: SummaryDiagramBlock) => void;

  function ensureBlock(b: SummaryDiagramBlock | undefined): SummaryDiagramBlock {
    const base: SummaryDiagramBlock =
      b ??
      ({
        type: "summary.diagram",
        version: "initial",
        hash: "",
        data: { pages: [] }
      } as SummaryDiagramBlock);

    if (!base.data) base.data = { pages: [] };
    if (!Array.isArray(base.data.pages)) base.data.pages = [];

    return base;
  }

  $: block = ensureBlock(block);
  $: onChange?.(block);

  function addPage() {
    block = {
      ...block,
      data: {
        ...block.data,
        pages: [
          ...(block.data?.pages ?? []),
          {
            title: "",
            image_id: "",
            body: ""
          }
        ]
      }
    };
  }

  function removePage(index: number) {
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

  function movePage(from: number, to: number) {
    const pages = [...(block.data?.pages ?? [])];
    if (
      from === to ||
      from < 0 ||
      to < 0 ||
      from >= pages.length ||
      to >= pages.length
    ) {
      return;
    }

    const [moved] = pages.splice(from, 1);
    pages.splice(to, 0, moved);

    block = {
      ...block,
      data: {
        ...block.data,
        pages
      }
    };
  }

  function updatePageTitle(index: number, title: string) {
    const pages = [...(block.data?.pages ?? [])];
    const page = pages[index] ?? {};
    pages[index] = { ...page, title };

    block = {
      ...block,
      data: {
        ...block.data,
        pages
      }
    };
  }

  function updatePageImageId(index: number, imageId: string) {
    const pages = [...(block.data?.pages ?? [])];
    const page = pages[index] ?? {};
    pages[index] = { ...page, image_id: imageId || null };

    block = {
      ...block,
      data: {
        ...block.data,
        pages
      }
    };
  }

  function updatePageBody(index: number, text: string) {
    const pages = [...(block.data?.pages ?? [])];
    const page = pages[index] ?? {};
    pages[index] = { ...page, body: text };

    block = {
      ...block,
      data: {
        ...block.data,
        pages
      }
    };
  }

  function getPageBodyText(page: Page | undefined): string {
    return typeof page?.body === "string" ? page.body : "";
  }
</script>

<div class="summary-diagram-editor">
  <p class="hint">
    Configure diagram pages. Each page can reference an image and optional
    markdown body text.
  </p>

  <div class="pages">
    {#if !block.data?.pages || block.data.pages.length === 0}
      <p>No diagram pages defined yet.</p>
    {/if}

    {#each block.data?.pages ?? [] as page, index}
      <div class="page-row">
        <div class="page-meta">
          <span class="page-index">Page {index + 1}</span>
          <div class="page-controls">
            <button
              type="button"
              class="move"
              on:click={() => movePage(index, index - 1)}
              disabled={index === 0}
            >
              ↑
            </button>
            <button
              type="button"
              class="move"
              on:click={() => movePage(index, index + 1)}
              disabled={index === (block.data?.pages?.length ?? 0) - 1}
            >
              ↓
            </button>
          </div>
          <button
            type="button"
            class="remove"
            on:click={() => removePage(index)}
          >
            Remove
          </button>
        </div>
        <div class="page-fields">
          <TextInput
            type="text"
            placeholder="Page title (optional)"
            value={page.title ?? ""}
            on:input={(event: Event) =>
              updatePageTitle(
                index,
                (event.currentTarget as HTMLInputElement).value
              )}
          />
          <TextInput
            type="text"
            placeholder="Image ID (from Content Library)"
            value={page.image_id ?? ""}
            on:input={(event: Event) =>
              updatePageImageId(
                index,
                (event.currentTarget as HTMLInputElement).value
              )}
          />
          <MarkdownEditor
            placeholder="Page body (markdown)"
            value={getPageBodyText(page)}
            onChange={(text: string) => updatePageBody(index, text)}
          />
        </div>
      </div>
    {/each}
  </div>

  <button type="button" class="add" on:click={addPage}>
    + Add page
  </button>
</div>

<style>
  .summary-diagram-editor {
    display: grid;
    gap: var(--froyo-space-3);
  }

  .hint {
    font-size: var(--froyo-font-size-sm);
    opacity: var(--froyo-summary-placeholder-opacity);
  }

  .pages {
    display: grid;
    gap: var(--froyo-space-2);
  }

  .page-row {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr);
    gap: var(--froyo-space-2);
  }

  .page-meta {
    display: flex;
    flex-direction: column;
    gap: var(--froyo-space-1);
    align-items: flex-start;
  }

  .page-index {
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

  .page-controls {
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

  .page-fields {
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
