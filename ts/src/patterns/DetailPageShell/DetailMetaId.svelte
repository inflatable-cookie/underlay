<script lang="ts">
  import Code from "../../components/Code.svelte";
  import PageHeaderMetaItem from "../PageHeaderMetaItem.svelte";

  interface Props {
    /** The ID value to display */
    value: string;
    /** Label (default: "ID") */
    label?: string;
  }

  let {
    value,
    label = "ID"
  }: Props = $props();
</script>

<PageHeaderMetaItem label={label}>
  <Code copy>{value}</Code>
</PageHeaderMetaItem>
