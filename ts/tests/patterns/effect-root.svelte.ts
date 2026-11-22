export function createEffectRoot(fn: () => void): () => void {
	return $effect.root(fn);
}

export function createReactiveBox<T>(initial: T): { value: T } {
	let value = $state(initial);
	return {
		get value() {
			return value;
		},
		set value(next: T) {
			value = next;
		},
	};
}
