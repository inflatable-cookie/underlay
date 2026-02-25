<script lang="ts">
  import Popover from "../../src/components/Popover.svelte";

  interface Props {
    initialOpen?: boolean;
    showTrigger?: boolean;
    triggerLabel?: string;
    triggerAriaLabel?: string;
    triggerType?: "button" | "submit" | "reset";
    contentClassName?: string;
    className?: string;
    withTriggerSnippet?: boolean;
    withChildren?: boolean;
  }

  let {
    initialOpen = false,
    showTrigger = true,
    triggerLabel = "Open popover",
    triggerAriaLabel = "Popover trigger",
    triggerType = "button",
    contentClassName = "",
    className = "",
    withTriggerSnippet = false,
    withChildren = true
  }: Props = $props();

  let open = $state(false);

  $effect(() => {
    open = initialOpen;
  });
</script>

{#snippet triggerSnippet()}
  <span data-testid="popover-custom-trigger">Trigger</span>
{/snippet}

{#snippet childrenSnippet()}
  <div data-testid="popover-content">Popover content</div>
{/snippet}

<p data-testid="popover-open-state">{open ? "open" : "closed"}</p>

<Popover
  bind:open
  {showTrigger}
  {triggerLabel}
  {triggerAriaLabel}
  {triggerType}
  {contentClassName}
  class={className}
  trigger={withTriggerSnippet ? triggerSnippet : undefined}
  children={withChildren ? childrenSnippet : undefined}
/>
