export type InputValidationStatus = "idle" | "validating" | "valid" | "invalid";

export function hasInputValue(value: string | number): boolean {
  if (typeof value === "number") {
    return !Number.isNaN(value);
  }

  return String(value).trim() !== "";
}

export function isValidationStatusValid(status: InputValidationStatus): boolean {
  return status === "valid" || status === "idle";
}
