import type { RestoreBlocker, RestoreFieldConflict, RestoreReference } from "../client/types";

export type RestoreReferenceFormatter = (input: {
  blocker: RestoreBlocker;
  reference: RestoreReference;
  role: "entity" | "parent" | "active_occupant";
  conflict?: RestoreFieldConflict | null;
}) => string | null;
