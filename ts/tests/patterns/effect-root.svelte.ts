export function createEffectRoot(fn: () => void): () => void {
	return $effect.root(fn);
}
