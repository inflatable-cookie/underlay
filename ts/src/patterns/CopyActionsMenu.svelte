<script lang="ts">
  import DropdownMenu from "../components/DropdownMenu.svelte";
  import { copyToClipboard } from "./clipboard";
  import type { ToastStore } from "./toasts";

  type DropdownMenuItem = {
    label?: string;
    onSelect?: (() => void) | undefined;
    disabled?: boolean;
    destructive?: boolean;
    separator?: boolean;
  };

  type CopyItem = {
    label: string;
    text: string;
    successMessage: string;
    failureMessage?: string;
  };

  interface Props {
    toastStore: ToastStore;

    copies?: CopyItem[];
    actions?: DropdownMenuItem[];

    triggerLabel?: string;
    showTrigger?: boolean;

    sideOffset?: number;
    align?: "start" | "center" | "end";
    side?: "top" | "right" | "bottom" | "left";

    class?: string;
  }

  let {
    toastStore,
    copies = [],
    actions = [],
    triggerLabel = "⋯",
    showTrigger = true,
    sideOffset = 6,
    align = "end",
    side = "bottom",
    class: className
  }: Props = $props();

  const toCopyMenuItems = (items: CopyItem[]): DropdownMenuItem[] =>
    items
      .filter((item) => Boolean(item.text))
      .map((item) => ({
        label: item.label,
        onSelect: () =>
          void copyToClipboard(
            toastStore,
            item.text,
            item.successMessage,
            item.failureMessage
          )
      }));

  const menuItems = $derived.by(() => {
    const copyItems = toCopyMenuItems(copies);
    return [
      ...copyItems,
      ...(copyItems.length && actions.length
        ? ([{ separator: true }] as DropdownMenuItem[])
        : []),
      ...actions
    ];
  });
</script>

<DropdownMenu
  class={className}
  {showTrigger}
  {triggerLabel}
  items={menuItems}
  {sideOffset}
  {align}
  {side}
/>