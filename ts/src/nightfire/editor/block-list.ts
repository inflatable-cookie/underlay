interface NightfireBlock {
	type: string;
	version?: string;
	hash?: string;
	data?: Record<string, unknown>;
}

export function createDefaultBlock(defaultType: string): NightfireBlock {
	return {
		type: defaultType,
		version: "initial",
		hash: "",
		data: {}
	};
}

export function addBlockToList(
	blocks: unknown[],
	defaultType: string
): unknown[] {
	return blocks.concat(createDefaultBlock(defaultType));
}

export function removeBlockFromList(
	blocks: unknown[],
	index: number
): unknown[] {
	const next = blocks.slice();
	next.splice(index, 1);
	return next;
}

export function moveBlockInList(
	blocks: unknown[],
	from: number,
	to: number
): unknown[] | null {
	if (from === to || from < 0 || to < 0 || from >= blocks.length || to >= blocks.length) {
		return null;
	}

	const next = blocks.slice();
	const [moved] = next.splice(from, 1);
	next.splice(to, 0, moved);
	return next;
}
