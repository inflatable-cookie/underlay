import { writable, get, type Readable, type Writable } from "svelte/store";
import { createNoopOperation } from "../optimistic/helpers";

export interface OptimisticOperation {
	confirm: (data?: unknown) => void;
	rollback: () => void;
}

export interface OptimisticUpdateOperation<T> {
	confirm: (realItem?: T) => void;
	rollback: () => void;
}

export interface OptimisticCounter extends Readable<number> {
	increment: (amount?: number) => OptimisticOperation;
	decrement: (amount?: number) => OptimisticOperation;
	set: (value: number) => OptimisticUpdateOperation<number>;
	pending: Readable<boolean>;
}

export function createOptimisticCounter(initial: number = 0): OptimisticCounter {
	const store: Writable<number> = writable(initial);
	const pending: Writable<boolean> = writable(false);

	function increment(amount: number = 1): OptimisticOperation {
		const previous = get(store);
		store.set(previous + amount);
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

	function decrement(amount: number = 1): OptimisticOperation {
		return increment(-amount);
	}

	function set(value: number): OptimisticUpdateOperation<number> {
		const previous = get(store);

		if (previous === value) {
			return createNoopOperation();
		}

		store.set(value);
		pending.set(true);

		return {
			confirm: (realValue?: number) => {
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
		increment,
		decrement,
		set,
		pending: { subscribe: pending.subscribe }
	};
}
