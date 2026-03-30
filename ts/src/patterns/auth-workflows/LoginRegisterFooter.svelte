<script lang="ts">
  interface Props {
    registerHref: string;
  }

  let { registerHref }: Props = $props();
</script>

<div class="underlay-login-page__register">
  <span>Don't have an account?</span>
  <a href={registerHref}>Sign up</a>
</div>

<style>
  .underlay-login-page__register {
    margin-top: var(--underlay-space-4, 1rem);
    padding-top: var(--underlay-space-3, 0.75rem);
    border-top: 1px solid var(--underlay-color-border, rgba(255, 255, 255, 0.1));
    text-align: center;
    font-size: var(--underlay-font-size-sm, 0.85rem);
    color: var(--underlay-color-text-muted, #64748b);
  }

  .underlay-login-page__register a {
    color: var(--underlay-color-accent, #3b82f6);
    text-decoration: underline;
    margin-left: var(--underlay-space-1, 0.25rem);
  }
</style>
