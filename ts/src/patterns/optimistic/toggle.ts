import { writable, get, type Readable, type Writable } from "svelte/store";
import { createNoopOperation } from "../optimistic/helpers";

export interface OptimisticOperation {
	confirm: (data?: unknown) => void;
	rollback: () => void;
}

export interface OptimisticToggle extends Readable<boolean> {
	toggle: () => OptimisticOperation;
	set: (value: boolean) => OptimisticOperation;
	pending: Readable<boolean>;
}

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
