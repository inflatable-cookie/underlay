<script lang="ts">
  import Breadcrumbs from "../../src/components/Breadcrumbs.svelte";

  interface Item {
    label: string;
    href?: string;
    icon?: string;
  }

  interface Props {
    items?: Item[];
    separator?: string;
    className?: string;
    collapseOnMobile?: boolean;
    maxItems?: number;
    dataTestId?: string;
  }

  let {
    items = [
      { label: "Home", href: "/" },
      { label: "Library", href: "/library" },
      { label: "Books", href: "/library/books" },
      { label: "Detail" }
    ],
    separator = "›",
    className = "",
    collapseOnMobile = true,
    maxItems = 4,
    dataTestId = "breadcrumbs-root"
  }: Props = $props();
</script>

<Breadcrumbs
  {items}
  {separator}
  className={className}
  {collapseOnMobile}
  {maxItems}
  data-testid={dataTestId}
/>
