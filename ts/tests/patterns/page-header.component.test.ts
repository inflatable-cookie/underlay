// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import PageHeaderHarness from "../fixtures/PageHeaderHarness.svelte";

describe("patterns/PageHeader.svelte", () => {
	it("renders basic title/subtitle without breadcrumbs or banner", () => {
		const view = render(PageHeaderHarness, {
			title: "Dashboard",
			subtitle: "Overview",
		});

		expect(view.container.querySelector(".underlay-page-header__title")?.textContent).toContain("Dashboard");
		expect(view.container.querySelector(".underlay-page-header__subtitle")?.textContent).toContain("Overview");
		expect(view.container.querySelector(".underlay-page-header__breadcrumbs")).toBeNull();
		expect(view.container.querySelector(".underlay-banner")).toBeNull();
		expect(view.container.querySelector(".underlay-page-header__back")).toBeNull();
	});

	it("renders section/title hierarchy, breadcrumbs, contextual back links, count, and banner", () => {
		const view = render(PageHeaderHarness, {
			section: "Users",
			title: "user profile",
			count: 42,
			breadcrumbs: [
				{ label: "Settings", href: "/settings" },
				{ label: "Users" },
			],
			backHref: "/settings/users",
			backLabel: "Back to user profile administration panel",
			backIsContextual: true,
			bannerMessage: "Scoped by context",
			bannerVariant: "info",
			level: 2,
		});

		const title = view.container.querySelector(".underlay-page-header__title");
		expect(title?.textContent).toContain("Users");
		expect(title?.textContent).toContain("42");

		const sectionTitle = view.container.querySelector(".underlay-page-header__section-title");
		expect(sectionTitle?.textContent).toContain("user profile");

		const crumbs = view.container.querySelectorAll(".underlay-page-header__breadcrumbs a, .underlay-page-header__breadcrumb-current");
		expect(crumbs.length).toBe(2);
		expect(view.container.querySelector(".underlay-page-header__breadcrumb-link")?.textContent).toContain("Settings");
		expect(view.container.querySelector(".underlay-page-header__breadcrumb-current")?.textContent).toContain("Users");

		const backLinks = view.container.querySelectorAll(".underlay-page-header__back");
		expect(backLinks.length).toBe(2);
		expect(backLinks[0]?.textContent).toContain("Back to User Profile Administratio");
		expect(backLinks[0]?.textContent).toContain("…");
		expect(view.container.querySelectorAll(".underlay-page-header__context-dot").length).toBe(2);

		const banner = view.container.querySelector(".underlay-banner");
		expect(banner).toBeTruthy();
		expect(banner?.classList.contains("underlay-banner--info")).toBe(true);
		expect(screen.getByText("Scoped by context")).toBeTruthy();
	});

	it("renders action/titleSuffix/subtitleSuffix/meta snippets", () => {
		const view = render(PageHeaderHarness, {
			title: "Title",
			subtitle: "Subtitle",
			withActions: true,
			withTitleSuffix: true,
			withSubtitleSuffix: true,
			withMeta: true,
		});

		expect(screen.getByTestId("ph-action").textContent).toContain("Header Action");
		expect(screen.getByTestId("ph-title-suffix").textContent).toContain("TS");
		expect(screen.getByTestId("ph-subtitle-suffix").textContent).toContain("SS");
		expect(screen.getByTestId("ph-meta").textContent).toContain("Meta content");
		expect(view.container.querySelector(".underlay-page-header__actions")).toBeTruthy();
		expect(view.container.querySelector(".underlay-page-header__meta")).toBeTruthy();
	});
});
