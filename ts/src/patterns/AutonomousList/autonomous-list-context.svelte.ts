import { createListController } from "../list-controller.svelte";
import { createPaginationController, type ServerPaginationResult } from "../pagination.svelte";
import { createClientPagination } from "../pagination.svelte";
import { useBatchActions, type BatchAction } from "../batch-actions.svelte";
import { createReorderController, type ReorderController } from "../reorder-controller.svelte";
import type { PaginationController } from "../pagination-types";
import type { AutonomousListProps, ServerFetcher } from "./autonomous-list-types";

interface ReorderableItemWithLabel {
  id: string;
  label: string;
}

export interface AutonomousListState<T> {
  /** Current items to display (from list controller or pagination) */
  readonly items: T[];
  /** Whether data is loading */
  readonly loading: boolean;
  /** Error message if fetch failed */
  readonly error: string | null;
  /** Total item count (for header display) */
  readonly total: number | null;

  readonly batch: ReturnType<typeof useBatchActions<string>>;
  readonly reorder: ReorderController<ReorderableItemWithLabel> | null;
  readonly pagination: PaginationController<T> | null;
  readonly selectionMode: boolean;
  readonly reorderMode: boolean;
  toggleSelectionMode: () => void;
  enterReorderMode: () => void;
  exitReorderMode: () => void;
  handleReorderSuccess: () => Promise<void>;
  handleKeydown: (event: KeyboardEvent) => void;
  readonly allItemIds: string[];
  readonly canReorder: boolean;

  /** Trigger initial data fetch when auth is ready */
  tryFetch: (authLoading: boolean, currentUser: unknown) => Promise<void>;
  /** Force refetch with current filters */
  refresh: () => Promise<void>;
  /** Reset pagination to page 1 (call when filters change) */
  resetPagination: () => Promise<void>;
}

interface AutonomousListStateOptions<T> {
  fetcher?: AutonomousListProps<T>["fetcher"];
  serverFetcher?: ServerFetcher<T>;
  idField?: string;
  batchActions?: BatchAction<string>[];
  reorderable?: AutonomousListProps<T>["reorderable"];
  pageSize?: number;
  persistKey?: string;
  onDataChange?: () => void;
}

/**
 * Creates the autonomous list state by composing list controller or
 * pagination controller, batch actions, and reorder controller.
 */
export function createAutonomousListState<T>(
  props: AutonomousListStateOptions<T>
): AutonomousListState<T> {
  const idField = props.idField ?? "id";
  const isServerMode = !!props.serverFetcher;

  // ── Data layer ──
  // Either a list controller (client mode) or pagination controller (server mode)

  let listController: ReturnType<typeof createListController<T, Record<string, unknown>>> | null = null;
  let serverPagination: ServerPaginationResult<T> | null = null;
  let clientPagination: PaginationController<T> | null = null;

  if (isServerMode) {
    // Server-side pagination mode
    const serverFetcher = props.serverFetcher!;

    // We need a wrapper that captures the current filter state.
    // Filter values will be set from the component via resetPagination.
    let currentFilters: Record<string, unknown> = {};

    serverPagination = createPaginationController<T>(
      async (fetchFn, token, params) => {
        return await serverFetcher(fetchFn, token, params, currentFilters);
      },
      {
        pageSize: props.pageSize ?? 30,
        persistKey: props.persistKey
      }
    );

    // Store a setter for filters so resetPagination can update them
    const _setFilters = (filters: Record<string, unknown>) => {
      currentFilters = filters;
    };

    // Expose the filter setter via a side-channel
    (serverPagination as any)._setFilters = _setFilters;
  } else if (props.fetcher) {
    // Client-side mode
    listController = createListController<T, Record<string, unknown>>(props.fetcher);

    // Create client-side pagination if pageSize is set
    if (props.pageSize) {
      clientPagination = createClientPagination<T>(
        () => listController!.items,
        { pageSize: props.pageSize, persistKey: props.persistKey }
      );
    }
  }

  // Unified accessors
  const items = $derived.by((): T[] => {
    if (serverPagination) return serverPagination.items;
    if (clientPagination) return clientPagination.items;
    if (listController) return listController.items;
    return [];
  });

  const loading = $derived.by((): boolean => {
    if (serverPagination) return serverPagination.loading;
    if (listController) return listController.loading;
    return false;
  });

  const error = $derived.by((): string | null => {
    if (serverPagination) return serverPagination.error;
    if (listController) return listController.error;
    return null;
  });

  const total = $derived.by((): number | null => {
    if (serverPagination) return serverPagination.total;
    if (listController) return listController.items.length;
    return null;
  });

  // The active pagination controller (server or client)
  const activePagination = $derived.by((): PaginationController<T> | null => {
    if (serverPagination) return serverPagination;
    if (clientPagination) return clientPagination;
    return null;
  });

  // ── Batch actions ──

  const batch = useBatchActions<string>();

  if (props.batchActions) {
    for (const action of props.batchActions) {
      batch.registerAction(action);
    }
  }

  // ── Selection & reorder mode ──

  let selectionMode = $state(false);
  let reorderMode = $state(false);
  let reorderController = $state<ReorderController<ReorderableItemWithLabel> | null>(null);

  function getItems(): T[] {
    if (serverPagination) return serverPagination.items;
    if (listController) return listController.items;
    return [];
  }

  function buildReorderItems(): ReorderableItemWithLabel[] {
    return getItems().map((item) => {
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
    if (serverPagination) {
      await serverPagination.refresh();
    } else if (listController) {
      await listController.refresh();
    }
    props.onDataChange?.();
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
    items.map((item) => {
      const record = item as Record<string, unknown>;
      return String(record[idField] ?? "");
    })
  );

  const canReorder = $derived.by(() => {
    if (!props.reorderable) return false;
    if (props.reorderable.condition) {
      const filters = listController ? listController.filters : {};
      return props.reorderable.condition(filters);
    }
    return true;
  });

  // ── Auth-gated fetch ──

  async function tryFetch(authLoading: boolean, currentUser: unknown) {
    if (serverPagination) {
      await serverPagination.tryFetch(authLoading, currentUser);
    } else if (listController) {
      await listController.tryFetch(authLoading, currentUser);
    }
  }

  async function refresh() {
    if (serverPagination) {
      await serverPagination.refresh();
    } else if (listController) {
      await listController.refresh();
    }
    props.onDataChange?.();
  }

  async function resetPagination() {
    if (serverPagination) {
      await serverPagination.reset();
    } else if (clientPagination) {
      await clientPagination.reset();
    } else if (listController) {
      await listController.refresh();
    }
  }

  return {
    get items() { return items; },
    get loading() { return loading; },
    get error() { return error; },
    get total() { return total; },
    batch,
    get reorder() { return reorderController; },
    get pagination() { return activePagination; },
    get selectionMode() { return selectionMode; },
    get reorderMode() { return reorderMode; },
    toggleSelectionMode,
    enterReorderMode,
    exitReorderMode,
    handleReorderSuccess,
    handleKeydown,
    get allItemIds() { return allItemIds; },
    get canReorder() { return canReorder; },
    tryFetch,
    refresh,
    resetPagination
  };
}
