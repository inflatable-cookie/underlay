import { describe, it, expect, vi, afterEach } from "vitest";
import {
	createStableId,
	requestSubmitById,
	submitFormWithIntent
} from "../../src/patterns/dom";

const originalDocumentDescriptor = Object.getOwnPropertyDescriptor(globalThis, "document");

function setDocument(value: unknown): void {
	Object.defineProperty(globalThis, "document", {
		configurable: true,
		value
	});
}

afterEach(() => {
	vi.restoreAllMocks();
	if (originalDocumentDescriptor) {
		Object.defineProperty(globalThis, "document", originalDocumentDescriptor);
	}
});

describe("createStableId", () => {
	it("creates deterministic incrementing ids", () => {
		const first = createStableId("field");
		const second = createStableId("field");
		expect(first).toMatch(/^field-\d+$/);
		expect(second).toMatch(/^field-\d+$/);
		expect(first).not.toBe(second);
	});
});

describe("requestSubmitById", () => {
	it("no-ops when document is unavailable", () => {
		setDocument(undefined);
		expect(() => requestSubmitById("x")).not.toThrow();
	});

	it("calls requestSubmit when target supports it", () => {
		const requestSubmit = vi.fn();
		setDocument({
			getElementById: vi.fn(() => ({ requestSubmit }))
		});

		requestSubmitById("my-form");
		expect(requestSubmit).toHaveBeenCalledOnce();
	});

	it("no-ops when target has no requestSubmit function", () => {
		setDocument({
			getElementById: vi.fn(() => ({ requestSubmit: "not-a-fn" }))
		});
		expect(() => requestSubmitById("my-form")).not.toThrow();
	});
});

describe("submitFormWithIntent", () => {
	it("no-ops when document/form is unavailable", () => {
		setDocument(undefined);
		expect(() => submitFormWithIntent("delete")).not.toThrow();

		setDocument({
			querySelector: vi.fn(() => null)
		});
		expect(() => submitFormWithIntent("delete")).not.toThrow();
	});

	it("sets intent input value and submits form", () => {
		const input = { value: "" };
		const requestSubmit = vi.fn();
		const formElement = {
			querySelector: vi.fn(() => input),
			requestSubmit
		};
		setDocument({
			querySelector: vi.fn(() => formElement)
		});

		submitFormWithIntent("archive");

		expect(input.value).toBe("archive");
		expect(requestSubmit).toHaveBeenCalledOnce();
	});

	it("submits form even when intent input is missing", () => {
		const requestSubmit = vi.fn();
		const formElement = {
			querySelector: vi.fn(() => null),
			requestSubmit
		};
		setDocument({
			querySelector: vi.fn(() => formElement)
		});

		submitFormWithIntent("save", "#main", "action");
		expect(formElement.querySelector).toHaveBeenCalledWith('input[name="action"]');
		expect(requestSubmit).toHaveBeenCalledOnce();
	});
});
