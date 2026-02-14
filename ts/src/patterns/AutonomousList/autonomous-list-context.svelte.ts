import { createListController } from "../list-controller.svelte";
import { useBatchActions, type BatchAction } from "../batch-actions.svelte";
import { createReorderController, type ReorderController } from "../reorder-controller.svelte";
import type { AutonomousListProps } from "./autonomous-list-types";

interface ReorderableItemWithLabel {
  id: string;
  label: string;
}

export interface AutonomousListState<T> {
  readonly list: ReturnType<typeof createListController<T, Record<string, unknown>>>;
  readonly batch: ReturnType<typeof useBatchActions<string>>;
  readonly reorder: ReorderController<ReorderableItemWithLabel> | null;
  readonly selectionMode: boolean;
  readonly reorderMode: boolean;
  toggleSelectionMode: () => void;
  enterReorderMode: () => void;
  exitReorderMode: () => void;
  handleReorderSuccess: () => Promise<void>;
  handleKeydown: (event: KeyboardEvent) => void;
  readonly allItemIds: string[];
  readonly canReorder: boolean;
}

/**
 * Creates the autonomous list state by composing list controller,
 * batch actions, and reorder controller.
 */
export function createAutonomousListState<T>(
  props: Pick<AutonomousListProps<T>, "fetcher" | "idField" | "batchActions" | "reorderable">
): AutonomousListState<T> {
  const idField = props.idField ?? "id";

  // List controller
  const list = createListController<T, Record<string, unknown>>(
    props.fetcher
  );

  // Batch actions
  const batch = useBatchActions<string>();

  // Register batch actions
  if (props.batchActions) {
    for (const action of props.batchActions) {
      batch.registerAction(action);
    }
  }

  // Selection mode
  let selectionMode = $state(false);
  let reorderMode = $state(false);

  // Reorder controller — created lazily when entering reorder mode
  // because items aren't available until after data fetch completes
  let reorderController = $state<ReorderController<ReorderableItemWithLabel> | null>(null);

  function buildReorderItems(): ReorderableItemWithLabel[] {
    return list.items.map((item) => {
      const record = item as Record<string, unknown>;
      return {
        id: String(record[idField] ?? ""),
        label: String(record["title"] ?? record["name"] ?? record["label"] ?? record[idField] ?? "")
      };
    });
  }

  function toggleSelectionMode() {
    selectionMode = !selectionMode;
    if (!selectionMode) {
      batch.clear();
    }
  }

  function enterReorderMode() {
    if (!props.reorderable) return;
    const reorderable = props.reorderable;
    reorderController = createReorderController(
      buildReorderItems(),
      async (orderedIds) => {
        await reorderable.execute(orderedIds, fetch, "");
      }
    );
    reorderMode = true;
    selectionMode = false;
    batch.clear();
  }

  function exitReorderMode() {
    reorderMode = false;
    reorderController = null;
  }

  async function handleReorderSuccess() {
    exitReorderMode();
    await list.refresh();
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      if (selectionMode) {
        selectionMode = false;
        batch.clear();
        event.preventDefault();
      } else if (reorderMode) {
        exitReorderMode();
        event.preventDefault();
      }
    }
  }

  const allItemIds = $derived(
    list.items.map((item) => {
      const record = item as Record<string, unknown>;
      return String(record[idField] ?? "");
    })
  );

  const canReorder = $derived.by(() => {
    if (!props.reorderable) return false;
    if (props.reorderable.condition) {
      return props.reorderable.condition(list.filters);
    }
    return true;
  });

  return {
    list,
    batch,
    get reorder() { return reorderController; },
    get selectionMode() { return selectionMode; },
    get reorderMode() { return reorderMode; },
    toggleSelectionMode,
    enterReorderMode,
    exitReorderMode,
    handleReorderSuccess,
    handleKeydown,
    get allItemIds() { return allItemIds; },
    get canReorder() { return canReorder; }
  };
}
