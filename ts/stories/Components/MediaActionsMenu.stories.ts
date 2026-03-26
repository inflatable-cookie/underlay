import type { Meta, StoryObj } from "@storybook/svelte-vite";

import MediaActionsMenuDemo from "../support/MediaActionsMenuDemo.svelte";

const meta = {
  title: "Components/MediaActionsMenu",
  component: MediaActionsMenuDemo,
  tags: ["autodocs"]
} satisfies Meta<MediaActionsMenuDemo>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};
