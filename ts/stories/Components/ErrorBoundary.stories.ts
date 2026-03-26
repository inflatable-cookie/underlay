import type { Meta, StoryObj } from "@storybook/svelte-vite";

import ErrorBoundaryDemo from "../support/ErrorBoundaryDemo.svelte";

const meta = {
  title: "Components/ErrorBoundary",
  component: ErrorBoundaryDemo,
  tags: ["autodocs"]
} satisfies Meta<ErrorBoundaryDemo>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};
