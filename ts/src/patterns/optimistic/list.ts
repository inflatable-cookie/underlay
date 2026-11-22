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

interface TrackedRow<T> {
	base: T;
	currentKey: string;
	ops: ActiveUpdateEntry<T>[];
	lastConfirmedOpId: number;
}

interface RemovedEntry<T> {
	item: T;
	index: number;
	row?: TrackedRow<T>;
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
	const trackedRows = new Map<string, TrackedRow<T>>();
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

	function movePendingCount(fromKey: string, toKey: string, delta: number): void {
		if (fromKey === toKey || delta <= 0) return;
		const fromCount = pendingCounts.get(fromKey) ?? 0;
		const moved = Math.min(fromCount, delta);
		if (moved <= 0) return;

		if (fromCount - moved <= 0) {
			pendingCounts.delete(fromKey);
			pending.update((p) => withoutSetValue(p, fromKey));
		} else {
			pendingCounts.set(fromKey, fromCount - moved);
		}

		const toCount = (pendingCounts.get(toKey) ?? 0) + moved;
		pendingCounts.set(toKey, toCount);
		if (toCount === moved) {
			pending.update((p) => withSetValue(p, toKey));
		}
	}

	function recomputeRow(row: TrackedRow<T>): T {
		let result: T = { ...row.base };
		for (const op of row.ops) {
			result = { ...result, ...op.changes };
		}
		return result;
	}

	function syncRowState(
		row: TrackedRow<T>,
		prevKey: string,
		nextItem: T,
		keepTracked: boolean,
	): void {
		const nextKey = getKey(nextItem);
		const removed = removedEntries.get(prevKey);
		if (removed) {
			removed.item = nextItem;
			if (nextKey !== prevKey) {
				removedEntries.delete(prevKey);
				removedEntries.set(nextKey, removed);
			}
		}

		if (prevKey !== nextKey) {
			if (trackedRows.get(prevKey) === row) {
				trackedRows.delete(prevKey);
			}
			row.currentKey = nextKey;
			if (keepTracked) {
				trackedRows.set(nextKey, row);
			}
			// Move pending counts for all still-active ops on this row to the new key.
			movePendingCount(prevKey, nextKey, row.ops.length);
		} else if (!keepTracked && trackedRows.get(prevKey) === row) {
			trackedRows.delete(prevKey);
		}

		store.update((items) => items.map((i) => (getKey(i) === prevKey ? nextItem : i)));
	}

	function add(item: Partial<T>): OptimisticAddOperation<T> {
		const id = tempId();
		const optimisticItem =
			keyField !== "id" && !("id" in item)
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
				const row = trackedRows.get(id);
				removedEntry = { item: items[removedIndex], index: removedIndex, row };
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
				const currentKey = removedEntry?.row?.currentKey ?? id;
				if (removedEntries.get(currentKey) === removedEntry) {
					removedEntries.delete(currentKey);
				}
				if (removedEntry?.row) {
					removedEntry.row.ops = [];
					if (trackedRows.get(currentKey) === removedEntry.row) {
						trackedRows.delete(currentKey);
					}
				}
				unmarkPending(id);
			},
			rollback: () => {
				if (settled) return;
				settled = true;
				if (removedEntry) {
					const currentKey =
						generation === listGeneration ? (removedEntry.row?.currentKey ?? id) : id;
					const itemToRestore =
						generation === listGeneration && removedEntry.row && removedEntry.row.ops.length > 0
							? recomputeRow(removedEntry.row)
							: removedEntry.item;
					const restoreKey = getKey(itemToRestore);
					const restoreIndex = removedEntry.index;
					if (removedEntries.get(currentKey) === removedEntry) {
						removedEntries.delete(currentKey);
					}
					store.update((items) => {
						if (items.some((i) => getKey(i) === restoreKey)) {
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
		let trackedRow: TrackedRow<T> | undefined;
		let settled = false;

		store.update((items) =>
			items.map((i) => {
				if (getKey(i) === id) {
					const existing = trackedRows.get(id);
					if (existing) {
						trackedRow = existing;
						existing.ops.push({ opId, changes: { ...changes } });
					} else {
						trackedRow = {
							base: i,
							currentKey: id,
							ops: [{ opId, changes: { ...changes } }],
							lastConfirmedOpId: 0,
						};
						trackedRows.set(id, trackedRow);
					}

					const nextItem = recomputeRow(trackedRow);
					const nextKey = getKey(nextItem);
					if (nextKey !== id) {
						if (trackedRows.get(id) === trackedRow) {
							trackedRows.delete(id);
						}
						trackedRow.currentKey = nextKey;
						trackedRows.set(nextKey, trackedRow);
						// Move existing pending ops (excluding this new op, marked below) to nextKey.
						movePendingCount(id, nextKey, trackedRow.ops.length - 1);
					}
					return nextItem;
				}
				return i;
			}),
		);

		markPending(trackedRow ? trackedRow.currentKey : id);

		return {
			confirm: (realItem?: T) => {
				if (settled || generation !== listGeneration) return;
				settled = true;

				if (trackedRow) {
					const row = trackedRow;
					const prevKey = row.currentKey;
					const currentOp = row.ops.find((op) => op.opId === opId);
					const isOlderThanConfirmed = opId < row.lastConfirmedOpId;

					if (opId > row.lastConfirmedOpId) {
						row.lastConfirmedOpId = opId;
					}

					if (currentOp) {
						const remaining = row.ops.filter((op) => op.opId !== opId);
						if (realItem && !isOlderThanConfirmed) {
							row.base = realItem;
							// Older uncommitted ops are superseded by the server snapshot.
							const newerOnly = remaining.filter((op) => op.opId > opId);
							row.ops = newerOnly;
							const nextItem =
								newerOnly.length > 0
									? newerOnly.reduce((acc, op) => ({ ...acc, ...op.changes }), { ...realItem })
									: realItem;
							unmarkPending(prevKey);
							syncRowState(row, prevKey, nextItem, newerOnly.length > 0);
							return;
						} else {
							// Apply only non-superseded fields from currentOp.changes so
							// confirming an older op (with or without a stale snapshot)
							// never overwrites a newer confirmed value.
							row.base = { ...row.base, ...currentOp.changes };
							const confirmedKeys = Object.keys(currentOp.changes) as Array<keyof T>;
							for (const op of remaining) {
								if (op.opId < opId) {
									for (const k of confirmedKeys) {
										delete op.changes[k];
									}
								}
							}
							row.ops = remaining;
							const nextItem = remaining.length > 0 ? recomputeRow(row) : row.base;
							unmarkPending(prevKey);
							syncRowState(row, prevKey, nextItem, remaining.length > 0);
							return;
						}
					}
					unmarkPending(prevKey);
					return;
				}
				unmarkPending(id);
			},
			rollback: () => {
				if (settled || generation !== listGeneration) return;
				settled = true;

				if (trackedRow) {
					const row = trackedRow;
					const prevKey = row.currentKey;
					if (row.ops.some((op) => op.opId === opId)) {
						row.ops = row.ops.filter((op) => op.opId !== opId);
						const nextItem = row.ops.length > 0 ? recomputeRow(row) : row.base;
						unmarkPending(prevKey);
						syncRowState(row, prevKey, nextItem, row.ops.length > 0);
						return;
					}
					unmarkPending(prevKey);
					return;
				}
				unmarkPending(id);
			},
		};
	}

	function set(items: T[]): void {
		listGeneration += 1;
		pendingCounts.clear();
		trackedRows.clear();
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
