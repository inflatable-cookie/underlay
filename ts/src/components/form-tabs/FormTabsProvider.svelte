<script lang="ts">
  import type { Snippet } from "svelte";
  import { getContext, setContext } from "svelte";
  import type { FormValidationContext } from "../text-input/form-validation";
  import type { FormTabsSectionRegistryContext, SectionValidationState } from "./types";

  interface Props {
    children: Snippet;
  }

  let { children }: Props = $props();

  const formValidation = getContext<FormValidationContext>("formValidation");

  // Map of sectionId -> Set of fieldIds
  let sections = $state<Map<string, Set<string>>>(new Map());
  let sectionVersion = $state(0);

  function ensureSection(sectionId: string): Set<string> {
    let section = sections.get(sectionId);
    if (!section) {
      section = new Set();
      sections.set(sectionId, section);
    }
    return section;
  }

  function registerSection(sectionId: string) {
    ensureSection(sectionId);
    sectionVersion++;
  }

  function unregisterSection(sectionId: string) {
    sections.delete(sectionId);
    sectionVersion++;
  }

  function registerFieldInSection(sectionId: string, fieldId: string) {
    // Lazily create section — child fields mount before parent section
    const section = ensureSection(sectionId);
    section.add(fieldId);
    sectionVersion++;
  }

  function unregisterFieldFromSection(sectionId: string, fieldId: string) {
    const section = sections.get(sectionId);
    if (section) {
      section.delete(fieldId);
      sectionVersion++;
    }
  }

  function getSectionState(sectionId: string): SectionValidationState {
    // Access both versions to create reactive dependency
    const _sv = sectionVersion;
    const _fv = formValidation.getVersion?.();

    const fieldIds = sections.get(sectionId);
    if (!fieldIds || fieldIds.size === 0) return "idle";

    let hasInvalid = false;
    let hasIncomplete = false;

    for (const fieldId of fieldIds) {
      const field = formValidation.getFieldState?.(fieldId);
      if (!field) continue;

      // Check for validation errors
      if (field.validationStatus === "invalid" || (field.validationStatus !== "idle" && !field.isValidationValid)) {
        hasInvalid = true;
      }

      // Check for required fields without values
      if (field.required && !field.hasValue) {
        hasIncomplete = true;
      }
    }

    if (hasInvalid) return "invalid";
    if (hasIncomplete) return "incomplete";
    return "valid";
  }

  const registry: FormTabsSectionRegistryContext = {
    registerSection,
    unregisterSection,
    registerFieldInSection,
    unregisterFieldFromSection,
    getSectionState,
  };

  setContext("underlay-form-tabs-registry", registry);
</script>

{@render children()}
