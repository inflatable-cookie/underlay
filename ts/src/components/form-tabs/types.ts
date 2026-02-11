export type SectionValidationState = "valid" | "invalid" | "incomplete" | "idle";

export interface FormTabsSectionRegistryContext {
  registerSection: (sectionId: string) => void;
  unregisterSection: (sectionId: string) => void;
  registerFieldInSection: (sectionId: string, fieldId: string) => void;
  unregisterFieldFromSection: (sectionId: string, fieldId: string) => void;
  getSectionState: (sectionId: string) => SectionValidationState;
}
