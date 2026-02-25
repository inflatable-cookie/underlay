<script lang="ts">
  import TabsRoot from "../../src/components/TabsRoot.svelte";
  import TabsList from "../../src/components/TabsList.svelte";
  import TabsTrigger from "../../src/components/TabsTrigger.svelte";
  import TabsContent from "../../src/components/TabsContent.svelte";

  interface Props {
    initialValue?: string;
    historyKey?: string;
    variant?: "pills" | "boxed" | "underline" | "plain" | "form";
    size?: "default" | "sm";
  }

  let {
    initialValue = "one",
    historyKey = "tab",
    variant = "pills",
    size = "default"
  }: Props = $props();

  let value = $state("");

  $effect(() => {
    if (!value) {
      value = initialValue;
    }
  });
</script>

<TabsRoot bind:value {historyKey} {variant} {size}>
  <TabsList>
    <TabsTrigger value="one">Overview</TabsTrigger>
    <TabsTrigger value="two">Details</TabsTrigger>
  </TabsList>
  <TabsContent value="one">
    <div data-testid="history-content-one">One</div>
  </TabsContent>
  <TabsContent value="two">
    <div data-testid="history-content-two">Two</div>
  </TabsContent>
</TabsRoot>

<div data-testid="history-active-tab">{value}</div>
