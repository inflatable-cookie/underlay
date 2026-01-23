<script lang="ts">
  import { setContext, onDestroy } from "svelte";
  import type { Snippet } from "svelte";

  interface Props {
    /** Whether the form is valid (bindable) */
    isValid?: boolean;
    /** Content to wrap */
    children: Snippet;
  }

  let { isValid = $bindable(true), children }: Props = $props();

  type FieldState = {
    id: string;
    required: boolean;
    hasValue: boolean;
    validationStatus: string;
    isValidationValid: boolean;
  };

  let fields = $state<Map<string, FieldState>>(new Map());

  // Compute overall form validity
  const computedIsValid = $derived.by(() => {
    console.log('[FormValidationProvider] Computing validity, fields:', Array.from(fields.entries()));
    for (const field of fields.values()) {
      // Required field must have a value
      if (field.required && !field.hasValue) {
        console.log('[FormValidationProvider] INVALID: Required field missing value:', field.id);
        return false;
      }

      // If field has async validation
      if (field.validationStatus !== "idle") {
        // Can't be validating
        if (field.validationStatus === "validating") {
          console.log('[FormValidationProvider] INVALID: Field is validating:', field.id);
          return false;
        }
        // Must be valid
        if (!field.isValidationValid) {
          console.log('[FormValidationProvider] INVALID: Field validation failed:', field.id);
          return false;
        }
      }
    }
    console.log('[FormValidationProvider] VALID: All fields passed');
    return true;
  });

  // Sync computed validity to bindable prop
  $effect(() => {
    isValid = computedIsValid;
  });

  // Context API for fields to register themselves
  const registerField = (
    id: string,
    required: boolean,
    hasValue: boolean,
    validationStatus: string,
    isValidationValid: boolean
  ) => {
    console.log('[FormValidationProvider] registerField:', { id, required, hasValue, validationStatus, isValidationValid });
    fields.set(id, {
      id,
      required,
      hasValue,
      validationStatus,
      isValidationValid,
    });
    // Create new Map to trigger reactivity
    fields = new Map(fields);
  };

  const unregisterField = (id: string) => {
    console.log('[FormValidationProvider] unregisterField:', id);
    fields.delete(id);
    // Create new Map to trigger reactivity
    fields = new Map(fields);
  };

  const updateField = (
    id: string,
    hasValue: boolean,
    validationStatus?: string,
    isValidationValid?: boolean
  ) => {
    const field = fields.get(id);
    console.log('[FormValidationProvider] updateField:', { id, hasValue, validationStatus, isValidationValid, fieldExists: !!field });
    if (field) {
      // Create new field object instead of mutating to trigger reactivity
      fields.set(id, {
        id: field.id,
        required: field.required,
        hasValue,
        validationStatus: validationStatus !== undefined ? validationStatus : field.validationStatus,
        isValidationValid: isValidationValid !== undefined ? isValidationValid : field.isValidationValid,
      });
      // Create new Map to trigger reactivity
      fields = new Map(fields);
    }
  };

  setContext("formValidation", {
    registerField,
    unregisterField,
    updateField,
  });
</script>

{@render children()}
