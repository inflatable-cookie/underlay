interface HandleSelectionModeEscapeInput {
  event: KeyboardEvent;
  selectionMode: boolean;
  reorderMode: boolean;
  clearSelection: () => void;
  exitReorderMode: () => void;
  setSelectionMode: (value: boolean) => void;
}

export interface SelectionModeController {
  readonly selectionMode: boolean;
  toggleSelectionMode: (reorderMode: boolean) => void;
  handleKeydown: (event: KeyboardEvent, reorderMode: boolean) => void;
}

function computeNextSelectionMode(
  currentSelectionMode: boolean,
  reorderMode: boolean,
  clearSelection: () => void,
  exitReorderMode: () => void
): boolean {
  if (!currentSelectionMode && reorderMode) {
    exitReorderMode();
  }

  const nextSelectionMode = !currentSelectionMode;
  if (!nextSelectionMode) {
    clearSelection();
  }

  return nextSelectionMode;
}

function handleSelectionModeEscape(input: HandleSelectionModeEscapeInput): void {
  const {
    event,
    selectionMode,
    reorderMode,
    clearSelection,
    exitReorderMode,
    setSelectionMode
  } = input;

  if (event.key !== "Escape") return;

  if (selectionMode) {
    setSelectionMode(false);
    clearSelection();
    return;
  }

  if (reorderMode) {
    exitReorderMode();
  }
}

interface CreateSelectionModeControllerParams {
  clearSelection: () => void;
  exitReorderMode: () => void;
}

export function createSelectionModeController(
  params: CreateSelectionModeControllerParams
): SelectionModeController & { selectionMode: boolean } {
  let selectionMode = $state(false);

  function toggleSelectionMode(reorderMode: boolean) {
    selectionMode = computeNextSelectionMode(
      selectionMode,
      reorderMode,
      params.clearSelection,
      params.exitReorderMode
    );
  }

  function handleKeydown(event: KeyboardEvent, reorderMode: boolean) {
    handleSelectionModeEscape({
      event,
      selectionMode,
      reorderMode,
      clearSelection: params.clearSelection,
      exitReorderMode: params.exitReorderMode,
      setSelectionMode: (value) => {
        selectionMode = value;
      }
    });
  }

  return {
    get selectionMode() {
      return selectionMode;
    },
    set selectionMode(value: boolean) {
      selectionMode = value;
    },
    toggleSelectionMode,
    handleKeydown
  };
}
