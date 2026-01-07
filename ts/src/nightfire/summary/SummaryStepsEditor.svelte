<script lang="ts">
  import Field from "../../components/Field.svelte";
  import MarkdownEditor from "../../components/MarkdownEditor.svelte";
  import TextInput from "../../components/TextInput.svelte";

  type Step = {
    title?: string | null;
    body?: string | null;
  };

  type SummaryStepsBlock = {
    type: string;
    version?: string;
    hash?: string;
    data?: {
      subTitle?: string | null;
      pages?: Step[];
    };
  };

  export let block: SummaryStepsBlock;
  export let onChange: (block: SummaryStepsBlock) => void;

  const MAX_STEPS = 5;

  function ensureBlock(b: SummaryStepsBlock | undefined): SummaryStepsBlock {
    const base: SummaryStepsBlock =
      b ??
      ({
        type: "summary.steps",
        version: "initial",
        hash: "",
        data: {
          subTitle: null,
          pages: []
        }
      } as SummaryStepsBlock);

    if (!base.data) {
      base.data = {
        subTitle: null,
        pages: []
      };
    }
    if (!Array.isArray(base.data.pages)) {
      base.data.pages = [];
    }

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

  function addStep() {
    const pages = block.data?.pages ?? [];
    if (pages.length >= MAX_STEPS) return;

    block = {
      ...block,
      data: {
        ...block.data,
        pages: [
          ...pages,
          {
            title: "",
            body: ""
          }
        ]
      }
    };
  }

  function removeStep(index: number) {
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

  function moveStep(from: number, to: number) {
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

  function updateStepTitle(index: number, title: string) {
    const pages = [...(block.data?.pages ?? [])];
    const step = pages[index] ?? {};
    pages[index] = { ...step, title };

    block = {
      ...block,
      data: {
        ...block.data,
        pages
      }
    };
  }

  function updateStepBody(index: number, text: string) {
    const pages = [...(block.data?.pages ?? [])];
    const step = pages[index] ?? {};
    pages[index] = { ...step, body: text };

    block = {
      ...block,
      data: {
        ...block.data,
        pages
      }
    };
  }

  function getStepBodyText(step: Step | undefined): string {
    return typeof step?.body === "string" ? step.body : "";
  }
</script>

<div class="summary-steps-editor">
  <p class="hint">
    Configure a sequence of steps with optional titles and markdown bodies. Up
    to {MAX_STEPS} steps are supported.
  </p>

  <Field label="Subtitle">
    <TextInput
      type="text"
      placeholder="Optional subtitle shown above the steps"
      value={block.data?.subTitle ?? ""}
      on:input={(event: Event) =>
        setSubTitle((event.currentTarget as HTMLInputElement).value)}
    />
  </Field>

  <div class="steps">
    {#if !block.data?.pages || block.data.pages.length === 0}
      <p>No steps defined yet.</p>
    {/if}

    {#each block.data?.pages ?? [] as step, index}
      <div class="step-row">
        <div class="step-meta">
          <span class="step-index">Step {index + 1}</span>
          <div class="step-controls">
            <button
              type="button"
              class="move"
              on:click={() => moveStep(index, index - 1)}
              disabled={index === 0}
            >
              ↑
            </button>
            <button
              type="button"
              class="move"
              on:click={() => moveStep(index, index + 1)}
              disabled={index === (block.data?.pages?.length ?? 0) - 1}
            >
              ↓
            </button>
          </div>
          <button
            type="button"
            class="remove"
            on:click={() => removeStep(index)}
          >
            Remove
          </button>
        </div>
        <div class="step-fields">
          <TextInput
            type="text"
            placeholder="Step title (optional)"
            value={step.title ?? ""}
            on:input={(event: Event) =>
              updateStepTitle(
                index,
                (event.currentTarget as HTMLInputElement).value
              )}
          />
          <MarkdownEditor
            placeholder="Step body (markdown)"
            value={getStepBodyText(step)}
            onChange={(text: string) => updateStepBody(index, text)}
          />
        </div>
      </div>
    {/each}
  </div>

  <button
    type="button"
    class="add"
    on:click={addStep}
    disabled={(block.data?.pages?.length ?? 0) >= MAX_STEPS}
  >
    + Add step
  </button>
</div>

<style>
  .summary-steps-editor {
    display: grid;
    gap: var(--froyo-space-3);
  }

  .hint {
    font-size: var(--froyo-font-size-sm);
    opacity: var(--froyo-summary-placeholder-opacity);
  }

  .steps {
    display: grid;
    gap: var(--froyo-space-2);
  }

  .step-row {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr);
    gap: var(--froyo-space-2);
  }

  .step-meta {
    display: flex;
    flex-direction: column;
    gap: var(--froyo-space-1);
    align-items: flex-start;
  }

  .step-index {
    font-size: var(--froyo-font-size-sm);
    font-weight: 600;
    opacity: var(--froyo-summary-placeholder-opacity);
  }

  .step-controls {
    display: flex;
    gap: var(--froyo-space-1);
  }

  .move,
  .remove {
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

  .step-fields {
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
