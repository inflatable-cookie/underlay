interface SelectionTransformStateInput {
  selectionMode: boolean;
  selectedIds: string[];
  buildCopyHref?: (ids: string[]) => string;
  buildMoveHref?: (ids: string[]) => string;
  requireSingleForCopy?: boolean;
}

export interface SelectionTransformState {
  canLaunchBatch: boolean;
  canLaunchCopy: boolean;
  copyHref: string;
  moveHref: string;
}

export function buildSelectionTransformState(
  input: SelectionTransformStateInput
): SelectionTransformState {
  const {
    selectionMode,
    selectedIds,
    buildCopyHref,
    buildMoveHref,
    requireSingleForCopy = false
  } = input;

  const canLaunchBatch = selectionMode && selectedIds.length > 0;
  const canLaunchCopy = requireSingleForCopy
    ? selectionMode && selectedIds.length === 1
    : canLaunchBatch && !!buildCopyHref;

  return {
    canLaunchBatch,
    canLaunchCopy,
    copyHref: canLaunchCopy && buildCopyHref ? buildCopyHref(selectedIds) : "",
    moveHref: canLaunchBatch && buildMoveHref ? buildMoveHref(selectedIds) : ""
  };
}
