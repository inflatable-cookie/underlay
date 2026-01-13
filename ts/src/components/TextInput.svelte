<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";

  interface Props extends Omit<HTMLInputAttributes, "value" | "oninput" | "onchange"> {
    type?: string;
    value?: string;
    autocomplete?: HTMLInputAttributes["autocomplete"];
    inputRef?: HTMLInputElement | null;
    oninput?: (value: string) => void;
    onchange?: (value: string) => void;
  }

  let {
    type = "text",
    value = $bindable(""),
    autocomplete = "off",
    inputRef = $bindable(null),
    oninput,
    onchange,
    class: className,
    ...restProps
  }: Props = $props();

  function handleInput(event: Event) {
    const target = event.currentTarget as HTMLInputElement | null;
    const next = target ? target.value : value;
    value = next;
    oninput?.(next);
  }

  function handleChange() {
    onchange?.(value);
  }
</script>

<input
  {...restProps}
  class={`underlay-input ${className ?? ""}`}
  {type}
  {autocomplete}
  bind:this={inputRef}
  bind:value
  oninput={handleInput}
  onchange={handleChange}
/>

<style>
  .underlay-input {
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
  }

  .underlay-input:focus,
  .underlay-input:focus-visible {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--underlay-focus-ring-offset, 2px));
  }
</style>
