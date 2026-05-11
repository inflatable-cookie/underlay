import {
  createLoadedReorderSession,
  createLocalReorderSession,
  createReorderController,
  createSelectionModeController,
  buildSelectionTransformState
} from "@decodelabs/underlay/runtime/data";

interface ItemRow {
  id: string;
  title: string;
}

type TransportRow = ItemRow & {
  domain: "alpha" | "beta" | "gamma";
};

type ShellRow = TransportRow & {
  domain: "alpha" | "beta";
};

// Narrow broad transport unions before values enter shell state.
function isShellRow(row: TransportRow): row is ShellRow {
  return row.domain === "alpha" || row.domain === "beta";
}

// Selection mode and transform-launch state are the first shared layer.
export function createExampleSelectionFlow(clearSelection: () => void, exitReorderMode: () => void) {
  const selection = createSelectionModeController({
    clearSelection,
    exitReorderMode
  });

  const selectedIds = ["row_1"];

  const transformState = buildSelectionTransformState({
    selectionMode: selection.selectionMode,
    selectedIds,
    buildCopyHref: (ids) => `/copy?ids=${ids.join(",")}`,
    buildMoveHref: (ids) => `/move?ids=${ids.join(",")}`,
    requireSingleForCopy: true
  });

  return { selection, transformState };
}

export function createExampleFetchBoundary(rows: TransportRow[]): ShellRow[] {
  return rows.filter(isShellRow);
}

// Use local constrained reorder when the list already has the full reorder set.
export function createExampleLocalReorderSession(
  items: ItemRow[],
  refresh: () => Promise<void>,
  submit: (orderedIds: string[]) => Promise<void>
) {
  const controller = createReorderController(items, submit);

  return createLocalReorderSession({
    getController: () => controller,
    getLatestItems: () => items,
    entityLabel: "item",
    pushInfo: (message) => console.info(message),
    pushSuccess: (message) => console.info(message),
    onRefresh: refresh
  });
}

// Use loaded reorder when normal browsing is paged/cursor-backed and reorder
// needs a separate fetch for the full ordered collection.
export function createExampleLoadedReorderSession(
  refresh: () => Promise<void>,
  loadItems: () => Promise<{ items: Array<{ rowId: string; title: string }>; error?: string }>,
  submit: (orderedIds: string[]) => Promise<void>
) {
  return createLoadedReorderSession({
    loadItems,
    mapItems: (rows) => rows.map((row) => ({ ...row, id: row.rowId })),
    submitReorder: submit,
    entityLabel: "item",
    pushInfo: (message) => console.info(message),
    pushError: (message) => console.error(message),
    onRefresh: refresh
  });
}
