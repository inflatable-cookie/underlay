<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";
  import type { Snippet } from "svelte";
  import { getContext } from "svelte";
  import TextInputAffordances from "./TextInputAffordances.svelte";
  import type { InputValidationStatus } from "./validation-state";
  import {
    FIELD_A11Y_CONTEXT_KEY,
    mergeAriaDescribedBy,
    type FieldA11yContext
  } from "../field/a11y-context";

  interface Props {
    needsWrapper?: boolean;
    fieldId: string;
    type: string;
    value?: string;
    autocomplete?: HTMLInputAttributes["autocomplete"];
    required?: boolean;
    className?: HTMLInputAttributes["class"];
    search?: boolean;
    showValidationIcon?: boolean;
    showClearButton?: boolean;
    showValidationStatus?: boolean;
    validationStatus?: InputValidationStatus;
    validationMessage?: string;
    prefix?: string;
    suffix?: Snippet;
    inputRef?: HTMLInputElement | null;
    onInput: (event: Event) => void;
    onChange: () => void;
    onClear: () => void;
    restProps?: Omit<HTMLInputAttributes, "value" | "oninput" | "onchange" | "type">;
  }

  let {
    needsWrapper = false,
    fieldId,
    type,
    value = $bindable(""),
    autocomplete = "off",
    required = false,
    className = "",
    search = false,
    showValidationIcon = false,
    showClearButton = false,
    showValidationStatus = false,
    validationStatus = "idle",
    validationMessage = "",
    prefix,
    suffix,
    inputRef = $bindable(null),
    onInput,
    onChange,
    onClear,
    restProps = {}
  }: Props = $props();

  const fieldA11y = getContext<FieldA11yContext | undefined>(FIELD_A11Y_CONTEXT_KEY);
  const validationMessageId = $derived(
    validationMessage && showValidationStatus ? `${fieldId}-validation-message` : undefined
  );
  const hasInvalidValidation = $derived(showValidationStatus && validationStatus === "invalid");

  const matchesField = $derived(fieldA11y?.matchesControl(fieldId) ?? false);
  const hasFieldError = $derived(matchesField && (fieldA11y?.hasError() ?? false));
  const fieldErrorId = $derived(matchesField ? fieldA11y?.errorId() : undefined);

  const externalAriaInvalid = $derived(restProps["aria-invalid"]);
  const externalAriaDescribedBy = $derived(restProps["aria-describedby"]);
  const externalAriaErrorMessage = $derived(restProps["aria-errormessage"]);

  const ariaInvalid = $derived(
    externalAriaInvalid ?? (hasFieldError || hasInvalidValidation ? true : undefined)
  );
  const ariaDescribedBy = $derived(
    mergeAriaDescribedBy(
      typeof externalAriaDescribedBy === "string" ? externalAriaDescribedBy : undefined,
      fieldErrorId,
      validationMessageId,
    )
  );
  const ariaErrorMessage = $derived(
    (typeof externalAriaErrorMessage === "string" ? externalAriaErrorMessage : undefined) ??
      (ariaInvalid ? fieldErrorId ?? validationMessageId : undefined)
  );
</script>

{#if needsWrapper}
  <div class="underlay-input-wrapper {prefix ? 'underlay-input-wrapper--prefixed' : ''} {suffix ? 'underlay-input-wrapper--suffixed' : ''}">
    {#if prefix}
      <span class="underlay-input-prefix">{prefix}</span>
    {/if}
    <input
      {...restProps}
      id={fieldId}
      class={`underlay-input ${prefix ? "underlay-input--prefixed" : ""} ${search ? "underlay-input--search" : ""} ${showValidationIcon ? "underlay-input--validated" : ""} ${className}`}
      {type}
      {autocomplete}
      {required}
      aria-invalid={ariaInvalid}
      aria-describedby={ariaDescribedBy}
      aria-errormessage={ariaErrorMessage}
      bind:this={inputRef}
      bind:value
      oninput={onInput}
      onchange={onChange}
    />
    <TextInputAffordances
      {showClearButton}
      {showValidationIcon}
      {showValidationStatus}
      showValidationMessage={false}
      {validationStatus}
      {validationMessage}
      onClear={onClear}
    />
    {#if suffix}
      {@render suffix()}
    {/if}
  </div>

  {#if validationMessage && showValidationStatus}
    <p
      id={validationMessageId}
      class="underlay-input-validation__message underlay-input-validation__message--{validationStatus}"
    >
      {validationMessage}
    </p>
  {/if}
{:else}
  <input
    {...restProps}
    id={fieldId}
    class={`underlay-input ${className}`}
    {type}
    {autocomplete}
    {required}
    aria-invalid={ariaInvalid}
    aria-describedby={ariaDescribedBy}
    aria-errormessage={ariaErrorMessage}
    bind:this={inputRef}
    bind:value
    oninput={onInput}
    onchange={onChange}
  />
{/if}

<style>
  .underlay-input-wrapper {
    position: relative;
    width: 100%;
  }

  .underlay-input {
    width: 100%;
    box-sizing: border-box;
    padding: 0.55em 0.7em;
    border-radius: 0.35rem;
    border: none;
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
    color: var(--underlay-color-text, #e5e7eb);
    font-size: 0.85rem;
  }

  .underlay-input[type="date"],
  .underlay-input[type="datetime-local"],
  .underlay-input[type="time"] {
    color-scheme: dark;
    font-family: inherit;
    font-size: 0.85rem;
    line-height: 1;
    height: 2.2em;
    padding: 0 0.7em;
    background-color: var(
      --underlay-color-field-bg,
      rgba(148, 163, 184, 0.18)
    ) !important;
  }

  .underlay-input[type="date"]::-webkit-calendar-picker-indicator,
  .underlay-input[type="datetime-local"]::-webkit-calendar-picker-indicator,
  .underlay-input[type="time"]::-webkit-calendar-picker-indicator {
    filter: invert(0.7);
    cursor: pointer;
    opacity: 0.7;
  }

  .underlay-input[type="date"]::-webkit-calendar-picker-indicator:hover,
  .underlay-input[type="datetime-local"]::-webkit-calendar-picker-indicator:hover,
  .underlay-input[type="time"]::-webkit-calendar-picker-indicator:hover {
    opacity: 1;
  }

  .underlay-input--search {
    padding-right: 2.2em;
  }

  .underlay-input--validated {
    padding-right: 2.5rem;
  }

  .underlay-input:focus,
  .underlay-input:focus-visible {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--underlay-focus-ring-offset, 2px));
  }

  .underlay-input-wrapper--prefixed {
    display: flex;
    align-items: stretch;
    border-radius: var(--underlay-radius-sm, 0.35rem);
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.08));
  }

  .underlay-input-validation__message {
    margin: var(--underlay-space-2, 0.5rem) 0 0;
    font-size: var(--underlay-font-size-sm, 0.8rem);
  }

  .underlay-input-validation__message--validating {
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .underlay-input-validation__message--valid {
    color: var(--underlay-color-success, #22c55e);
  }

  .underlay-input-validation__message--invalid {
    color: var(--underlay-color-error, #ef4444);
  }

  .underlay-input-wrapper--prefixed:focus-within {
    outline: var(--underlay-focus-ring-width, 2px) solid var(--underlay-color-primary, #2563eb);
    outline-offset: var(--underlay-focus-ring-offset, 2px);
  }

  .underlay-input-prefix {
    display: flex;
    align-items: center;
    padding: 0 0.6em;
    font-family: var(--underlay-font-mono, monospace);
    font-size: 0.9em;
    font-size-adjust: var(--underlay-font-mono-size-adjust, 0.52);
    font-weight: 600;
    color: var(--underlay-color-text-muted, #9ca3af);
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.08));
    border-radius: var(--underlay-radius-sm, 0.35rem) 0 0 var(--underlay-radius-sm, 0.35rem);
    user-select: none;
    white-space: nowrap;
  }

  .underlay-input--prefixed {
    flex: 1;
    min-width: 0;
    border: none !important;
    border-radius: 0 var(--underlay-radius-sm, 0.35rem) var(--underlay-radius-sm, 0.35rem) 0 !important;
    background: transparent !important;
  }

  .underlay-input--prefixed:focus,
  .underlay-input--prefixed:focus-visible {
    outline: none !important;
  }

  .underlay-input:read-only {
    color: var(--underlay-color-text-muted, #9ca3af);
    cursor: default;
  }

  .underlay-input:disabled {
    color: var(--underlay-color-text-disabled, rgba(156, 163, 175, 0.5));
    cursor: not-allowed;
    opacity: 0.6;
  }
</style>
