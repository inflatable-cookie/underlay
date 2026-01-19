/**
 * Definition for a field that can be sorted
 */
export interface OrderByFieldDefinition {
  /** Unique key for the field */
  key: string;
  /** Display label for the field */
  label: string;
  /** Default sort direction when field is first added (default: "asc") */
  defaultDirection?: "asc" | "desc";
}

/**
 * A single field in the current sort order
 */
export interface OrderByField {
  /** Key of the field being sorted */
  key: string;
  /** Sort direction */
  direction: "asc" | "desc";
}

/**
 * The complete sort order - an array of fields with directions
 */
export type OrderByValue = OrderByField[];
