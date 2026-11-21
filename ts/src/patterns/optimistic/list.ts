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

export type OptimisticStringKey<T extends object> = {
	[K in keyof T]-?: [string] extends [T[K]] ? ([T[K]] extends [string] ? K : never) : never;
}[keyof T];

type UnionToIntersection<U> = (U extends unknown ? (arg: U) => void : never) extends (
	arg: infer I,
) => void
	? I
	: never;

type IsSingleKey<K> = [K] extends [never]
	? false
	: [K] extends [UnionToIntersection<K>]
		? true
		: false;

export type OptimisticAddItemForKey<T extends object, K extends OptimisticStringKey<T>> =
	IsSingleKey<K> extends true ? Omit<T, K> : T;

export interface OptimisticListOptions<T extends object> {
	tempId?: () => string;
	key?: OptimisticStringKey<T>;
}

export type OptimisticDefaultAddItem<T extends object> = "id" extends keyof T ? Omit<T, "id"> : T;

export interface OptimisticList<
	T extends object,
	TAdd = OptimisticDefaultAddItem<T>,
> extends Readable<T[]> {
	add: (item: TAdd) => OptimisticAddOperation<T>;
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

interface RemovedEntry<T> {
	item: T;
	index: number;
}

export function createOptimisticList<T extends { id: string }>(
	initial?: T[],
	options?: { tempId?: () => string; key?: "id" },
): OptimisticList<T>;
export function createOptimisticList<T extends object, K extends OptimisticStringKey<T>>(
	initial: T[] | undefined,
	options: { tempId?: () => string; key: K },
): OptimisticList<T, OptimisticAddItemForKey<T, K>>;
export function createOptimisticList<T extends object>(
	initial: T[] | undefined,
	options: { tempId?: () => string; key: OptimisticStringKey<T> },
): OptimisticList<T, T>;
export function createOptimisticList<T extends { id: string }>(
	initial?: T[],
	options?: OptimisticListOptions<T>,
): OptimisticList<T>;
export function createOptimisticList<T extends object>(
	initial: T[] = [],
	options: OptimisticListOptions<T> = {},
): OptimisticList<T, Partial<T>> {
	const { tempId = generateTempId } = options;
	const keyField = (options.key ?? ("id" as keyof T)) as keyof T;
	const getKey = (item: T): string => String(item[keyField]);

	const store: Writable<T[]> = writable([...initial]);
	const pending: Writable<Set<string>> = writable(new Set());
	const pendingCounts = new Map<string, number>();
	const baseItems = new Map<string, T>();
	const activeUpdates = new Map<string, ActiveUpdateEntry<T>[]>();
	const lastConfirmedOpId = new Map<string, number>();
	const removedEntries = new Map<string, RemovedEntry<T>>();
	let nextOpId = 0;
	let listGeneration = 0;

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

	function syncItemState(id: string, nextItem: T): void {
		const removed = removedEntries.get(id);
		if (removed) {
			removed.item = nextItem;
		}
		store.update((items) => items.map((i) => (getKey(i) === id ? nextItem : i)));
	}

	function add(item: Partial<T>): OptimisticAddOperation<T> {
		const id = tempId();
		const optimisticItem =
			keyField !== "id" && keyField in item && !("id" in item)
				? ({ ...item, id, [keyField]: id } as T)
				: ({ ...item, [keyField]: id } as T);
		const generation = listGeneration;
		let settled = false;

		store.update((items) => [...items, optimisticItem]);
		markPending(id);

		return {
			confirm: (realItem: T) => {
				if (settled || generation !== listGeneration) return;
				settled = true;
				store.update((items) => items.map((i) => (getKey(i) === id ? realItem : i)));
				unmarkPending(id);
			},
			rollback: () => {
				if (settled || generation !== listGeneration) return;
				settled = true;
				store.update((items) => items.filter((i) => getKey(i) !== id));
				unmarkPending(id);
			},
		};
	}

	function remove(id: string): OptimisticOperation {
		const generation = listGeneration;
		let removedEntry: RemovedEntry<T> | undefined;
		let settled = false;

		store.update((items) => {
			const removedIndex = items.findIndex((i) => getKey(i) === id);
			if (removedIndex !== -1) {
				removedEntry = { item: items[removedIndex], index: removedIndex };
				removedEntries.set(id, removedEntry);
				return items.filter((i) => getKey(i) !== id);
			}
			return items;
		});

		markPending(id);

		return {
			confirm: () => {
				if (settled || generation !== listGeneration) return;
				settled = true;
				if (removedEntries.get(id) === removedEntry) {
					removedEntries.delete(id);
				}
				baseItems.delete(id);
				activeUpdates.delete(id);
				lastConfirmedOpId.delete(id);
				unmarkPending(id);
			},
			rollback: () => {
				if (settled) return;
				settled = true;
				if (removedEntry) {
					const itemToRestore =
						generation === listGeneration
							? (recomputeItem(id) ?? removedEntry.item)
							: removedEntry.item;
					const restoreIndex = removedEntry.index;
					if (removedEntries.get(id) === removedEntry) {
						removedEntries.delete(id);
					}
					store.update((items) => {
						if (items.some((i) => getKey(i) === id)) {
							return items;
						}
						if (restoreIndex >= 0 && restoreIndex <= items.length) {
							const result = [...items];
							result.splice(restoreIndex, 0, itemToRestore);
							return result;
						}
						return [...items, itemToRestore];
					});
				}
				if (generation === listGeneration) {
					unmarkPending(id);
				}
			},
		};
	}

	function update(id: string, changes: Partial<T>): OptimisticUpdateOperation<T> {
		const opId = ++nextOpId;
		const generation = listGeneration;
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
				if (settled || generation !== listGeneration) return;
				settled = true;

				if (found) {
					const ops = activeUpdates.get(id);
					const base = baseItems.get(id);
					const currentOp = ops?.find((op) => op.opId === opId);
					const latestConfirmed = lastConfirmedOpId.get(id) ?? 0;
					const isOlderThanConfirmed = opId < latestConfirmed;

					if (opId > latestConfirmed) {
						lastConfirmedOpId.set(id, opId);
					}

					if (ops && base && currentOp) {
						const remaining = ops.filter((op) => op.opId !== opId);
						if (realItem && !isOlderThanConfirmed) {
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
							syncItemState(id, nextItem);
						} else {
							// Apply only non-superseded fields from currentOp.changes so
							// confirming an older op (with or without a stale snapshot)
							// never overwrites a newer confirmed value.
							const nextBase = { ...base, ...currentOp.changes };
							baseItems.set(id, nextBase);
							const confirmedKeys = Object.keys(currentOp.changes) as Array<keyof T>;
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
									syncItemState(id, nextItem);
								}
							} else {
								activeUpdates.delete(id);
								baseItems.delete(id);
								syncItemState(id, nextBase);
							}
						}
					}
				}
				unmarkPending(id);
			},
			rollback: () => {
				if (settled || generation !== listGeneration) return;
				settled = true;

				if (found) {
					const ops = activeUpdates.get(id);
					if (ops && ops.some((op) => op.opId === opId)) {
						const remaining = ops.filter((op) => op.opId !== opId);
						if (remaining.length > 0) {
							activeUpdates.set(id, remaining);
							const nextItem = recomputeItem(id);
							if (nextItem) {
								syncItemState(id, nextItem);
							}
						} else {
							const base = baseItems.get(id);
							activeUpdates.delete(id);
							baseItems.delete(id);
							if (base) {
								syncItemState(id, base);
							}
						}
					}
				}
				unmarkPending(id);
			},
		};
	}

	function set(items: T[]): void {
		listGeneration += 1;
		pendingCounts.clear();
		baseItems.clear();
		activeUpdates.clear();
		lastConfirmedOpId.clear();
		removedEntries.clear();
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
