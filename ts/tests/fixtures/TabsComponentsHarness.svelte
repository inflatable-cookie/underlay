<script lang="ts">
  import { setContext } from "svelte";
  import TabsRoot from "../../src/components/TabsRoot.svelte";
  import TabsList from "../../src/components/TabsList.svelte";
  import TabsTrigger from "../../src/components/TabsTrigger.svelte";
  import TabsSeparator from "../../src/components/TabsSeparator.svelte";
  import TabsContent from "../../src/components/TabsContent.svelte";

  type SectionValidationState = "valid" | "invalid" | "incomplete" | "idle";

  interface Props {
    variant?: "pills" | "boxed" | "underline" | "plain" | "form";
    size?: "default" | "sm";
    initialValue?: string;
    countOne?: number | null;
    countTwo?: number | null;
    stateOne?: SectionValidationState;
    stateTwo?: SectionValidationState;
    withSeparator?: boolean;
    collapsible?: boolean;
  }

  let {
    variant = "boxed",
    size = "sm",
    initialValue = "one",
    countOne = 2,
    countTwo = null,
    stateOne = "idle",
    stateTwo = "idle",
    withSeparator = true,
    collapsible = false
  }: Props = $props();

  let value = $state("");

  $effect(() => {
    value = initialValue;
  });

  setContext("underlay-form-tabs-registry", {
    registerSection: () => undefined,
    unregisterSection: () => undefined,
    registerFieldInSection: () => undefined,
    unregisterFieldFromSection: () => undefined,
    getSectionState: (sectionId: string): SectionValidationState =>
      sectionId === "one" ? stateOne : stateTwo
  });
</script>

<TabsRoot bind:value {variant} {size}>
  <TabsList {collapsible}>
    <TabsTrigger value="one" count={countOne}>Overview</TabsTrigger>
    {#if withSeparator}
      <TabsSeparator />
    {/if}
    <TabsTrigger value="two" count={countTwo}>Details</TabsTrigger>
  </TabsList>
  <TabsContent value="one">
    <div data-testid="content-one">One content</div>
  </TabsContent>
  <TabsContent value="two">
    <div data-testid="content-two">Two content</div>
  </TabsContent>
</TabsRoot>

<div data-testid="active-tab">{value}</div>
