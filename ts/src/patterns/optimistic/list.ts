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

export interface OptimisticListOptions<T, K extends keyof T = keyof T> {
	tempId?: () => string;
	key?: K;
}

export interface OptimisticList<
	T extends object,
	K extends keyof T = "id" extends keyof T ? "id" : keyof T,
> extends Readable<T[]> {
	add: (item: Omit<T, K>) => OptimisticAddOperation<T>;
	remove: (id: string) => OptimisticOperation;
	update: (id: string, changes: Partial<T>) => OptimisticUpdateOperation<T>;
	set: (items: T[]) => void;
	isPending: Readable<(id: string) => boolean>;
	pendingIds: Readable<Set<string>>;
}

function generateTempId(): string {
	return `temp-${Date.now()}-${Math.random().toString(36).slice(2, 9)}`;
}

interface ActiveUpdateEntry<T> {
	opId: number;
	changes: Partial<T>;
}

export function createOptimisticList<
	T extends object,
	K extends keyof T = "id" extends keyof T ? "id" : keyof T,
>(initial: T[] = [], options: OptimisticListOptions<T, K> = {}): OptimisticList<T, K> {
	const { tempId = generateTempId } = options;
	const keyField = (options.key ?? ("id" as K)) as keyof T;
	const getKey = (item: T): string => String(item[keyField]);

	const store: Writable<T[]> = writable([...initial]);
	const pending: Writable<Set<string>> = writable(new Set());
	const pendingCounts = new Map<string, number>();
	const baseItems = new Map<string, T>();
	const activeUpdates = new Map<string, ActiveUpdateEntry<T>[]>();
	let nextOpId = 0;

	function markPending(id: string): void {
		const count = (pendingCounts.get(id) ?? 0) + 1;
		pendingCounts.set(id, count);
		if (count === 1) {
			pending.update((p) => withSetValue(p, id));
		}
	}

	function unmarkPending(id: string): void {
		const count = pendingCounts.get(id) ?? 0;
		if (count <= 1) {
			pendingCounts.delete(id);
			pending.update((p) => withoutSetValue(p, id));
		} else {
			pendingCounts.set(id, count - 1);
		}
	}

	function recomputeItem(id: string): T | undefined {
		const base = baseItems.get(id);
		if (!base) return undefined;
		const ops = activeUpdates.get(id) ?? [];
		let result: T = { ...base };
		for (const op of ops) {
			result = { ...result, ...op.changes };
		}
		return result;
	}

	function add(item: Omit<T, K>): OptimisticAddOperation<T> {
		const id = tempId();
		const optimisticItem = { ...item, [keyField]: id } as T;
		let settled = false;

		store.update((items) => [...items, optimisticItem]);
		markPending(id);

		return {
			confirm: (realItem: T) => {
				if (settled) return;
				settled = true;
				store.update((items) => items.map((i) => (getKey(i) === id ? realItem : i)));
				unmarkPending(id);
			},
			rollback: () => {
				if (settled) return;
				settled = true;
				store.update((items) => items.filter((i) => getKey(i) !== id));
				unmarkPending(id);
			},
		};
	}

	function remove(id: string): OptimisticOperation {
		let removedItem: T | undefined;
		let removedIndex = -1;
		let settled = false;

		store.update((items) => {
			removedIndex = items.findIndex((i) => getKey(i) === id);
			if (removedIndex !== -1) {
				removedItem = items[removedIndex];
				return items.filter((i) => getKey(i) !== id);
			}
			return items;
		});

		markPending(id);

		return {
			confirm: () => {
				if (settled) return;
				settled = true;
				baseItems.delete(id);
				activeUpdates.delete(id);
				unmarkPending(id);
			},
			rollback: () => {
				if (settled) return;
				settled = true;
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
				unmarkPending(id);
			},
		};
	}

	function update(id: string, changes: Partial<T>): OptimisticUpdateOperation<T> {
		const opId = ++nextOpId;
		let found = false;
		let settled = false;

		store.update((items) =>
			items.map((i) => {
				if (getKey(i) === id) {
					found = true;
					if (!baseItems.has(id)) {
						baseItems.set(id, i);
					}
					const ops = activeUpdates.get(id) ?? [];
					ops.push({ opId, changes: { ...changes } });
					activeUpdates.set(id, ops);
					return recomputeItem(id) ?? { ...i, ...changes };
				}
				return i;
			}),
		);

		markPending(id);

		return {
			confirm: (realItem?: T) => {
				if (settled) return;
				settled = true;

				if (found) {
					const ops = activeUpdates.get(id);
					const base = baseItems.get(id);
					if (ops && base) {
						const remaining = ops.filter((op) => op.opId !== opId);
						if (realItem) {
							baseItems.set(id, realItem);
							// Older uncommitted ops are superseded by the server snapshot.
							const newerOnly = remaining.filter((op) => op.opId > opId);
							if (newerOnly.length > 0) {
								activeUpdates.set(id, newerOnly);
							} else {
								activeUpdates.delete(id);
								baseItems.delete(id);
							}
							const nextItem =
								newerOnly.length > 0
									? newerOnly.reduce((acc, op) => ({ ...acc, ...op.changes }), { ...realItem })
									: realItem;
							store.update((items) => items.map((i) => (getKey(i) === id ? nextItem : i)));
						} else {
							const nextBase = { ...base, ...changes };
							baseItems.set(id, nextBase);
							// Strip fields confirmed by this op from older still-pending ops
							// so their later rollback/replay cannot overwrite newer confirmed values.
							const confirmedKeys = Object.keys(changes) as Array<keyof T>;
							for (const op of remaining) {
								if (op.opId < opId) {
									for (const k of confirmedKeys) {
										delete op.changes[k];
									}
								}
							}
							if (remaining.length > 0) {
								activeUpdates.set(id, remaining);
								const nextItem = recomputeItem(id);
								if (nextItem) {
									store.update((items) => items.map((i) => (getKey(i) === id ? nextItem : i)));
								}
							} else {
								activeUpdates.delete(id);
								baseItems.delete(id);
								store.update((items) => items.map((i) => (getKey(i) === id ? nextBase : i)));
							}
						}
					} else if (realItem) {
						store.update((items) => items.map((i) => (getKey(i) === id ? realItem : i)));
					}
				}
				unmarkPending(id);
			},
			rollback: () => {
				if (settled) return;
				settled = true;

				if (found) {
					const ops = activeUpdates.get(id);
					if (ops) {
						const remaining = ops.filter((op) => op.opId !== opId);
						if (remaining.length > 0) {
							activeUpdates.set(id, remaining);
							const nextItem = recomputeItem(id);
							if (nextItem) {
								store.update((items) => items.map((i) => (getKey(i) === id ? nextItem : i)));
							}
						} else {
							const base = baseItems.get(id);
							activeUpdates.delete(id);
							baseItems.delete(id);
							if (base) {
								store.update((items) => items.map((i) => (getKey(i) === id ? base : i)));
							}
						}
					}
				}
				unmarkPending(id);
			},
		};
	}

	function set(items: T[]): void {
		pendingCounts.clear();
		baseItems.clear();
		activeUpdates.clear();
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
		pendingIds,
	};
}
