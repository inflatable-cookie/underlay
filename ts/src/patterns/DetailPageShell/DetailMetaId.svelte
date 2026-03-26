<script lang="ts">
  import Code from "../../components/Code.svelte";
  import DetailMetaItem from "./DetailMetaItem.svelte";

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

<DetailMetaItem {label}>
  <Code copy>{value}</Code>
</DetailMetaItem>
