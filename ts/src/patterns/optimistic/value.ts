import { writable, get, type Readable, type Writable } from "svelte/store";
import { createNoopOperation } from "../optimistic/helpers";

export interface OptimisticUpdateOperation<T> {
	confirm: (realItem?: T) => void;
	rollback: () => void;
}

export interface OptimisticValueOptions<T> {
	equals?: (a: T, b: T) => boolean;
}

export interface OptimisticValue<T> extends Readable<T> {
	set: (value: T) => OptimisticUpdateOperation<T>;
	pending: Readable<boolean>;
}

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
