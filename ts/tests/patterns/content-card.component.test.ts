// @vitest-environment jsdom
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import ContentCard from "../../src/components/ContentCard.svelte";

describe("components/ContentCard.svelte", () => {
	let originalResizeObserver: typeof ResizeObserver | undefined;

	beforeEach(() => {
		vi.restoreAllMocks();
		originalResizeObserver = globalThis.ResizeObserver;
		class ResizeObserverMock {
			observe() {
				return undefined;
			}
			disconnect() {
				return undefined;
			}
		}
		// @ts-expect-error test shim
		globalThis.ResizeObserver = ResizeObserverMock;
	});

	afterEach(() => {
		globalThis.ResizeObserver = originalResizeObserver as typeof ResizeObserver;
	});

	it("renders title and empty message when no content", () => {
		const view = render(ContentCard, {
			title: "Overview",
			value: "",
			emptyMessage: "Nothing set.",
		});

		expect(screen.getByText("Overview")).toBeTruthy();
		expect(screen.getByText("Nothing set.")).toBeTruthy();
		expect(view.container.querySelector(".underlay-content-card__empty")).toBeTruthy();
	});

	it("renders sanitized string content and supports markdown mode", () => {
		const first = render(ContentCard, {
			value: "<script>alert(1)</script><p>Safe</p>",
			markdown: false,
		});
		expect(screen.getByText("Safe")).toBeTruthy();
		expect(first.container.querySelector("script")).toBeNull();
		first.unmount();

		render(ContentCard, {
			value: "# Heading",
			markdown: true,
		});
		expect(screen.getByText("Heading")).toBeTruthy();
	});

	it("applies scroll overflow class and max-height custom property", () => {
		const view = render(ContentCard, {
			value: "<p>Content</p>",
			overflowBehavior: "scroll",
			maxHeight: 150,
		});

		const body = view.container.querySelector(".underlay-content-card__body") as HTMLElement;
		expect(body.classList.contains("underlay-content-card__body--scroll")).toBe(true);
		expect(body.getAttribute("style")).toContain("--content-card-max-height: 150px;");
	});

	it("shows reveal toggle when overflowing and toggles collapsed class", async () => {
		const originalScrollHeight = Object.getOwnPropertyDescriptor(HTMLElement.prototype, "scrollHeight");
		Object.defineProperty(HTMLElement.prototype, "scrollHeight", {
			configurable: true,
			get() {
				return 500;
			},
		});

		let callback: ResizeObserverCallback | null = null;
		class ResizeObserverMock {
			constructor(cb: ResizeObserverCallback) {
				callback = cb;
			}
			observe() {
				if (callback) callback([], this as unknown as ResizeObserver);
			}
			disconnect() {
				return undefined;
			}
		}
		// @ts-expect-error test shim
		globalThis.ResizeObserver = ResizeObserverMock;

		try {
			const view = render(ContentCard, {
				value: "<p>Long content</p>",
				overflowBehavior: "reveal",
				maxHeight: 100,
			});

			const body = view.container.querySelector(".underlay-content-card__body") as HTMLElement;
			callback?.([], {} as ResizeObserver);

			const toggle = screen.getByRole("button", { name: "Show more" });
			expect(toggle).toBeTruthy();
			expect(body.classList.contains("underlay-content-card__body--collapsed")).toBe(true);

			await fireEvent.click(toggle);
			expect(screen.getByRole("button", { name: "Show less" })).toBeTruthy();
			expect(body.classList.contains("underlay-content-card__body--collapsed")).toBe(false);
		} finally {
			if (originalScrollHeight) {
				Object.defineProperty(HTMLElement.prototype, "scrollHeight", originalScrollHeight);
			}
		}
	});
});
