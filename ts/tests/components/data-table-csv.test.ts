import { describe, expect, it, vi } from "vitest";
import { exportRowsToCsv } from "../../src/components/data-table/csv";

describe("components/data-table/csv", () => {
	it("exports rows as csv and clicks download link", () => {
		const originalDocument = globalThis.document;
		const originalUrl = globalThis.URL;

		const click = vi.fn();
		const setAttribute = vi.fn();
		const appendChild = vi.fn();
		const removeChild = vi.fn();
		const createElement = vi.fn(() => ({
			setAttribute,
			click,
			style: { visibility: "" },
		}));
		(globalThis as any).document = {
			createElement,
			body: { appendChild, removeChild },
		};
		(globalThis as any).URL = {
			createObjectURL: vi.fn(() => "blob:123"),
			revokeObjectURL: vi.fn(),
		};

		exportRowsToCsv(
			[{ user: { name: 'Clay "A"' }, status: "active" }],
			[
				{ key: "user.name", label: "Name" },
				{ key: "status", label: "Status", formatter: (v) => String(v).toUpperCase() },
			],
			"users.csv"
		);

		expect(createElement).toHaveBeenCalledWith("a");
		expect(setAttribute).toHaveBeenNthCalledWith(1, "href", "blob:123");
		expect(setAttribute).toHaveBeenNthCalledWith(2, "download", "users.csv");
		expect(click).toHaveBeenCalledOnce();
		expect((globalThis as any).URL.revokeObjectURL).toHaveBeenCalledWith("blob:123");

		(globalThis as any).document = undefined;
		expect(() =>
			exportRowsToCsv([{ x: 1 }], [{ key: "x", label: "X" }], "x.csv")
		).not.toThrow();

		(globalThis as any).document = originalDocument;
		(globalThis as any).URL = originalUrl;
	});

	it("handles nested lookup through non-object values", () => {
		const originalDocument = globalThis.document;
		const originalUrl = globalThis.URL;

		const click = vi.fn();
		(globalThis as any).document = {
			createElement: vi.fn(() => ({
				setAttribute: vi.fn(),
				click,
				style: { visibility: "" },
			})),
			body: { appendChild: vi.fn(), removeChild: vi.fn() },
		};
		(globalThis as any).URL = {
			createObjectURL: vi.fn(() => "blob:456"),
			revokeObjectURL: vi.fn(),
		};

		expect(() =>
			exportRowsToCsv(
				[{ user: { name: "clay" } }],
				[{ key: "user.name.first", label: "First" }],
				"users.csv"
			)
		).not.toThrow();
		expect(click).toHaveBeenCalledOnce();

		(globalThis as any).document = originalDocument;
		(globalThis as any).URL = originalUrl;
	});
});
