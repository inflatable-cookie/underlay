import { writable, derived, type Readable, type Writable } from "svelte/store";
import { withSetValue, withoutSetValue } from "../optimistic/helpers";

export interface OptimisticOperation {
	confirm: (data?: unknown) => void;
	rollback: () => void;
}

export interface OptimisticAddOperation<T> {
	confirm: (realItem: T) => void;
	rollback: () => void;
}

export interface OptimisticUpdateOperation<T> {
	confirm: (realItem?: T) => void;
	rollback: () => void;
}

export interface OptimisticListOptions<T> {
	tempId?: () => string;
	key?: keyof T;
}

export interface OptimisticList<T extends { id: string }> extends Readable<T[]> {
	add: (item: Omit<T, "id">) => OptimisticAddOperation<T>;
	remove: (id: string) => OptimisticOperation;
	update: (id: string, changes: Partial<T>) => OptimisticUpdateOperation<T>;
	set: (items: T[]) => void;
	isPending: Readable<(id: string) => boolean>;
	pendingIds: Readable<Set<string>>;
}

function generateTempId(): string {
	return `temp-${Date.now()}-${Math.random().toString(36).slice(2, 9)}`;
}

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
						if (removedIndex >= 0 && removedIndex <= items.length) {
							const result = [...items];
							result.splice(removedIndex, 0, removedItem!);
							return result;
						}
						return [...items, removedItem!];
					});
				}
				pending.update((p) => withoutSetValue(p, id));
			}
		};
	}

	function update(id: string, changes: Partial<T>): OptimisticUpdateOperation<T> {
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
