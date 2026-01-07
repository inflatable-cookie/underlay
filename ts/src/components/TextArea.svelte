<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let value: string = "";

  const dispatch = createEventDispatcher<{ input: string; change: string }>();

  function handleInput(event: Event) {
    const target = event.currentTarget as HTMLTextAreaElement | null;
    const next = target ? target.value : value;
    value = next;
    dispatch("input", next);
  }

  function handleChange() {
    dispatch("change", value);
  }
</script>

<textarea
  {...$$restProps}
  class={`underlay-textarea ${$$restProps.class ?? ""}`}
  bind:value
  on:input={handleInput}
  on:change={handleChange}
></textarea>

<style>
  .underlay-textarea {
    width: 100%;
    box-sizing: border-box;
    padding: var(--underlay-field-padding-block, var(--froyo-field-padding-block, 0.55em))
      var(--underlay-field-padding-inline, var(--froyo-field-padding-inline, 0.7em));
    border-radius: var(--underlay-radius-sm, var(--froyo-radius-sm, 0.35rem));
    border: none;
    background: var(
      --underlay-color-field-bg,
      var(--froyo-color-field-bg, rgba(148, 163, 184, 0.18))
    );
    color: var(--underlay-color-text, var(--froyo-color-text, inherit));
    font-size: var(--underlay-font-size-md, var(--froyo-font-size-md, 0.85rem));
    resize: vertical;
  }

  .underlay-textarea:focus,
  .underlay-textarea:focus-visible {
    outline: var(--underlay-focus-ring-width, var(--froyo-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--froyo-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--froyo-focus-ring-offset, 2px));
  }
</style>
