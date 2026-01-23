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
  let version = $state(0); // Increment to trigger reactivity

  // Compute overall form validity
  const computedIsValid = $derived.by(() => {
    // Access version to create dependency
    const _v = version;
    for (const field of fields.values()) {
      // Required field must have a value
      if (field.required && !field.hasValue) {
        return false;
      }

      // If field has async validation
      if (field.validationStatus !== "idle") {
        // Can't be validating
        if (field.validationStatus === "validating") {
          return false;
        }
        // Must be valid
        if (!field.isValidationValid) {
          return false;
        }
      }
    }
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
    fields.set(id, {
      id,
      required,
      hasValue,
      validationStatus,
      isValidationValid,
    });
    version++;
  };

  const unregisterField = (id: string) => {
    fields.delete(id);
    version++;
  };

  const updateField = (
    id: string,
    hasValue: boolean,
    validationStatus?: string,
    isValidationValid?: boolean
  ) => {
    const field = fields.get(id);
    if (field) {
      // Mutate field in place (Map reference stays same)
      field.hasValue = hasValue;
      if (validationStatus !== undefined) {
        field.validationStatus = validationStatus;
      }
      if (isValidationValid !== undefined) {
        field.isValidationValid = isValidationValid;
      }
      version++;
    }
  };

  setContext("formValidation", {
    registerField,
    unregisterField,
    updateField,
  });
</script>

{@render children()}
