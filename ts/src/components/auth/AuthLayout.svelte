<script lang="ts">
  /**
   * Centered authentication layout container.
   *
   * Provides a consistent layout for auth pages with optional
   * branding slots for logo and footer content.
   *
   * @example
   * ```svelte
   * <AuthLayout title="Admin">
   *   {#snippet logo()}<img src="/logo.svg" alt="Logo" />{/snippet}
   *   <LoginPage ... />
   *   {#snippet footer()}<a href="/terms">Terms</a>{/snippet}
   * </AuthLayout>
   * ```
   */

  import type { Snippet } from "svelte";

  interface Props {
    /** Optional title displayed above the content */
    title?: string;
    /** Maximum width of the content card (default: 24rem) */
    maxWidth?: string;
    /** Additional CSS class */
    class?: string;
    /** Logo/branding slot rendered above the title */
    logo?: Snippet;
    /** Main content */
    children: Snippet;
    /** Footer slot rendered below the content */
    footer?: Snippet;
  }

  let {
    title,
    maxWidth = "24rem",
    class: className = "",
    logo,
    children,
    footer,
  }: Props = $props();
</script>

<div class="underlay-auth-layout {className}">
  <div class="underlay-auth-layout__card" style:max-width={maxWidth}>
    {#if logo}
      <div class="underlay-auth-layout__logo">
        {@render logo()}
      </div>
    {/if}

    {#if title}
      <h1 class="underlay-auth-layout__title">{title}</h1>
    {/if}

    <div class="underlay-auth-layout__content">
      {@render children()}
    </div>

    {#if footer}
      <div class="underlay-auth-layout__footer">
        {@render footer()}
      </div>
    {/if}
  </div>
</div>

<style>
  .underlay-auth-layout {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--underlay-space-4, 1rem);
    box-sizing: border-box;
  }

  .underlay-auth-layout__card {
    width: 100%;
  }

  .underlay-auth-layout__logo {
    display: flex;
    justify-content: center;
    margin-bottom: var(--underlay-space-4, 1rem);
  }

  .underlay-auth-layout__title {
    text-align: center;
    font-size: var(--underlay-font-size-xl, 1.5rem);
    font-weight: 600;
    margin: 0 0 var(--underlay-space-4, 1.25rem);
    letter-spacing: 0.04em;
    color: var(--underlay-color-text, #e5e7eb);
  }

  /* .underlay-auth-layout__content: Content renders in the slot, no additional styles needed */

  .underlay-auth-layout__footer {
    margin-top: var(--underlay-space-4, 1rem);
    text-align: center;
    font-size: var(--underlay-font-size-sm, 0.85rem);
    color: var(--underlay-color-text-muted, #64748b);
  }

  .underlay-auth-layout__footer :global(a) {
    color: var(--underlay-color-accent, #3b82f6);
    text-decoration: underline;
  }

  .underlay-auth-layout__footer :global(a:hover) {
    text-decoration-thickness: 2px;
  }
</style>
