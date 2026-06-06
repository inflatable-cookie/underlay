import { describe, expect, it } from "vitest";
import { writeNightfireToFormData } from "../../../src/nightfire/utils";

describe("writeNightfireToFormData", () => {
	it("writes empty string for null value", () => {
		const formData = new FormData();
		writeNightfireToFormData(formData, "body", null);
		expect(formData.get("body")).toBe("");
	});

	it("writes empty string for undefined value", () => {
		const formData = new FormData();
		writeNightfireToFormData(formData, "body", undefined);
		expect(formData.get("body")).toBe("");
	});

	it("writes empty string for empty NightfireValue", () => {
		const formData = new FormData();
		writeNightfireToFormData(formData, "body", { schema: "test@1" });
		expect(formData.get("body")).toBe("");
	});

	it("writes JSON string for non-empty value", () => {
		const formData = new FormData();
		const value = {
			schema: "test@1",
			block: { type: "markdown", version: "initial", hash: "", data: { text: "Hello" } },
		};
		writeNightfireToFormData(formData, "body", value);

		const written = formData.get("body") as string;
		expect(JSON.parse(written)).toEqual(value);
	});

	it("overwrites existing form field", () => {
		const formData = new FormData();
		formData.set("body", "old value");

		const value = {
			schema: "test@1",
			block: { type: "markdown", version: "initial", hash: "", data: {} },
		};
		writeNightfireToFormData(formData, "body", value);

		expect(formData.get("body")).not.toBe("old value");
	});

	it("uses provided field name", () => {
		const formData = new FormData();
		const value = {
			schema: "test@1",
			block: { type: "markdown", version: "initial", hash: "", data: {} },
		};
		writeNightfireToFormData(formData, "customField", value);

		expect(formData.get("customField")).not.toBeNull();
		expect(formData.get("body")).toBeNull();
	});
});
