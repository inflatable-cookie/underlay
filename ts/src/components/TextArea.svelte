<script lang="ts">
  import type { HTMLTextareaAttributes } from "svelte/elements";

  interface Props extends Omit<HTMLTextareaAttributes, "value" | "oninput" | "onchange"> {
    value?: string;
    oninput?: (value: string) => void;
    onchange?: (value: string) => void;
  }

  let {
    value = $bindable(""),
    oninput,
    onchange,
    class: className,
    ...restProps
  }: Props = $props();

  function handleInput(event: Event) {
    const target = event.currentTarget as HTMLTextAreaElement | null;
    const next = target ? target.value : value;
    value = next;
    oninput?.(next);
  }

  function handleChange() {
    onchange?.(value);
  }
</script>

<textarea
  {...restProps}
  class={`underlay-textarea ${className ?? ""}`}
  bind:value
  oninput={handleInput}
  onchange={handleChange}
></textarea>

<style>
  .underlay-textarea {
    width: 100%;
    box-sizing: border-box;
    padding: var(--underlay-field-padding-block, var(--underlay-field-padding-block, 0.55em))
      var(--underlay-field-padding-inline, var(--underlay-field-padding-inline, 0.7em));
    border-radius: var(--underlay-radius-sm, var(--underlay-radius-sm, 0.35rem));
    border: none;
    background: var(
      --underlay-color-field-bg,
      var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18))
    );
    color: var(--underlay-color-text, var(--underlay-color-text, inherit));
    font-size: var(--underlay-font-size-md, var(--underlay-font-size-md, 0.85rem));
    resize: vertical;
  }

  .underlay-textarea:focus,
  .underlay-textarea:focus-visible {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--underlay-focus-ring-offset, 2px));
  }
</style>
