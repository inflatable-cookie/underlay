<script lang="ts">
  import type { MarkdownEditorContext } from "../markup/markdown-editor-context";
  import type { NightfireBlockDefinition, NightfireTypeOption } from "../utils";
  import type { GroupedOptions } from "./grouped-options";
  import NightfireBlockEditor from "../NightfireBlockEditor.svelte";
  import NightfireTypeSelect from "./NightfireTypeSelect.svelte";

  interface Props {
    block: any;
    index: number;
    totalBlocks: number;
    editorSchema: string;
    effectiveDef: NightfireBlockDefinition;
    editorTypeOptions: NightfireTypeOption[];
    groupedOptions: GroupedOptions[] | null;
    onTypeChange: (index: number, value: string) => void;
    onMove: (from: number, to: number) => void;
    onRemove: (index: number) => void;
    onBlockChange: (index: number, next: any) => void;
    onBlockContextChange?: (index: number, context: MarkdownEditorContext) => void;
  }

  let {
    block,
    index,
    totalBlocks,
    editorSchema,
    effectiveDef,
    editorTypeOptions,
    groupedOptions,
    onTypeChange,
    onMove,
    onRemove,
    onBlockChange,
    onBlockContextChange = () => {}
  }: Props = $props();
</script>

<div
  class="underlay-nightfire-field__multi-item"
  data-nightfire-block-card
  data-block-index={index}
>
  <div class="underlay-nightfire-field__multi-toolbar">
    <NightfireTypeSelect
      value={(block as any)?.type ??
        editorTypeOptions[0]?.type ??
        effectiveDef.defaultType}
      onChange={(value) => onTypeChange(index, value)}
      {groupedOptions}
      typeOptions={editorTypeOptions}
    />
    <div class="underlay-nightfire-field__multi-controls">
      <button
        type="button"
        class="underlay-nightfire-field__multi-icon-btn"
        onclick={() => onMove(index, index - 1)}
        disabled={index === 0}
        aria-label="Move block up"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 15-6-6-6 6"/></svg>
      </button>
      <button
        type="button"
        class="underlay-nightfire-field__multi-icon-btn"
        onclick={() => onMove(index, index + 1)}
        disabled={index === totalBlocks - 1}
        aria-label="Move block down"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>
      </button>
      <button
        type="button"
        class="underlay-nightfire-field__multi-icon-btn underlay-nightfire-field__multi-icon-btn--danger"
        onclick={() => onRemove(index)}
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
    onChange={(next) => onBlockChange(index, next)}
    onContextChange={(context) => onBlockContextChange(index, context)}
  />
</div>

<style>
  .underlay-nightfire-field__multi-toolbar {
    display: flex;
    align-items: center;
    gap: var(--underlay-space-2);
    padding-bottom: var(--underlay-space-2);
    margin-bottom: var(--underlay-space-1);
    border-bottom: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.35));
  }

  .underlay-nightfire-field__multi-item {
    border-radius: var(--underlay-radius-control);
    display: grid;
    gap: var(--underlay-space-2);
  }

  .underlay-nightfire-field__multi-controls {
    display: flex;
    align-items: center;
    gap: var(--underlay-space-1);
  }

  .underlay-nightfire-field__multi-icon-btn {
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

  .underlay-nightfire-field__multi-icon-btn:hover:not([disabled]) {
    background: var(--underlay-color-field-bg);
    color: var(--underlay-color-text);
  }

  .underlay-nightfire-field__multi-icon-btn[disabled] {
    opacity: 0.35;
    cursor: default;
  }

  .underlay-nightfire-field__multi-icon-btn--danger:hover:not([disabled]) {
    background: rgba(239, 68, 68, 0.15);
    color: #f87171;
    border-color: rgba(239, 68, 68, 0.4);
  }

  .underlay-nightfire-field__multi-icon-btn svg {
    display: block;
  }
</style>
