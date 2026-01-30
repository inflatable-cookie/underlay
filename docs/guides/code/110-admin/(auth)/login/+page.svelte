<script lang="ts">
  import { goto } from "$app/navigation";
  import { Button, Card } from "@decodelabs/underlay/components";
</script>

<Card title="Sign in" subtitle="Auth wiring is project-specific." variant="muted">
  <p class="copy">In SPA mode, login is typically handled client-side via an auth store.</p>
  <div class="actions">
    <Button variant="primary" onclick={() => goto("/")}>Continue</Button>
  </div>
</Card>

<style>
  .copy {
    margin: 0;
    color: var(--admin-color-text-muted);
    line-height: 1.5;
  }

  .actions {
    margin-top: 1rem;
    display: flex;
    justify-content: flex-end;
  }
</style>
