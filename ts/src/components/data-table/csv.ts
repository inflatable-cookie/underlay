interface CsvColumn<T> {
	key: string;
	label: string;
	formatter?: (value: unknown, row: T) => string;
}

export function exportRowsToCsv<T extends object>(
	data: T[],
	columns: CsvColumn<T>[],
	filename = "export.csv"
): void {
	const headers = columns.map((column) => escapeCsvValue(column.label));

	const rows = data.map((row) => {
		return columns.map((column) => {
			const value = getNestedValue(row, column.key);
			const formatted = column.formatter ? column.formatter(value, row) : String(value ?? "");
			return escapeCsvValue(formatted);
		});
	});

	const csvContent = [headers.join(","), ...rows.map((row) => row.join(","))].join("\n");

	const blob = new Blob([csvContent], { type: "text/csv;charset=utf-8;" });
	const url = URL.createObjectURL(blob);
	const doc = globalThis?.document;
	if (!doc) return;

	const link = doc.createElement("a");
	link.setAttribute("href", url);
	link.setAttribute("download", filename);
	link.style.visibility = "hidden";
	doc.body.appendChild(link);
	link.click();
	doc.body.removeChild(link);
	URL.revokeObjectURL(url);
}

function escapeCsvValue(value: string): string {
	if (value.includes(",") || value.includes("\n") || value.includes('"')) {
		return `"${value.replace(/"/g, '""')}"`;
	}
	return value;
}

function getNestedValue(obj: object, path: string): unknown {
	const keys = path.split(".");
	let value: unknown = obj;
	for (const key of keys) {
		if (value && typeof value === "object") {
			value = (value as Record<string, unknown>)[key];
		} else {
			return undefined;
		}
	}
	return value;
}
