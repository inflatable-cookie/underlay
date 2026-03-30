<script lang="ts">
  import { Card } from "@poodle/svelte-primitives";

  let { children } = $props();
</script>

<div class="admin-auth-layout">
  <div class="admin-auth-layout__card">
    <Card variant="elevated">
      {@render children()}
    </Card>
  </div>
</div>

<style>
  .admin-auth-layout {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: clamp(1.5rem, 4vw, 3rem);
    box-sizing: border-box;
  }

  .admin-auth-layout__card {
    width: 100%;
    max-width: 24rem;
  }
</style>
