// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import ListContainerHarness from "../fixtures/ListContainerHarness.svelte";

describe("components/ListContainer.svelte", () => {
	it("renders loading state", () => {
		render(ListContainerHarness, {
			title: "People",
			loading: true,
		});

		expect(screen.getByText("People")).toBeTruthy();
		expect(screen.getByText("Loading...")).toBeTruthy();
		expect(screen.queryByTestId("list-content")).toBeNull();
	});

	it("renders error state", () => {
		render(ListContainerHarness, {
			title: "People",
			error: "Load failed",
		});

		expect(screen.getByText("Load failed")).toBeTruthy();
		expect(screen.queryByText("Loading...")).toBeNull();
	});

	it("renders full content state with filters/batch/content/pagination snippets", () => {
		const view = render(ListContainerHarness, {
			title: "People",
			variant: "tab",
			hasItems: true,
			withActions: true,
			withFilters: true,
			withBatchBar: true,
			withContent: true,
			withPagination: true,
		});

		expect(screen.getByTestId("list-actions")).toBeTruthy();
		expect(screen.getByTestId("list-filters")).toBeTruthy();
		expect(screen.getByTestId("list-batch")).toBeTruthy();
		expect(screen.getByTestId("list-content")).toBeTruthy();
		expect(screen.getByTestId("list-pagination")).toBeTruthy();
		expect(view.container.querySelector(".underlay-list-container--tab")).toBeTruthy();
	});

	it("renders empty state and hides filters/pagination when no items", () => {
		render(ListContainerHarness, {
			title: "People",
			hasItems: false,
			emptyMessage: "No users yet.",
			withFilters: true,
			withPagination: true,
			withContent: true,
		});

		expect(screen.getByText("No users yet.")).toBeTruthy();
		expect(screen.queryByTestId("list-filters")).toBeNull();
		expect(screen.queryByTestId("list-pagination")).toBeNull();
		expect(screen.queryByTestId("list-content")).toBeNull();
	});
});
