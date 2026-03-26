import type { Meta, StoryObj } from "@storybook/svelte-vite";

import CopyActionsMenuDemo from "../support/CopyActionsMenuDemo.svelte";

const meta = {
  title: "Patterns/CopyActionsMenu",
  component: CopyActionsMenuDemo,
  tags: ["autodocs"]
} satisfies Meta<CopyActionsMenuDemo>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};
