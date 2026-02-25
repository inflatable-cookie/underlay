// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import RelationPickerStatus from "../../src/patterns/relation-picker/RelationPickerStatus.svelte";

describe("patterns/relation-picker/RelationPickerStatus.svelte", () => {
	it("renders error and optional retry action", async () => {
		const onRetry = vi.fn();
		render(RelationPickerStatus, {
			error: "Fetch failed",
			onRetry,
			showLoading: false,
			showEmpty: false,
			searchQuery: "",
			emptyMessage: "No items",
		});

		expect(screen.getByText("Fetch failed")).toBeTruthy();
		const retry = screen.getByRole("button", { name: "Retry" });
		expect(retry).toBeTruthy();

		await fireEvent.click(retry);
		expect(onRetry).toHaveBeenCalledTimes(1);
	});

	it("renders loading state before empty state", () => {
		const view = render(RelationPickerStatus, {
			showLoading: true,
			showEmpty: true,
			searchQuery: "alpha",
			emptyMessage: "Nothing available",
		});

		expect(screen.getByText("Loading...")).toBeTruthy();
		expect(screen.queryByText("No matches found.")).toBeNull();
		expect(view.container.querySelector(".relation-picker-dialog__loading-spinner")).toBeTruthy();
	});

	it("renders empty copy based on whether search query is present", () => {
		const queryEmpty = render(RelationPickerStatus, {
			showLoading: false,
			showEmpty: true,
			searchQuery: "  beta  ",
			emptyMessage: "Nothing available",
		});
		expect(screen.getByText("No matches found.")).toBeTruthy();
		queryEmpty.unmount();

		render(RelationPickerStatus, {
			showLoading: false,
			showEmpty: true,
			searchQuery: "   ",
			emptyMessage: "Nothing available",
		});
		expect(screen.getByText("Nothing available")).toBeTruthy();
	});
});
