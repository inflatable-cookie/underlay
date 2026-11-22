/**
 * Reorder Controller
 *
 * A Svelte 5 reactive controller for managing drag-and-drop reordering state.
 * Used with EditableList component for batch-commit reordering.
 *
 * @example
 * ```svelte
 * <script lang="ts">
 *   import { createReorderController } from '@inflatable-cookie/underlay/runtime/data';
 *
 *   const controller = createReorderController(items, async (orderedIds) => {
 *     await api.reorder(orderedIds);
 *   });
 * </script>
 *
 * {#each controller.pending as item (item.id)}
 *   <ListCard {item} variant="compact" />
 * {/each}
 *
 * <Button onclick={controller.submit} disabled={!controller.isDirty}>
 *   Save Order
 * </Button>
 * ```
 */

/** Item constraint - must have an id property */
export interface ReorderableItem {
	id: string;
}

/** Controller interface for reorder state management */
export interface ReorderController<T extends ReorderableItem> {
	/** Current pending order (mutable, used by drag-drop library) */
	pending: T[];
	/** Original order for comparison */
	readonly original: readonly T[];
	/** Whether order has changed from original */
	readonly isDirty: boolean;
	/** Whether submit is in progress */
	readonly isPending: boolean;
	/** Error from last submit attempt */
	readonly error: Error | null;
	/** Move item from one index to another */
	move(fromIndex: number, toIndex: number): void;
	/** Reset to original order */
	reset(): void;
	/** Submit the new order */
	submit(): Promise<void>;
	/** Update pending items (used by dnd library handlers) */
	updatePending(items: T[]): void;
	/** Merge new items (for conflict resolution) */
	mergeNewItems(newItems: T[]): void;
	/** Remove items by ID (for conflict resolution) */
	removeItems(idsToRemove: string[]): void;
}

const controllerSyncHandlers = new WeakMap<
	object,
	(latestItems: readonly ReorderableItem[]) => void
>();

/**
 * Reconcile mapped row data in an active reorder controller while retaining
 * its pending/original ID order and any caller-supplied extra row fields.
 */
export function syncReorderControllerItems<T extends ReorderableItem>(
	controller: ReorderController<T>,
	latestItems: readonly T[],
): void {
	controllerSyncHandlers.get(controller)?.(latestItems);
}

/**
 * Creates a reactive reorder controller for managing drag-and-drop state.
 *
 * @param initialItems - The initial list of items to reorder
 * @param submitFn - Async function called with ordered IDs on submit
 * @returns A reactive controller with state and methods
 */
export function createReorderController<T extends ReorderableItem>(
	initialItems: T[],
	submitFn: (orderedIds: string[]) => Promise<void>,
): ReorderController<T> {
	// Store original for comparison
	let original = $state<readonly T[]>([...initialItems]);

	// Mutable pending state
	let pending = $state<T[]>([...initialItems]);
	let isPending = $state(false);
	let error = $state<Error | null>(null);

	const isDirty = $derived(
		pending.length !== original.length || pending.some((item, i) => item.id !== original[i].id),
	);

	const controller: ReorderController<T> = {
		get pending() {
			return pending;
		},
		set pending(value: T[]) {
			pending = value;
		},
		get original() {
			return original;
		},
		get isDirty() {
			return isDirty;
		},
		get isPending() {
			return isPending;
		},
		get error() {
			return error;
		},

		/**
		 * Move an item from one index to another
		 */
		move(fromIndex: number, toIndex: number) {
			if (fromIndex === toIndex) return;
			if (fromIndex < 0 || fromIndex >= pending.length) return;
			if (toIndex < 0 || toIndex >= pending.length) return;

			const item = pending[fromIndex];
			const newPending = [...pending];
			newPending.splice(fromIndex, 1);
			newPending.splice(toIndex, 0, item);
			pending = newPending;
		},

		/**
		 * Reset to original order, clearing any pending changes
		 */
		reset() {
			pending = [...original];
			error = null;
		},

		/**
		 * Submit the new order via the provided submit function
		 */
		async submit() {
			if (isPending || !isDirty) return;

			isPending = true;
			error = null;
			const submitted = [...pending];

			try {
				await submitFn(submitted.map((item) => item.id));
				original = submitted;
			} catch (e) {
				error = e instanceof Error ? e : new Error(String(e));
				throw e;
			} finally {
				isPending = false;
			}
		},

		/**
		 * Update pending items (called by drag-drop library event handlers)
		 */
		updatePending(items: T[]) {
			pending = items;
		},

		/**
		 * Merge new items into pending list (for conflict resolution)
		 * New items are appended at the end
		 */
		mergeNewItems(newItems: T[]) {
			const existingIds = new Set(pending.map((item) => item.id));
			const toAdd = newItems.filter((item) => !existingIds.has(item.id));
			if (toAdd.length > 0) {
				pending = [...pending, ...toAdd];
			}
		},

		/**
		 * Remove items by ID from pending list (for conflict resolution)
		 */
		removeItems(idsToRemove: string[]) {
			const removeSet = new Set(idsToRemove);
			pending = pending.filter((item) => !removeSet.has(item.id));
		},
	};

	let prevMappedById = new Map<string, T>(initialItems.map((item) => [item.id, item]));

	controllerSyncHandlers.set(controller, (latestItems) => {
		const byId = new Map((latestItems as readonly T[]).map((item) => [item.id, item]));
		const mergePendingRow = (item: T): T => {
			const next = byId.get(item.id);
			if (!next) return item;

			const prevMapped = prevMappedById.get(item.id);
			if (prevMapped === item) {
				return next;
			}

			const extraDescriptors: PropertyDescriptorMap = {};
			let hasExtra = false;
			for (const key of Object.keys(item)) {
				if (prevMapped && Object.prototype.hasOwnProperty.call(prevMapped, key)) {
					continue;
				}
				if (Object.prototype.hasOwnProperty.call(next, key)) {
					continue;
				}
				const descriptor = Object.getOwnPropertyDescriptor(item, key);
				if (descriptor) {
					extraDescriptors[key] = descriptor;
					hasExtra = true;
				}
			}

			if (!hasExtra) {
				return next;
			}

			return Object.defineProperties(Object.create(Object.getPrototypeOf(next)), {
				...Object.getOwnPropertyDescriptors(next),
				...extraDescriptors,
			}) as T;
		};

		original = original.map((item) => byId.get(item.id) ?? item);
		pending = pending.map(mergePendingRow);
		prevMappedById = byId;
	});

	return controller;
}
