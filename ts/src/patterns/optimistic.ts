/**
 * Optimistic update utilities for Svelte applications.
 *
 * Provides helpers for implementing optimistic UI patterns where the UI
 * is updated immediately before the server confirms the operation.
 *
 * @example
 * ```svelte
 * <script lang="ts">
 *   import { createOptimisticList } from '@decodelabs/underlay/patterns';
 *
 *   const todos = createOptimisticList<Todo>([]);
 *
 *   async function addTodo(name: string) {
 *     const { confirm, rollback } = todos.add({ name, completed: false });
 *
 *     try {
 *       const newTodo = await api.todos.create({ name });
 *       confirm(newTodo);
 *     } catch (error) {
 *       rollback();
 *       showToast({ message: 'Failed to add todo', type: 'error' });
 *     }
 *   }
 * </script>
 * ```
 */

import { writable, derived, get, type Readable, type Writable } from "svelte/store";
import { createNoopOperation, withSetValue, withoutSetValue } from "./optimistic/helpers";
export { createOptimisticCounter, type OptimisticCounter } from "./optimistic/counter";

// ============================================================================
// Types
// ============================================================================

/** Result of an optimistic operation with confirm/rollback methods */
export interface OptimisticOperation {
	/** Confirm the operation succeeded (optionally with server data) */
	confirm: (data?: unknown) => void;
	/** Rollback the operation on failure */
	rollback: () => void;
}

/** Result of an optimistic add operation */
export interface OptimisticAddOperation<T> {
	/** Confirm the operation with the real item from server */
	confirm: (realItem: T) => void;
	/** Rollback the operation on failure */
	rollback: () => void;
}

/** Result of an optimistic update operation */
export interface OptimisticUpdateOperation<T> {
	/** Confirm the operation, optionally with real item from server */
	confirm: (realItem?: T) => void;
	/** Rollback the operation on failure */
	rollback: () => void;
}

/** Options for createOptimisticList */
export interface OptimisticListOptions<T> {
	/** Generate temporary ID for new items (default: `temp-${Date.now()}-${random}`) */
	tempId?: () => string;
	/** Key field for matching items (default: 'id') */
	key?: keyof T;
}

/** An optimistic list store with add/remove/update operations */
export interface OptimisticList<T extends { id: string }> extends Readable<T[]> {
	/** Add item optimistically, returns confirm/rollback */
	add: (item: Omit<T, "id">) => OptimisticAddOperation<T>;
	/** Remove item optimistically, returns confirm/rollback */
	remove: (id: string) => OptimisticOperation;
	/** Update item optimistically, returns confirm/rollback */
	update: (id: string, changes: Partial<T>) => OptimisticUpdateOperation<T>;
	/** Set the entire list (e.g., from server) */
	set: (items: T[]) => void;
	/** Derived store: check if an item has a pending operation */
	isPending: Readable<(id: string) => boolean>;
	/** Derived store: get all pending item IDs */
	pendingIds: Readable<Set<string>>;
}

/** An optimistic toggle store for boolean values */
export interface OptimisticToggle extends Readable<boolean> {
	/** Toggle the value optimistically */
	toggle: () => OptimisticOperation;
	/** Set the value optimistically */
	set: (value: boolean) => OptimisticOperation;
	/** Whether there's a pending operation */
	pending: Readable<boolean>;
}

/** Options for createOptimisticValue */
export interface OptimisticValueOptions<T> {
	/** Compare function to detect if value changed (default: strict equality) */
	equals?: (a: T, b: T) => boolean;
}

/** An optimistic value store for any value type */
export interface OptimisticValue<T> extends Readable<T> {
	/** Set the value optimistically */
	set: (value: T) => OptimisticUpdateOperation<T>;
	/** Whether there's a pending operation */
	pending: Readable<boolean>;
}

// ============================================================================
// Helpers
// ============================================================================

/**
 * Generate a unique temporary ID.
 */
function generateTempId(): string {
	return `temp-${Date.now()}-${Math.random().toString(36).slice(2, 9)}`;
}

// ============================================================================
// createOptimisticList
// ============================================================================

/**
 * Create an optimistic list store with add/remove/update operations.
 *
 * Each operation immediately updates the UI and returns `confirm` and `rollback`
 * functions to finalize or undo the change.
 *
 * @example
 * ```typescript
 * const users = createOptimisticList<User>([]);
 *
 * // Add user optimistically
 * async function addUser(name: string) {
 *   const { confirm, rollback } = users.add({ name, email: '' });
 *
 *   try {
 *     const newUser = await api.users.create({ name });
 *     confirm(newUser); // Replace temp item with real data
 *   } catch {
 *     rollback(); // Remove the temp item
 *   }
 * }
 *
 * // Remove user optimistically
 * async function deleteUser(id: string) {
 *   const { confirm, rollback } = users.remove(id);
 *
 *   try {
 *     await api.users.delete(id);
 *     confirm();
 *   } catch {
 *     rollback(); // Restore the removed item
 *   }
 * }
 * ```
 */
export function createOptimisticList<T extends { id: string }>(
	initial: T[] = [],
	options: OptimisticListOptions<T> = {}
): OptimisticList<T> {
	const { tempId = generateTempId } = options;

	const store: Writable<T[]> = writable([...initial]);
	const pending: Writable<Set<string>> = writable(new Set());

	function add(item: Omit<T, "id">): OptimisticAddOperation<T> {
		const id = tempId();
		const optimisticItem = { ...item, id } as T;

		store.update((items) => [...items, optimisticItem]);
		pending.update((p) => withSetValue(p, id));

		return {
			confirm: (realItem: T) => {
				store.update((items) => items.map((i) => (i.id === id ? realItem : i)));
				pending.update((p) => withoutSetValue(p, id));
			},
			rollback: () => {
				store.update((items) => items.filter((i) => i.id !== id));
				pending.update((p) => withoutSetValue(p, id));
			}
		};
	}

	function remove(id: string): OptimisticOperation {
		let removedItem: T | undefined;
		let removedIndex: number = -1;

		store.update((items) => {
			removedIndex = items.findIndex((i) => i.id === id);
			if (removedIndex !== -1) {
				removedItem = items[removedIndex];
				return items.filter((i) => i.id !== id);
			}
			return items;
		});

		pending.update((p) => withSetValue(p, id));

		return {
			confirm: () => {
				pending.update((p) => withoutSetValue(p, id));
			},
			rollback: () => {
				if (removedItem) {
					store.update((items) => {
						// Try to restore at original position
						if (removedIndex >= 0 && removedIndex <= items.length) {
							const result = [...items];
							result.splice(removedIndex, 0, removedItem!);
							return result;
						}
						// Fallback: append to end
						return [...items, removedItem!];
					});
				}
				pending.update((p) => withoutSetValue(p, id));
			}
		};
	}

	function update(
		id: string,
		changes: Partial<T>
	): OptimisticUpdateOperation<T> {
		let previousItem: T | undefined;

		store.update((items) =>
			items.map((i) => {
				if (i.id === id) {
					previousItem = i;
					return { ...i, ...changes };
				}
				return i;
			})
		);

		pending.update((p) => withSetValue(p, id));

		return {
			confirm: (realItem?: T) => {
				if (realItem) {
					store.update((items) => items.map((i) => (i.id === id ? realItem : i)));
				}
				pending.update((p) => withoutSetValue(p, id));
			},
			rollback: () => {
				if (previousItem) {
					store.update((items) => items.map((i) => (i.id === id ? previousItem! : i)));
				}
				pending.update((p) => withoutSetValue(p, id));
			}
		};
	}

	function set(items: T[]): void {
		store.set([...items]);
		pending.set(new Set());
	}

	const isPending = derived(pending, ($p) => (id: string) => $p.has(id));
	const pendingIds = derived(pending, ($p) => new Set($p));

	return {
		subscribe: store.subscribe,
		add,
		remove,
		update,
		set,
		isPending,
		pendingIds
	};
}

// ============================================================================
// createOptimisticToggle
// ============================================================================

/**
 * Create an optimistic toggle store for boolean values.
 *
 * @example
 * ```typescript
 * const liked = createOptimisticToggle(false);
 *
 * async function toggleLike() {
 *   const { confirm, rollback } = liked.toggle();
 *
 *   try {
 *     await api.posts.toggleLike(postId);
 *     confirm();
 *   } catch {
 *     rollback();
 *     showToast({ message: 'Failed to update like', type: 'error' });
 *   }
 * }
 * ```
 */
export function createOptimisticToggle(initial: boolean = false): OptimisticToggle {
	const store: Writable<boolean> = writable(initial);
	const pending: Writable<boolean> = writable(false);

	function toggle(): OptimisticOperation {
		const previous = get(store);
		store.set(!previous);
		pending.set(true);

		return {
			confirm: () => {
				pending.set(false);
			},
			rollback: () => {
				store.set(previous);
				pending.set(false);
			}
		};
	}

	function set(value: boolean): OptimisticOperation {
		const previous = get(store);

		if (previous === value) {
			// No change needed
			return createNoopOperation();
		}

		store.set(value);
		pending.set(true);

		return {
			confirm: () => {
				pending.set(false);
			},
			rollback: () => {
				store.set(previous);
				pending.set(false);
			}
		};
	}

	return {
		subscribe: store.subscribe,
		toggle,
		set,
		pending: { subscribe: pending.subscribe }
	};
}

// ============================================================================
// createOptimisticValue
// ============================================================================

/**
 * Create an optimistic value store for any value type.
 *
 * @example
 * ```typescript
 * const status = createOptimisticValue<'draft' | 'published'>('draft');
 *
 * async function publish() {
 *   const { confirm, rollback } = status.set('published');
 *
 *   try {
 *     await api.posts.publish(postId);
 *     confirm();
 *   } catch {
 *     rollback();
 *     showToast({ message: 'Failed to publish', type: 'error' });
 *   }
 * }
 * ```
 */
export function createOptimisticValue<T>(
	initial: T,
	options: OptimisticValueOptions<T> = {}
): OptimisticValue<T> {
	const { equals = (a, b) => a === b } = options;

	const store: Writable<T> = writable(initial);
	const pending: Writable<boolean> = writable(false);

	function set(value: T): OptimisticUpdateOperation<T> {
		const previous = get(store);

		if (equals(previous, value)) {
			// No change needed
			return createNoopOperation();
		}

		store.set(value);
		pending.set(true);

		return {
			confirm: (realValue?: T) => {
				if (realValue !== undefined) {
					store.set(realValue);
				}
				pending.set(false);
			},
			rollback: () => {
				store.set(previous);
				pending.set(false);
			}
		};
	}

	return {
		subscribe: store.subscribe,
		set,
		pending: { subscribe: pending.subscribe }
	};
}
