<script lang="ts">
  import DropdownMenu from "../../src/components/DropdownMenu.svelte";

  type MenuItem = {
    label?: string;
    onSelect?: (() => void) | undefined;
    disabled?: boolean;
    destructive?: boolean;
    separator?: boolean;
  };

  interface Props {
    initialOpen?: boolean;
    showTrigger?: boolean;
    triggerLabel?: string;
    triggerAriaLabel?: string;
    items?: MenuItem[] | null | undefined;
    className?: string;
    contentClassName?: string;
    withTriggerSnippet?: boolean;
    withChildren?: boolean;
  }

  let {
    initialOpen = false,
    showTrigger = true,
    triggerLabel = "Menu",
    triggerAriaLabel = "Open menu",
    items = undefined,
    className = "",
    contentClassName = "",
    withTriggerSnippet = false,
    withChildren = false
  }: Props = $props();

  let open = $state(false);
  $effect(() => {
    open = initialOpen;
  });
</script>

{#snippet triggerSnippet()}
  <span data-testid="dropdown-custom-trigger">Open</span>
{/snippet}

{#snippet childrenSnippet()}
  <button type="button" data-testid="dropdown-custom-child">Child action</button>
{/snippet}

<p data-testid="dropdown-open-state">{open ? "open" : "closed"}</p>

<DropdownMenu
  bind:open
  {showTrigger}
  {triggerLabel}
  {triggerAriaLabel}
  {items}
  class={className}
  {contentClassName}
  trigger={withTriggerSnippet ? triggerSnippet : undefined}
  children={withChildren ? childrenSnippet : undefined}
/>
