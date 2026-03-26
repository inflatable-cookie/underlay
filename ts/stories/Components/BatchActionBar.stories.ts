import type { Meta, StoryObj } from "@storybook/svelte-vite";

import BatchActionBarDemo from "../support/BatchActionBarDemo.svelte";

const meta = {
  title: "Components/BatchActionBar",
  component: BatchActionBarDemo,
  tags: ["autodocs"]
} satisfies Meta<BatchActionBarDemo>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};
