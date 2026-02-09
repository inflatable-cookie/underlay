export interface NightfireBlockOptionInput {
	type: string;
	label: string;
	category?: string;
}

export interface GroupedOptions {
	category: string | null;
	label: string;
	options: NightfireBlockOptionInput[];
}

export function buildGroupedOptions(
	options: NightfireBlockOptionInput[]
): GroupedOptions[] {
	const groups = new Map<string | null, NightfireBlockOptionInput[]>();

	for (const option of options) {
		const key = option.category ?? null;
		const existing = groups.get(key) ?? [];
		existing.push(option);
		groups.set(key, existing);
	}

	const sortedKeys = Array.from(groups.keys()).sort((a, b) => {
		const labelA = (a ?? "").toLowerCase();
		const labelB = (b ?? "").toLowerCase();
		if (labelA < labelB) return -1;
		if (labelA > labelB) return 1;
		return 0;
	});

	return sortedKeys.map((key) => {
		const grouped = groups.get(key) ?? [];
		grouped.sort((a, b) => a.label.localeCompare(b.label));
		return {
			category: key,
			label: key ?? "Other",
			options: grouped
		};
	});
}
