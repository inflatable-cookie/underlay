import type { Meta, StoryObj } from "@storybook/svelte-vite";

import PageHeader from "../../src/patterns/PageHeader.svelte";

const breadcrumbs = [
  { label: "System", href: "/system" },
  { label: "Projects", href: "/projects" }
];

const meta = {
  title: "Patterns/PageHeader",
  component: PageHeader,
  tags: ["autodocs"],
  args: {
    title: "Projects",
    subtitle: "Shared admin workflow overview",
    level: 1
  }
} satisfies Meta<PageHeader>;

export default meta;

type Story = StoryObj<typeof meta>;

export const SimpleTitle: Story = {};

export const WithCountAndBackLink: Story = {
  args: {
    count: 24,
    backHref: "/system",
    backLabel: "Back to system"
  }
};

export const SectionSplitWithBanner: Story = {
  args: {
    section: "System",
    title: "Import Jobs",
    subtitle: "Operational detail header that still belongs to Underlay",
    breadcrumbs,
    bannerMessage: "One queued import needs manual review",
    bannerVariant: "warning"
  }
};
