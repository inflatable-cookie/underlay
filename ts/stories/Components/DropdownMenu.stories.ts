import type { Meta, StoryObj } from "@storybook/svelte-vite";

import DropdownMenuDemo from "../support/DropdownMenuDemo.svelte";

const meta = {
  title: "Components/DropdownMenu",
  component: DropdownMenuDemo,
  tags: ["autodocs"]
} satisfies Meta<DropdownMenuDemo>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};
