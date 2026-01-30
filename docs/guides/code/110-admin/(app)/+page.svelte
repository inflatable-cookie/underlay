<script lang="ts">
  import { Card } from "@decodelabs/underlay/components";

  const envName = "local";
</script>

<Card title="Dashboard" subtitle={`Environment: ${envName}`} variant="muted">
  <p class="copy">This is a placeholder dashboard inside the default admin shell.</p>
</Card>

<style>
  .copy {
    margin: 0;
    color: var(--admin-color-text-muted);
    line-height: 1.5;
  }
</style>
