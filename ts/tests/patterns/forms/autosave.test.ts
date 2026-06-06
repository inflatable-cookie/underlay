// @vitest-environment jsdom
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { createFormState } from "../../../src/patterns/forms";
import { createMemoryDraftStorage } from "./fixtures";

describe("createFormState.enhance autosave", () => {
	beforeEach(() => {
		vi.useFakeTimers();
	});

	afterEach(() => {
		vi.useRealTimers();
		vi.restoreAllMocks();
	});

	it("restores saved drafts into real form controls when auto-save is enabled", () => {
		const draftStorage = createMemoryDraftStorage(
			new Map([
				[
					"profile-draft",
					{
						name: { kind: "single", value: "Alice" },
						newsletter: { kind: "boolean", checked: true },
						role: { kind: "single", value: "admin" },
						tags: { kind: "multi", values: ["billing", "ops"] }
					}
				]
			])
		);
		const form = createFormState({
			autoSave: {
				key: "profile-draft",
				storage: draftStorage.wrapper
			}
		});
		const formEl = document.createElement("form");
		formEl.innerHTML = `
			<input name="name" value="" />
			<input type="checkbox" name="newsletter" />
			<input type="radio" name="role" value="user" />
			<input type="radio" name="role" value="admin" />
			<input type="checkbox" name="tags" value="billing" />
			<input type="checkbox" name="tags" value="ops" />
		`;

		form.enhance(formEl);

		expect((formEl.elements.namedItem("name") as HTMLInputElement).value).toBe("Alice");
		expect((formEl.elements.namedItem("newsletter") as HTMLInputElement).checked).toBe(true);
		expect((formEl.querySelector('input[name="role"][value="user"]') as HTMLInputElement).checked).toBe(false);
		expect((formEl.querySelector('input[name="role"][value="admin"]') as HTMLInputElement).checked).toBe(true);
		expect((formEl.querySelector('input[name="tags"][value="billing"]') as HTMLInputElement).checked).toBe(true);
		expect((formEl.querySelector('input[name="tags"][value="ops"]') as HTMLInputElement).checked).toBe(true);
	});

	it("debounces draft writes and clears the saved draft on success", () => {
		const draftStorage = createMemoryDraftStorage();
		const form = createFormState({
			autoSave: {
				key: "profile-draft",
				storage: draftStorage.wrapper,
				debounce: 100
			}
		});
		const formEl = document.createElement("form");
		formEl.innerHTML = `
			<input name="name" value="" />
			<input type="checkbox" name="newsletter" />
			<input type="file" name="avatar" />
		`;
		const nameInput = formEl.querySelector('input[name="name"]') as HTMLInputElement;
		const newsletterInput = formEl.querySelector(
			'input[name="newsletter"]'
		) as HTMLInputElement;

		form.enhance(formEl);

		nameInput.value = "Bea";
		nameInput.dispatchEvent(new Event("input", { bubbles: true }));
		newsletterInput.checked = true;
		newsletterInput.dispatchEvent(new Event("change", { bubbles: true }));

		expect(draftStorage.wrapper.set).not.toHaveBeenCalled();
		vi.advanceTimersByTime(99);
		expect(draftStorage.wrapper.set).not.toHaveBeenCalled();

		vi.advanceTimersByTime(1);
		expect(draftStorage.wrapper.set).toHaveBeenCalledTimes(1);
		expect(draftStorage.data.get("profile-draft")).toEqual({
			name: { kind: "single", value: "Bea" },
			newsletter: { kind: "boolean", checked: true }
		});

		form.setSuccess();
		expect(draftStorage.wrapper.remove).toHaveBeenCalledWith("profile-draft");
		expect(draftStorage.data.has("profile-draft")).toBe(false);
	});

	it("keeps drafts after success when clearOnSuccess is disabled", () => {
		const draftStorage = createMemoryDraftStorage();
		const form = createFormState({
			autoSave: {
				key: "profile-draft",
				storage: draftStorage.wrapper,
				clearOnSuccess: false
			}
		});
		const formEl = document.createElement("form");
		formEl.innerHTML = `<input name="name" value="Sam" />`;
		const input = formEl.querySelector("input") as HTMLInputElement;

		form.enhance(formEl);
		input.dispatchEvent(new Event("input", { bubbles: true }));
		vi.advanceTimersByTime(600);

		form.setSuccess();

		expect(draftStorage.data.get("profile-draft")).toEqual({
			name: { kind: "single", value: "Sam" }
		});
		expect(draftStorage.wrapper.remove).not.toHaveBeenCalled();
	});
});
