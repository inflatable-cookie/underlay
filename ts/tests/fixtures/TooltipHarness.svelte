<script lang="ts">
  import Tooltip from "../../src/components/Tooltip.svelte";

  interface Props {
    initialOpen?: boolean;
    content?: string;
    showTrigger?: boolean;
    triggerLabel?: string;
    inline?: boolean;
    disabled?: boolean;
    className?: string;
    withTriggerSnippet?: boolean;
  }

  let {
    initialOpen = false,
    content = "Tooltip content",
    showTrigger = true,
    triggerLabel = "ⓘ",
    inline = false,
    disabled = false,
    className = "",
    withTriggerSnippet = false
  }: Props = $props();

  let open = $state(false);

  $effect(() => {
    open = initialOpen;
  });
</script>

{#snippet triggerSnippet()}
  <span data-testid="tooltip-custom-trigger">Hover target</span>
{/snippet}

<p data-testid="tooltip-open-state">{open ? "open" : "closed"}</p>

<Tooltip
  bind:open
  {content}
  {showTrigger}
  {triggerLabel}
  {inline}
  {disabled}
  class={className}
  trigger={withTriggerSnippet ? triggerSnippet : undefined}
/>
