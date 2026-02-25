<script lang="ts">
  import Dialog from "../../src/components/Dialog.svelte";

  interface Props {
    initialOpen?: boolean;
    title?: string | null;
    description?: string | null;
    showTrigger?: boolean;
    triggerLabel?: string;
    showCloseX?: boolean;
    contentClassName?: string;
    overlayClassName?: string;
    withTriggerSnippet?: boolean;
    withFooter?: boolean;
    withChildren?: boolean;
  }

  let {
    initialOpen = false,
    title = "Dialog title",
    description = "Dialog description",
    showTrigger = true,
    triggerLabel = "Open dialog",
    showCloseX = true,
    contentClassName = "",
    overlayClassName = "",
    withTriggerSnippet = false,
    withFooter = true,
    withChildren = true
  }: Props = $props();

  let open = $state(false);

  $effect(() => {
    open = initialOpen;
  });
</script>

{#snippet triggerSnippet()}
  <span data-testid="dialog-custom-trigger">Launch</span>
{/snippet}

{#snippet footerSnippet()}
  <button type="button" data-testid="dialog-footer-button">Confirm</button>
{/snippet}

{#snippet childrenSnippet()}
  <p data-testid="dialog-body">Dialog body content</p>
{/snippet}

<p data-testid="dialog-open-state">{open ? "open" : "closed"}</p>

<Dialog
  bind:open
  {title}
  {description}
  {showTrigger}
  {triggerLabel}
  {showCloseX}
  {contentClassName}
  {overlayClassName}
  trigger={withTriggerSnippet ? triggerSnippet : undefined}
  footer={withFooter ? footerSnippet : undefined}
  children={withChildren ? childrenSnippet : undefined}
/>
