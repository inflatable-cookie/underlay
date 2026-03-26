<script lang="ts">
  import AuthLayout from "../../src/components/auth/AuthLayout.svelte";
</script>

<AuthLayout title="Acme Admin">
  {#snippet logo()}
    <div class="auth-layout-demo__logo">A</div>
  {/snippet}

  <div class="auth-layout-demo__content">
    <h2>Retained Auth Shell</h2>
    <p>Use Underlay here for auth-page framing, not for generic card or button primitives.</p>
  </div>

  {#snippet footer()}
    <a href="/">Terms</a> · <a href="/">Privacy</a>
  {/snippet}
</AuthLayout>

<style>
  .auth-layout-demo__logo {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 3rem;
    height: 3rem;
    border-radius: 999px;
    background: linear-gradient(135deg, #14b8a6, #0f172a);
    color: white;
    font-weight: 700;
  }

  .auth-layout-demo__content h2 {
    margin: 0 0 0.5rem;
    font-size: 1.05rem;
  }

  .auth-layout-demo__content p {
    margin: 0;
    color: var(--underlay-color-text-muted, #94a3b8);
  }
</style>
