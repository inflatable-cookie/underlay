<script lang="ts">
  import Code from "../../src/components/Code.svelte";

  interface Props {
    text?: string;
    copy?: boolean;
    className?: string;
    dataTestId?: string;
  }

  let {
    text = "example-code",
    copy = false,
    className = "",
    dataTestId = "code-node"
  }: Props = $props();
</script>

<Code copy={copy} class={className} data-testid={dataTestId}>
  {#snippet children()}
    {text}
  {/snippet}
</Code>
