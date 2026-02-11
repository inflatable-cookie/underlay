<script lang="ts">
  import type { Snippet } from "svelte";
  import { getContext, setContext, onMount } from "svelte";
  import type { FormValidationContext } from "../text-input/form-validation";
  import type { FormTabsSectionRegistryContext } from "./types";

  interface Props {
    sectionId: string;
    children: Snippet;
  }

  let { sectionId, children }: Props = $props();

  const registry = getContext<FormTabsSectionRegistryContext>("underlay-form-tabs-registry");
  const parentFormValidation = getContext<FormValidationContext>("formValidation");

  onMount(() => {
    registry.registerSection(sectionId);
    return () => registry.unregisterSection(sectionId);
  });

  // Wrap form validation context to intercept field registrations
  setContext("formValidation", {
    registerField: (
      id: string,
      required: boolean,
      hasValue: boolean,
      validationStatus: string,
      isValidationValid: boolean
    ) => {
      parentFormValidation.registerField(id, required, hasValue, validationStatus, isValidationValid);
      registry.registerFieldInSection(sectionId, id);
    },
    unregisterField: (id: string) => {
      registry.unregisterFieldFromSection(sectionId, id);
      parentFormValidation.unregisterField(id);
    },
    updateField: parentFormValidation.updateField,
    getFieldState: parentFormValidation.getFieldState,
    getVersion: parentFormValidation.getVersion,
  } satisfies FormValidationContext);
</script>

{@render children()}
