import type { Meta, StoryObj } from "@storybook/svelte-vite";

import MediaPickerDemo from "../support/MediaPickerDemo.svelte";

const meta = {
  title: "Components/MediaPicker",
  component: MediaPickerDemo,
  tags: ["autodocs"]
} satisfies Meta<MediaPickerDemo>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};
