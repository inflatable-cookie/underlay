import type {
  RestoreBlockedResult,
  RestoreBlocker,
  RestoreFieldConflict,
  RestoreReference,
} from "../client/soft-delete";

export interface RestoreBlockedAction {
  label: string;
  href: string;
}

export interface RestoreBlockedActionContext {
  blocker: RestoreBlocker;
  reference: RestoreReference;
  role: "entity" | "parent" | "active_occupant";
  conflict?: RestoreFieldConflict;
}

export type RestoreReferenceFormatter = (
  context: RestoreBlockedActionContext,
) => string | null | undefined;

export type RestoreBlockedActionResolver = (
  context: RestoreBlockedActionContext,
) => RestoreBlockedAction[] | null | undefined;

function startCase(value: string): string {
  return value
    .split(/[_\s-]+/)
    .filter(Boolean)
    .map((token) => token.charAt(0).toUpperCase() + token.slice(1))
    .join(" ");
}

function resolveRestoreReferenceText(
  context: RestoreBlockedActionContext,
  formatter?: RestoreReferenceFormatter,
): string {
  const formatted = formatter?.(context)?.trim();
  if (formatted) {
    return formatted;
  }

  const reference = context.reference;
  const kind = startCase(reference.kind);
  if (reference.displayName && reference.displayName.trim().length > 0) {
    return `${kind}: ${reference.displayName}`;
  }
  return `${kind} ${reference.id}`;
}

export function formatRestoreReference(
  reference: RestoreReference,
  formatter?: RestoreReferenceFormatter,
): string {
  return resolveRestoreReferenceText(
    {
      blocker: {
        kind: "conflict",
        entity: reference,
        fieldConflicts: [],
      },
      reference,
      role: "entity",
    },
    formatter,
  );
}

export function formatRestoreFieldConflict(
  conflict: RestoreFieldConflict,
  blocker?: RestoreBlocker,
  formatter?: RestoreReferenceFormatter,
): string {
  const fieldLabel = startCase(conflict.fieldName);
  const candidate =
    conflict.candidateValue && conflict.candidateValue.trim().length > 0
      ? ` "${conflict.candidateValue}"`
      : "";

  if (conflict.activeOccupant) {
    return `${fieldLabel}${candidate} is already used by ${resolveRestoreReferenceText(
      {
        blocker: blocker ?? {
          kind: "conflict",
          entity: conflict.activeOccupant,
          fieldConflicts: [conflict],
        },
        reference: conflict.activeOccupant,
        role: "active_occupant",
        conflict,
      },
      formatter,
    )}.`;
  }

  return `${fieldLabel}${candidate} conflicts with an active item.`;
}

export function formatRestoreBlockerSummary(
  blocker: RestoreBlocker,
  formatter?: RestoreReferenceFormatter,
): string {
  if (blocker.message && blocker.message.trim().length > 0) {
    return blocker.message;
  }

  if (blocker.kind === "parent_state") {
    const parent = blocker.parent
      ? resolveRestoreReferenceText(
        {
          blocker,
          reference: blocker.parent,
          role: "parent",
        },
        formatter,
      )
      : "a parent item";
    const parentState =
      blocker.parentState && blocker.parentState.trim().length > 0
        ? blocker.parentState.replace(/_/g, " ")
        : "not restorable";
    return `${resolveRestoreReferenceText(
      {
        blocker,
        reference: blocker.entity,
        role: "entity",
      },
      formatter,
    )} depends on ${parent}, which is ${parentState}.`;
  }

  if (blocker.kind === "invalid_state") {
    return `${resolveRestoreReferenceText(
      {
        blocker,
        reference: blocker.entity,
        role: "entity",
      },
      formatter,
    )} cannot be restored in its current state.`;
  }

  if (blocker.fieldConflicts.length > 0) {
    return formatRestoreFieldConflict(blocker.fieldConflicts[0], blocker, formatter);
  }

  return `${resolveRestoreReferenceText(
    {
      blocker,
      reference: blocker.entity,
      role: "entity",
    },
    formatter,
  )} conflicts with an active item.`;
}

export function formatRestoreBlockedHeadline(result: RestoreBlockedResult): string {
  const blockerCount = result.blockers.length;
  const noun = blockerCount === 1 ? "blocker" : "blockers";
  return `Restore blocked by ${blockerCount} ${noun}.`;
}
