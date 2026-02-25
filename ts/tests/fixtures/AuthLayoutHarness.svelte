<script lang="ts">
  import AuthLayout from "../../src/components/auth/AuthLayout.svelte";

  interface Props {
    title?: string;
    maxWidth?: string;
    className?: string;
    withLogo?: boolean;
    withFooter?: boolean;
  }

  let {
    title = "Welcome",
    maxWidth = "30rem",
    className = "auth-layout-harness",
    withLogo = true,
    withFooter = true,
  }: Props = $props();
</script>

{#snippet logoSnippet()}
  <span data-testid="logo">Underlay</span>
{/snippet}

{#snippet footerSnippet()}
  <a href="/terms" data-testid="footer-link">Terms</a>
{/snippet}

<AuthLayout
  title={title}
  {maxWidth}
  class={className}
  logo={withLogo ? logoSnippet : undefined}
  footer={withFooter ? footerSnippet : undefined}
>
  <div data-testid="content">Auth content</div>
</AuthLayout>
