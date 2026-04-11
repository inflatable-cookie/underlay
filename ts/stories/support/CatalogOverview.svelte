<script lang="ts">
  import { Card } from "@poodle/svelte";
</script>

<div class="catalog-overview">
  <Card class="catalog-overview__card">
    <h1>Underlay Shared UI Catalog</h1>
    <p>
      This catalog documents the retained Underlay surface after the Poodle migration wave.
      Use it for workflow shells, structural composites, and helper surfaces that still belong
      to Underlay directly.
    </p>
  </Card>

  <div class="catalog-overview__grid">
    <Card class="catalog-overview__card">
      <h2>Use Underlay Here</h2>
      <ul>
        <li>Auth shells and auth-specific helpers</li>
        <li>Media workflows and batch/list orchestration</li>
        <li>Structural shells such as page and form flows</li>
        <li>Retained workflow helpers like copy-action menus</li>
      </ul>
    </Card>

    <Card class="catalog-overview__card">
      <h2>Use Poodle Instead</h2>
      <ul>
        <li>Primitives such as buttons, fields, inputs, dialogs, and tabs</li>
        <li>Generic composites such as list containers and file upload</li>
        <li>Reusable visual chrome that no longer needs Underlay ownership</li>
      </ul>
    </Card>
  </div>
</div>

<style>
  .catalog-overview {
    display: grid;
    gap: 1rem;
    width: min(72rem, calc(100vw - 2rem));
  }

  .catalog-overview__grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(18rem, 1fr));
    gap: 1rem;
  }
</style>
