<script lang="ts">
  import { Card } from "@poodle/svelte-primitives";

  const envName = "local";
</script>

<Card>
  <h1>Dashboard</h1>
  <p class="eyebrow">Environment: {envName}</p>
  <p class="copy">This is a placeholder dashboard inside the default admin shell.</p>
</Card>

<style>
  .eyebrow {
    margin: 0 0 0.35rem;
    color: var(--admin-color-text-muted);
    font-size: 0.85rem;
  }

  .copy {
    margin: 0;
    color: var(--admin-color-text-muted);
    line-height: 1.5;
  }
</style>
