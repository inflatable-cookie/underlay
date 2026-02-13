<script lang="ts">
  import type { HTMLTextareaAttributes } from "svelte/elements";
  import { getContext } from "svelte";
  import {
    FIELD_A11Y_CONTEXT_KEY,
    mergeAriaDescribedBy,
    type FieldA11yContext
  } from "./field/a11y-context";

  interface Props extends Omit<HTMLTextareaAttributes, "value" | "oninput" | "onchange"> {
    value?: string;
    oninput?: (value: string) => void;
    onchange?: (value: string) => void;
  }

  let {
    value = $bindable(""),
    oninput,
    onchange,
    id,
    class: className,
    ...restProps
  }: Props = $props();

  const fieldA11y = getContext<FieldA11yContext | undefined>(FIELD_A11Y_CONTEXT_KEY);
  const controlId = $derived(id ?? fieldA11y?.controlId());
  const matchesField = $derived(fieldA11y?.matchesControl(controlId) ?? false);
  const hasFieldError = $derived(matchesField && (fieldA11y?.hasError() ?? false));
  const fieldErrorId = $derived(matchesField ? fieldA11y?.errorId() : undefined);

  const externalAriaInvalid = $derived(restProps["aria-invalid"]);
  const externalAriaDescribedBy = $derived(restProps["aria-describedby"]);
  const externalAriaErrorMessage = $derived(restProps["aria-errormessage"]);

  const ariaInvalid = $derived(externalAriaInvalid ?? (hasFieldError ? true : undefined));
  const ariaDescribedBy = $derived(
    mergeAriaDescribedBy(
      typeof externalAriaDescribedBy === "string" ? externalAriaDescribedBy : undefined,
      fieldErrorId,
    )
  );
  const ariaErrorMessage = $derived(
    (typeof externalAriaErrorMessage === "string" ? externalAriaErrorMessage : undefined) ??
      (hasFieldError ? fieldErrorId : undefined)
  );

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
  id={controlId}
  class={`underlay-textarea ${className ?? ""}`}
  aria-invalid={ariaInvalid}
  aria-describedby={ariaDescribedBy}
  aria-errormessage={ariaErrorMessage}
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

  /* Readonly state - slightly dimmed */
  .underlay-textarea:read-only {
    color: var(--underlay-color-text-muted, #9ca3af);
    cursor: default;
  }

  /* Disabled state - more dimmed */
  .underlay-textarea:disabled {
    color: var(--underlay-color-text-disabled, rgba(156, 163, 175, 0.5));
    cursor: not-allowed;
    opacity: 0.6;
  }
</style>
