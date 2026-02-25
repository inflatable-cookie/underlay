<script lang="ts">
  import PageHeaderMeta from "../../src/patterns/PageHeaderMeta.svelte";
  import PageHeaderMetaRow from "../../src/patterns/PageHeaderMetaRow.svelte";
  import PageHeaderMetaItem from "../../src/patterns/PageHeaderMetaItem.svelte";
  import PageHeaderMetaSeparator from "../../src/patterns/PageHeaderMetaSeparator.svelte";

  interface Props {
    label?: string;
    value?: string;
  }

  let { label = "Status", value = "Active" }: Props = $props();
</script>

<PageHeaderMeta>
  <PageHeaderMetaRow>
    <PageHeaderMetaItem {label}>
      <span data-testid="meta-item-value">{value}</span>
    </PageHeaderMetaItem>
    <PageHeaderMetaSeparator />
    <PageHeaderMetaItem>
      <span data-testid="meta-item-unlabelled">Unlabelled</span>
    </PageHeaderMetaItem>
  </PageHeaderMetaRow>
</PageHeaderMeta>
