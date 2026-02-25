<script lang="ts">
  import type { Component, SvelteComponent } from "svelte";
  import EmptyState from "../../src/components/EmptyState.svelte";
  import TestIcon from "./TestIcon.svelte";

  type IconComponent =
    | Component<{ size?: number }>
    | (new (...args: any[]) => SvelteComponent);

  interface Props {
    title?: string;
    description?: string;
    actionLabel?: string;
    actionHref?: string;
    variant?: "default" | "compact";
    className?: string;
    withIcon?: boolean;
    withChildren?: boolean;
    onaction?: () => void;
  }

  let {
    title = "Nothing here yet",
    description = "Create your first record to get started.",
    actionLabel = undefined,
    actionHref = undefined,
    variant = "default",
    className = "",
    withIcon = false,
    withChildren = false,
    onaction = undefined
  }: Props = $props();

  let iconComponent = $derived<IconComponent | undefined>(withIcon ? TestIcon : undefined);
</script>

{#snippet childrenSnippet()}
  <div data-testid="empty-state-custom">Custom empty content</div>
{/snippet}

<EmptyState
  {title}
  {description}
  {actionLabel}
  {actionHref}
  {variant}
  class={className}
  icon={iconComponent}
  {onaction}
  children={withChildren ? childrenSnippet : undefined}
/>
