<script lang="ts">
  import Drawer from "../../src/components/Drawer.svelte";

  interface Props {
    initialOpen?: boolean;
    title?: string;
    position?: "right" | "left";
    width?: string;
    overlay?: boolean | "auto";
    className?: string;
    withHeaderActions?: boolean;
    withChildren?: boolean;
    onclose?: () => void;
  }

  let {
    initialOpen = true,
    title = "Details",
    position = "right",
    width = "30rem",
    overlay = "auto",
    className = "",
    withHeaderActions = true,
    withChildren = true,
    onclose = undefined
  }: Props = $props();

  let open = $state(false);

  $effect(() => {
    open = initialOpen;
  });
</script>

{#snippet headerActionsSnippet()}
  <button type="button" data-testid="drawer-action">Action</button>
{/snippet}

{#snippet childrenSnippet()}
  <div data-testid="drawer-content">Drawer content</div>
{/snippet}

<p data-testid="drawer-open-state">{open ? "open" : "closed"}</p>

<Drawer
  bind:open
  {title}
  {position}
  {width}
  {overlay}
  class={className}
  {onclose}
  headerActions={withHeaderActions ? headerActionsSnippet : undefined}
  children={withChildren ? childrenSnippet : undefined}
/>
