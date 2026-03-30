<script lang="ts">
  import { goto } from "$app/navigation";
  import { Button } from "@poodle/svelte-primitives";
  import { Card } from "@poodle/svelte-primitives";
</script>

<Card>
  <h1>Sign in</h1>
  <p class="eyebrow">Auth wiring is project-specific.</p>
  <p class="copy">In SPA mode, login is typically handled client-side via an auth store.</p>
  <div class="actions">
    <Button variant="primary" onclick={() => goto("/")}>Continue</Button>
  </div>
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

  .actions {
    margin-top: 1rem;
    display: flex;
    justify-content: flex-end;
  }
</style>
