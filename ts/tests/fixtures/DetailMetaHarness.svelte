<script lang="ts">
  import DetailMeta from "../../src/patterns/DetailPageShell/DetailMeta.svelte";
  import DetailMetaId from "../../src/patterns/DetailPageShell/DetailMetaId.svelte";
  import DetailMetaSeparator from "../../src/patterns/DetailPageShell/DetailMetaSeparator.svelte";
  import DetailMetaStatus from "../../src/patterns/DetailPageShell/DetailMetaStatus.svelte";

  interface Props {
    idValue?: string;
    idLabel?: string;
    statusValue?: boolean;
    trueLabel?: string;
    falseLabel?: string;
    variant?: "success" | "danger";
  }

  let {
    idValue = "abc-123",
    idLabel = "ID",
    statusValue = true,
    trueLabel = "Enabled",
    falseLabel = "Disabled",
    variant = "danger",
  }: Props = $props();
</script>

<DetailMeta>
  <span data-testid="meta-first">First item</span>
  <DetailMetaSeparator />
  <DetailMetaId value={idValue} label={idLabel} />
  <DetailMetaSeparator />
  <DetailMetaStatus value={statusValue} {trueLabel} {falseLabel} {variant} />
</DetailMeta>
