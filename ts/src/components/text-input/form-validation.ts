import {
  hasInputValue,
  isValidationStatusValid,
  type InputValidationStatus
} from "./validation-state";

export interface FormValidationContext {
  registerField: (
    id: string,
    required: boolean,
    hasValue: boolean,
    validationStatus: string,
    isValidationValid: boolean
  ) => void;
  unregisterField: (id: string) => void;
  updateField: (
    id: string,
    hasValue: boolean,
    validationStatus?: string,
    isValidationValid?: boolean
  ) => void;
}

export function registerFormValidationField(
  formValidation: FormValidationContext,
  fieldId: string,
  required: boolean,
  value: string,
  validationStatus: InputValidationStatus
): void {
  formValidation.registerField(
    fieldId,
    required,
    hasInputValue(value),
    validationStatus,
    isValidationStatusValid(validationStatus)
  );
}

export function updateFormValidationField(
  formValidation: FormValidationContext,
  fieldId: string,
  value: string,
  validationStatus: InputValidationStatus
): void {
  formValidation.updateField(
    fieldId,
    hasInputValue(value),
    validationStatus,
    isValidationStatusValid(validationStatus)
  );
}
