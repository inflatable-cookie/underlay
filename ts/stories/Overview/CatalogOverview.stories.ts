import type { Meta, StoryObj } from "@storybook/svelte-vite";

import CatalogOverview from "../support/CatalogOverview.svelte";

const meta = {
  title: "Overview/Catalog Boundary",
  component: CatalogOverview,
  tags: ["autodocs"]
} satisfies Meta<CatalogOverview>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Overview: Story = {};
