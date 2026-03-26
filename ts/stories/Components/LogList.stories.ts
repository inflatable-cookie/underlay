import type { Meta, StoryObj } from "@storybook/svelte-vite";

import LogListDemo from "../support/LogListDemo.svelte";

const meta = {
  title: "Components/LogList",
  component: LogListDemo,
  tags: ["autodocs"]
} satisfies Meta<LogListDemo>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};
