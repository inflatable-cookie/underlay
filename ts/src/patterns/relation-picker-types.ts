/**
 * Types for RelationPickerDialog component.
 */

/**
 * A pickable item in the dialog.
 */
export interface PickableItem {
  /** Unique identifier */
  id: string;
  /** Primary display text */
  label: string;
  /** Optional secondary text */
  description?: string | null;
  /** Whether this item cannot be selected */
  disabled?: boolean;
}

/**
 * A section of items with a label.
 */
export interface PickerSection {
  /** Section label (e.g., "Suggestions", "Results (5)") */
  label: string;
  /** Items in this section */
  items: PickableItem[];
}
