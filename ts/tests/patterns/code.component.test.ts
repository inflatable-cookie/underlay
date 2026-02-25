// @vitest-environment jsdom
import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import CodeHarness from "../fixtures/CodeHarness.svelte";

describe("components/Code.svelte", () => {
	beforeEach(() => {
		vi.restoreAllMocks();
	});

	it("renders code element/class/attrs and hides copy control by default", () => {
		const view = render(CodeHarness, {
			text: "const value = 1",
			copy: false,
			className: "custom-code",
			dataTestId: "my-code",
		});

		const code = screen.getByTestId("my-code");
		expect(code.tagName.toLowerCase()).toBe("code");
		expect(code.classList.contains("underlay-code")).toBe(true);
		expect(code.classList.contains("custom-code")).toBe(true);
		expect(code.textContent).toContain("const value = 1");
		expect(view.container.querySelector(".underlay-code-copy")).toBeNull();
	});

	it("copies trimmed code text when copy button is pressed", async () => {
		const writeText = vi.fn(async () => undefined);
		Object.defineProperty(globalThis.navigator, "clipboard", {
			value: { writeText },
			configurable: true,
		});
		render(CodeHarness, { text: "  hello world  ", copy: true });

		await fireEvent.click(screen.getByRole("button", { name: "Copy code" }));
		expect(writeText).toHaveBeenCalledWith("hello world");
	});

	it("does not attempt clipboard write for empty code content", async () => {
		const writeText = vi.fn(async () => undefined);
		Object.defineProperty(globalThis.navigator, "clipboard", {
			value: { writeText },
			configurable: true,
		});
		render(CodeHarness, { text: "   ", copy: true });

		await fireEvent.click(screen.getByRole("button", { name: "Copy code" }));
		expect(writeText).not.toHaveBeenCalled();
	});
});
